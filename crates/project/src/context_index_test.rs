#[cfg(test)]
mod context_index_tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use language::LanguageRegistry;
    use std::{path::Path, sync::Arc};
    use serde_json::json;

    #[gpui::test]
    async fn test_context_index_tracks_file_metadata(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        
        // Create a simple Rust file
        fs.insert_file("/project/src/main.rs", r#"
fn main() {
    println!("Hello, world!");
}

struct MyStruct {
    field: i32,
}
        "#).await;

        fs.insert_file("/project/src/lib.rs", r#"
use std::collections::HashMap;

pub fn function_one() -> i32 {
    42
}

pub struct AnotherStruct {
    data: String,
}
        "#).await;

        // Create project
        let project = Project::test(Arc::new(fs), [Path::new("/project")], cx).await;
        
        cx.executor().run_until_parked();

        // Open the main.rs buffer
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((WorktreeId::default(), "src/main.rs"), cx)
            })
            .await
            .unwrap();

        // Update context metadata  
        project.update(cx, |project, cx| {
            project.update_buffer_context(&buffer, cx);
        });

        // Test that we can get context metadata
        let metadata = project.read_with(cx, |project, _| {
            project.get_buffer_context_metadata(buffer.read(cx).remote_id())
        }).unwrap();

        assert!(metadata.is_some());
        let metadata = metadata.unwrap();
        assert!(metadata.symbol_count > 0); // Should find functions/structs
        assert_eq!(metadata.import_count, 0); // No imports in main.rs

        // Test finding related files
        let main_path = std::path::PathBuf::from("/project/src/main.rs");
        let related_files = project.read_with(cx, |project, _| {
            project.find_related_files(&main_path)
        });

        // Should find lib.rs in the same directory
        assert!(related_files.iter().any(|path| path.ends_with("lib.rs")));
    }

    #[gpui::test]
    async fn test_context_index_counts_symbols_and_imports(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        
        // Create a file with imports and symbols
        fs.insert_file("/project/test.rs", r#"
use std::collections::HashMap;
use serde::Serialize;
use anyhow::Result;

fn function_one() -> i32 {
    42
}

fn function_two() -> String {
    "hello".to_string()
}

struct StructOne {
    field1: i32,
    field2: String,
}

enum EnumOne {
    Variant1,
    Variant2(i32),
}
        "#).await;

        let project = Project::test(Arc::new(fs), [Path::new("/project")], cx).await;
        
        cx.executor().run_until_parked();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((WorktreeId::default(), "test.rs"), cx)
            })
            .await
            .unwrap();

        project.update(cx, |project, cx| {
            project.update_buffer_context(&buffer, cx);
        });

        let metadata = project.read_with(cx, |project, _| {
            project.get_buffer_context_metadata(buffer.read(cx).remote_id())
        }).unwrap().unwrap();

        // Should count functions, structs, and enums
        assert!(metadata.symbol_count >= 4); // 2 functions + 1 struct + 1 enum
        assert_eq!(metadata.import_count, 3); // 3 use statements
    }

    #[gpui::test]
    async fn test_context_aware_search_prioritizes_related_files(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        
        // Create related files in the same directory
        fs.insert_file("/project/src/main.rs", r#"
fn main() {
    let helper = helper_function();
    println!("Result: {}", helper);
}
        "#).await;

        fs.insert_file("/project/src/helper.rs", r#"
pub fn helper_function() -> i32 {
    42
}
        "#).await;

        fs.insert_file("/project/tests/integration_test.rs", r#"
use project::helper_function;

#[test]
fn test_helper_function() {
    assert_eq!(helper_function(), 42);
}
        "#).await;

        let project = Project::test(Arc::new(fs), [Path::new("/project")], cx).await;
        
        cx.executor().run_until_parked();

        // Open buffers
        let main_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((WorktreeId::default(), "src/main.rs"), cx)
            })
            .await
            .unwrap();

        let helper_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((WorktreeId::default(), "src/helper.rs"), cx)
            })
            .await
            .unwrap();

        let test_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((WorktreeId::default(), "tests/integration_test.rs"), cx)
            })
            .await
            .unwrap();

        // Update context for all buffers
        project.update(cx, |project, cx| {
            project.update_buffer_context(&main_buffer, cx);
            project.update_buffer_context(&helper_buffer, cx);
            project.update_buffer_context(&test_buffer, cx);
        });

        // Test context-aware search from main.rs perspective
        let main_path = std::path::PathBuf::from("/project/src/main.rs");
        let related_files = project.read_with(cx, |project, _| {
            project.find_related_files(&main_path)
        });

        // Should prioritize helper.rs as it's in the same directory
        assert!(related_files.iter().any(|path| path.ends_with("helper.rs")));
        
        // The integration test should be less prioritized (different directory)
        let helper_index = related_files.iter().position(|path| path.ends_with("helper.rs"));
        let test_index = related_files.iter().position(|path| path.ends_with("integration_test.rs"));
        
        if let (Some(helper_idx), Some(test_idx)) = (helper_index, test_index) {
            assert!(helper_idx < test_idx, "helper.rs should come before integration_test.rs");
        }
    }
}