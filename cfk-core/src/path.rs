//! Virtual path abstraction

use serde::{Deserialize, Serialize};
use std::fmt;

/// Virtual path representing a location across any backend
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualPath {
    /// Backend identifier (e.g., "local", "dropbox", "gdrive")
    pub backend: String,
    /// Path segments
    pub segments: Vec<String>,
}

impl VirtualPath {
    pub fn new(backend: impl Into<String>, path: impl AsRef<str>) -> Self {
        let path = path.as_ref();
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        Self {
            backend: backend.into(),
            segments,
        }
    }

    pub fn root(backend: impl Into<String>) -> Self {
        Self {
            backend: backend.into(),
            segments: Vec::new(),
        }
    }

    pub fn join(&self, name: impl AsRef<str>) -> Self {
        let mut segments = self.segments.clone();
        for part in name.as_ref().split('/').filter(|s| !s.is_empty()) {
            if part == ".." {
                segments.pop();
            } else if part != "." {
                segments.push(part.to_string());
            }
        }
        Self {
            backend: self.backend.clone(),
            segments,
        }
    }

    pub fn parent(&self) -> Option<Self> {
        if self.segments.is_empty() {
            None
        } else {
            let mut segments = self.segments.clone();
            segments.pop();
            Some(Self {
                backend: self.backend.clone(),
                segments,
            })
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    pub fn extension(&self) -> Option<&str> {
        self.name().and_then(|n| n.rsplit_once('.')).map(|(_, ext)| ext)
    }

    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn to_path_string(&self) -> String {
        if self.segments.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.segments.join("/"))
        }
    }

    pub fn to_uri(&self) -> String {
        format!("cfk://{}{}", self.backend, self.to_path_string())
    }

    pub fn parse_uri(uri: &str) -> Option<Self> {
        let uri = uri.strip_prefix("cfk://")?;
        let (backend, path) = uri.split_once('/').unwrap_or((uri, ""));
        Some(Self::new(backend, path))
    }
}

impl fmt::Display for VirtualPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uri())
    }
}
