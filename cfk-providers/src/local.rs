//! Local filesystem backend

use async_trait::async_trait;
use bytes::Bytes;
use cfk_core::{
    backend::{ByteStream, SpaceInfo, StorageBackend, StorageCapabilities},
    entry::{DirectoryListing, Entry, EntryKind},
    error::{CfkError, CfkResult},
    metadata::{Metadata, Permissions},
    operations::*,
    VirtualPath,
};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncReadExt;

/// Local filesystem backend
pub struct LocalBackend {
    id: String,
    root: PathBuf,
    capabilities: StorageCapabilities,
}

impl LocalBackend {
    pub fn new(id: impl Into<String>, root: impl AsRef<Path>) -> Self {
        Self {
            id: id.into(),
            root: root.as_ref().to_path_buf(),
            capabilities: StorageCapabilities::local_filesystem(),
        }
    }

    fn to_real_path(&self, path: &VirtualPath) -> PathBuf {
        let mut real = self.root.clone();
        for seg in &path.segments {
            real.push(seg);
        }
        real
    }

    fn to_virtual_path(&self, real: &Path) -> CfkResult<VirtualPath> {
        let relative = real
            .strip_prefix(&self.root)
            .map_err(|_| CfkError::InvalidPath(real.display().to_string()))?;
        Ok(VirtualPath::new(&self.id, relative.to_string_lossy()))
    }

    async fn metadata_from_path(&self, path: &Path) -> CfkResult<(EntryKind, Metadata)> {
        let meta = fs::metadata(path).await?;
        let kind = if meta.is_dir() {
            EntryKind::Directory
        } else if meta.is_file() {
            EntryKind::File
        } else if meta.is_symlink() {
            EntryKind::Symlink
        } else {
            EntryKind::Unknown
        };

        let mut metadata = Metadata::new();
        metadata.size = Some(meta.len());

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            metadata.permissions = Some(Permissions::new(meta.mode()));
        }

        if let Ok(modified) = meta.modified() {
            metadata.modified = Some(modified.into());
        }
        if let Ok(created) = meta.created() {
            metadata.created = Some(created.into());
        }

        Ok((kind, metadata))
    }
}

#[async_trait]
impl StorageBackend for LocalBackend {
    fn id(&self) -> &str {
        &self.id
    }

    fn display_name(&self) -> &str {
        "Local Filesystem"
    }

    fn capabilities(&self) -> &StorageCapabilities {
        &self.capabilities
    }

    async fn is_available(&self) -> bool {
        self.root.exists()
    }

    async fn get_metadata(&self, path: &VirtualPath) -> CfkResult<Entry> {
        let real = self.to_real_path(path);
        if !real.exists() {
            return Err(CfkError::NotFound(path.to_string()));
        }
        let (kind, metadata) = self.metadata_from_path(&real).await?;
        Ok(Entry { path: path.clone(), kind, metadata })
    }

    async fn list_directory(&self, path: &VirtualPath, _options: &ListOptions) -> CfkResult<DirectoryListing> {
        let real = self.to_real_path(path);
        if !real.is_dir() {
            return Err(CfkError::NotADirectory(path.to_string()));
        }

        let mut entries = Vec::new();
        let mut read_dir = fs::read_dir(&real).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let entry_path = entry.path();
            let vpath = self.to_virtual_path(&entry_path)?;
            let (kind, metadata) = self.metadata_from_path(&entry_path).await?;
            entries.push(Entry { path: vpath, kind, metadata });
        }

        Ok(DirectoryListing::new(path.clone(), entries))
    }

    async fn read_file(&self, path: &VirtualPath, options: &ReadOptions) -> CfkResult<ByteStream> {
        let real = self.to_real_path(path);
        if !real.is_file() {
            return Err(CfkError::NotAFile(path.to_string()));
        }

        let mut file = fs::File::open(&real).await?;
        let mut buffer = Vec::new();

        if let Some((start, end)) = options.range {
            use tokio::io::AsyncSeekExt;
            file.seek(std::io::SeekFrom::Start(start)).await?;
            let len = (end - start) as usize;
            buffer.resize(len, 0);
            file.read_exact(&mut buffer).await?;
        } else {
            file.read_to_end(&mut buffer).await?;
        }

        let bytes = Bytes::from(buffer);
        Ok(Box::pin(futures::stream::once(async { Ok(bytes) })))
    }

    async fn write_file(&self, path: &VirtualPath, data: Bytes, options: &WriteOptions) -> CfkResult<Entry> {
        let real = self.to_real_path(path);

        if real.exists() && !options.overwrite {
            return Err(CfkError::AlreadyExists(path.to_string()));
        }

        if options.create_parents {
            if let Some(parent) = real.parent() {
                fs::create_dir_all(parent).await?;
            }
        }

        fs::write(&real, &data).await?;
        self.get_metadata(path).await
    }

    async fn write_file_stream(&self, path: &VirtualPath, mut stream: ByteStream, _size_hint: Option<u64>, options: &WriteOptions) -> CfkResult<Entry> {
        use futures::StreamExt;

        let mut data = Vec::new();
        while let Some(chunk) = stream.next().await {
            data.extend_from_slice(&chunk?);
        }
        self.write_file(path, Bytes::from(data), options).await
    }

    async fn create_directory(&self, path: &VirtualPath) -> CfkResult<Entry> {
        let real = self.to_real_path(path);
        fs::create_dir_all(&real).await?;
        self.get_metadata(path).await
    }

    async fn delete(&self, path: &VirtualPath, options: &DeleteOptions) -> CfkResult<()> {
        let real = self.to_real_path(path);

        if !real.exists() {
            if options.force {
                return Ok(());
            }
            return Err(CfkError::NotFound(path.to_string()));
        }

        if real.is_dir() {
            if options.recursive {
                fs::remove_dir_all(&real).await?;
            } else {
                fs::remove_dir(&real).await?;
            }
        } else {
            fs::remove_file(&real).await?;
        }
        Ok(())
    }

    async fn copy(&self, source: &VirtualPath, dest: &VirtualPath, options: &CopyOptions) -> CfkResult<Entry> {
        let src_real = self.to_real_path(source);
        let dst_real = self.to_real_path(dest);

        if !src_real.exists() {
            return Err(CfkError::NotFound(source.to_string()));
        }
        if dst_real.exists() && !options.overwrite {
            return Err(CfkError::AlreadyExists(dest.to_string()));
        }

        fs::copy(&src_real, &dst_real).await?;
        self.get_metadata(dest).await
    }

    async fn rename(&self, source: &VirtualPath, dest: &VirtualPath, options: &MoveOptions) -> CfkResult<Entry> {
        let src_real = self.to_real_path(source);
        let dst_real = self.to_real_path(dest);

        if !src_real.exists() {
            return Err(CfkError::NotFound(source.to_string()));
        }
        if dst_real.exists() && !options.overwrite {
            return Err(CfkError::AlreadyExists(dest.to_string()));
        }

        fs::rename(&src_real, &dst_real).await?;
        self.get_metadata(dest).await
    }

    async fn get_space_info(&self) -> CfkResult<SpaceInfo> {
        // Platform-specific disk space detection would go here
        Ok(SpaceInfo::unknown())
    }
}
