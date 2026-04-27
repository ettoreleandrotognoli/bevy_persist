//! Storage abstraction for persistence.
//!
//! This module provides a trait-based abstraction over different storage backends,
//! allowing the same persistence logic to work on desktop (filesystem) and web
//! (localStorage/IndexedDB) platforms.

use crate::PersistResult;

/// Trait for abstract storage operations.
///
/// Implement this trait to add support for different storage backends
/// (filesystem, IndexedDB, localStorage, etc.)
pub trait Storage: Send + Sync {
    /// Read content from storage
    fn read(&self, path: &str) -> PersistResult<Option<String>>;

    /// Write content to storage
    fn write(&self, path: &str, content: &str) -> PersistResult<()>;

    /// Check if a path exists
    fn exists(&self, path: &str) -> bool;

    /// Delete content from storage
    fn delete(&self, path: &str) -> PersistResult<()>;

    /// Create parent directories if needed
    fn create_dir(&self, path: &str) -> PersistResult<()>;
}

#[cfg(not(feature = "wasm"))]
mod filesystem;

#[cfg(not(feature = "wasm"))]
pub use filesystem::FileSystemStorage;

#[cfg(not(feature = "wasm"))]
pub fn create_storage() -> FileSystemStorage {
    FileSystemStorage::new()
}

#[cfg(feature = "wasm")]
mod wasm_storage;

#[cfg(feature = "wasm")]
pub use wasm_storage::WasmStorage;

#[cfg(feature = "wasm")]
pub fn create_storage() -> WasmStorage {
    WasmStorage::new()
}
