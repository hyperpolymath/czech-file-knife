//! File system entries

use crate::{Metadata, VirtualPath};
use serde::{Deserialize, Serialize};

/// Entry kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Unknown,
}

/// A file system entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub path: VirtualPath,
    pub kind: EntryKind,
    pub metadata: Metadata,
}

impl Entry {
    pub fn file(path: VirtualPath, metadata: Metadata) -> Self {
        Self { path, kind: EntryKind::File, metadata }
    }

    pub fn directory(path: VirtualPath, metadata: Metadata) -> Self {
        Self { path, kind: EntryKind::Directory, metadata }
    }

    pub fn is_file(&self) -> bool {
        self.kind == EntryKind::File
    }

    pub fn is_directory(&self) -> bool {
        self.kind == EntryKind::Directory
    }

    pub fn name(&self) -> Option<&str> {
        self.path.name()
    }

    pub fn size(&self) -> Option<u64> {
        self.metadata.size
    }
}

/// Directory listing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryListing {
    pub path: VirtualPath,
    pub entries: Vec<Entry>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

impl DirectoryListing {
    pub fn new(path: VirtualPath, entries: Vec<Entry>) -> Self {
        Self { path, entries, cursor: None, has_more: false }
    }
}
