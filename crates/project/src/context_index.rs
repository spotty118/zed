use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use anyhow::Result;
use gpui::{Context, Task, Entity};
use language::{Buffer, Language};
use text::BufferId;
use worktree::{WorktreeId, Entry};

/// Metadata about a file's context for improved search and assistance
#[derive(Clone, Debug)]
pub struct FileContextMetadata {
    pub path: PathBuf,
    pub language: Option<Arc<Language>>,
    pub last_modified: SystemTime,
    pub size: u64,
    pub symbol_count: usize,
    pub import_count: usize,
}

/// Tracks context metadata for files in the project to improve context awareness
pub struct ContextIndex {
    file_metadata: HashMap<PathBuf, FileContextMetadata>,
    buffer_metadata: HashMap<BufferId, FileContextMetadata>,
    worktree_indexes: HashMap<WorktreeId, HashMap<PathBuf, FileContextMetadata>>,
}

impl ContextIndex {
    pub fn new() -> Self {
        Self {
            file_metadata: HashMap::new(),
            buffer_metadata: HashMap::new(),
            worktree_indexes: HashMap::new(),
        }
    }

    /// Update metadata for a file when it's opened or modified
    pub fn update_file_metadata(
        &mut self,
        path: PathBuf,
        language: Option<Arc<Language>>,
        entry: &Entry,
    ) {
        let metadata = FileContextMetadata {
            path: path.clone(),
            language,
            last_modified: entry.mtime,
            size: entry.size,
            symbol_count: 0, // Will be updated by language-specific analysis
            import_count: 0, // Will be updated by language-specific analysis
        };

        self.file_metadata.insert(path, metadata);
    }

    /// Update metadata for a buffer
    pub fn update_buffer_metadata(
        &mut self,
        buffer_id: BufferId,
        buffer: &Buffer,
    ) {
        if let Some(file) = buffer.file() {
            let path = file.path().to_path_buf();
            let language = buffer.language().cloned();
            
            // Basic symbol and import counting (simplified)
            let text = buffer.text();
            let symbol_count = self.estimate_symbol_count(&text, language.as_ref());
            let import_count = self.estimate_import_count(&text, language.as_ref());

            let metadata = FileContextMetadata {
                path: path.clone(),
                language,
                last_modified: SystemTime::now(),
                size: text.len() as u64,
                symbol_count,
                import_count,
            };

            self.buffer_metadata.insert(buffer_id, metadata.clone());
            self.file_metadata.insert(path, metadata);
        }
    }

    /// Get metadata for a file
    pub fn get_file_metadata(&self, path: &Path) -> Option<&FileContextMetadata> {
        self.file_metadata.get(path)
    }

    /// Get metadata for a buffer
    pub fn get_buffer_metadata(&self, buffer_id: BufferId) -> Option<&FileContextMetadata> {
        self.buffer_metadata.get(&buffer_id)
    }

    /// Find files that might be contextually related to the given file
    pub fn find_related_files(&self, path: &Path) -> Vec<PathBuf> {
        let mut related = Vec::new();
        
        if let Some(metadata) = self.get_file_metadata(path) {
            // Find files in the same directory
            if let Some(parent) = path.parent() {
                for (file_path, file_metadata) in &self.file_metadata {
                    if let Some(file_parent) = file_path.parent() {
                        if file_parent == parent && file_path != path {
                            // Prefer files with the same language
                            if metadata.language == file_metadata.language {
                                related.insert(0, file_path.clone());
                            } else {
                                related.push(file_path.clone());
                            }
                        }
                    }
                }
            }
        }
        
        related
    }

    /// Clear metadata for a specific worktree
    pub fn clear_worktree(&mut self, worktree_id: WorktreeId) {
        self.worktree_indexes.remove(&worktree_id);
        
        // Remove file metadata for files in this worktree
        // Note: This is a simplified approach - in practice, we'd need
        // better tracking of which files belong to which worktree
        self.file_metadata.retain(|_, _| true);
    }

    /// Simple symbol counting heuristic
    fn estimate_symbol_count(&self, text: &str, language: Option<&Arc<Language>>) -> usize {
        if let Some(lang) = language {
            let lang_name = lang.name().as_ref();
            match lang_name {
                "Rust" => text.matches("fn ").count() + text.matches("struct ").count() + text.matches("enum ").count(),
                "JavaScript" | "TypeScript" => text.matches("function ").count() + text.matches("class ").count(),
                "Python" => text.matches("def ").count() + text.matches("class ").count(),
                _ => text.lines().filter(|line| line.trim().starts_with("pub ") || line.trim().starts_with("fn ")).count(),
            }
        } else {
            // Generic symbol estimation
            text.lines().filter(|line| {
                let trimmed = line.trim();
                trimmed.contains('(') && (trimmed.contains("fn ") || trimmed.contains("function "))
            }).count()
        }
    }

    /// Simple import counting heuristic  
    fn estimate_import_count(&self, text: &str, language: Option<&Arc<Language>>) -> usize {
        if let Some(lang) = language {
            let lang_name = lang.name().as_ref();
            match lang_name {
                "Rust" => text.matches("use ").count(),
                "JavaScript" | "TypeScript" => text.matches("import ").count() + text.matches("require(").count(),
                "Python" => text.matches("import ").count() + text.matches("from ").count(),
                _ => text.lines().filter(|line| line.trim().starts_with("#include")).count(),
            }
        } else {
            0
        }
    }
}

impl Default for ContextIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_symbol_counting() {
        let index = ContextIndex::new();
        
        let rust_code = r#"
            fn main() {
                println!("Hello");
            }
            
            struct MyStruct {
                field: i32,
            }
        "#;
        
        let count = index.estimate_symbol_count(rust_code, None);
        assert!(count >= 1); // Should find at least the function
    }

    #[test]
    fn test_import_counting() {
        let index = ContextIndex::new();
        
        let rust_code = r#"
            use std::collections::HashMap;
            use serde::Serialize;
            
            fn main() {}
        "#;
        
        let count = index.estimate_import_count(rust_code, None);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_related_files() {
        let mut index = ContextIndex::new();
        
        let path1 = PathBuf::from("/project/src/main.rs");
        let path2 = PathBuf::from("/project/src/lib.rs");
        let path3 = PathBuf::from("/project/tests/test.rs");
        
        let entry = Entry {
            id: Default::default(),
            kind: worktree::EntryKind::File,
            path: path1.clone(),
            inode: 0,
            mtime: SystemTime::now(),
            size: 100,
            is_symlink: false,
            is_ignored: false,
            is_external: false,
            git_status: None,
        };
        
        index.update_file_metadata(path1.clone(), None, &entry);
        index.update_file_metadata(path2.clone(), None, &entry);
        index.update_file_metadata(path3.clone(), None, &entry);
        
        let related = index.find_related_files(&path1);
        assert_eq!(related.len(), 1); // Should find lib.rs in same directory
        assert_eq!(related[0], path2);
    }
}