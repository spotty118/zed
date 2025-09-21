use crate::edit_agent::{EditAgent, EditFormat};
use action_log::ActionLog;
use anyhow::Result;
use fs::FakeFs;
use gpui::{AppContext, TestAppContext};
use language::{Buffer, LanguageRegistry};
use language_model::fake_provider::FakeLanguageModel;
use project::Project;
use std::sync::Arc;
use crate::Templates;

#[gpui::test]
async fn test_agentic_editing_consciousness(cx: &mut TestAppContext) -> Result<()> {
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), None, cx).await;
    let action_log = cx.new(|_| ActionLog::new(project.clone()));
    let model = Arc::new(FakeLanguageModel::default());
    let templates = Arc::new(Templates::new());
    
    // Create an agent with consciousness enabled
    let edit_agent = EditAgent::new(
        model.clone(),
        project.clone(),
        action_log.clone(),
        templates,
        EditFormat::XmlTags,
    )
    .with_consciousness(true)
    .with_deliberation(true);

    // Create a test buffer
    let buffer = cx.new(|cx| {
        Buffer::from_text(
            "fn main() {\n    println!(\"Hello, world!\");\n}",
            Some(LanguageRegistry::test(cx.background_executor())),
            cx,
        )
    });
    
    // Test that consciousness and deliberation flags are set correctly
    assert!(edit_agent.consciousness_enabled);
    assert!(edit_agent.deliberation_enabled);
    
    // Test that context analysis works without errors  
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let path = Some(std::path::PathBuf::from("test.rs"));
    let context = edit_agent.analyze_file_context(&snapshot, &path);
    assert!(!context.is_empty());
    assert!(context.contains("Rust source file"));
    
    Ok(())
}

#[gpui::test]
async fn test_agentic_editing_disabled(cx: &mut TestAppContext) -> Result<()> {
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), None, cx).await;
    let action_log = cx.new(|_| ActionLog::new(project.clone()));
    let model = Arc::new(FakeLanguageModel::default());
    let templates = Arc::new(Templates::new());
    
    // Create an agent with consciousness disabled
    let edit_agent = EditAgent::new(
        model.clone(),
        project.clone(),
        action_log.clone(),
        templates,
        EditFormat::XmlTags,
    )
    .with_consciousness(false)
    .with_deliberation(false);

    // Test that consciousness and deliberation flags are set correctly
    assert!(!edit_agent.consciousness_enabled);
    assert!(!edit_agent.deliberation_enabled);
    
    // Test that context analysis works even when consciousness is disabled
    let buffer = cx.new(|cx| {
        Buffer::from_text(
            "console.log('Hello, world!');",
            Some(LanguageRegistry::test(cx.background_executor())),
            cx,
        )
    });
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let path = Some(std::path::PathBuf::from("test.js"));
    let context = edit_agent.analyze_file_context(&snapshot, &path);
    assert!(!context.is_empty());
    assert!(context.contains("JavaScript/TypeScript file"));
    
    Ok(())
}

#[gpui::test]
async fn test_agentic_editing_context_analysis_edge_cases(cx: &mut TestAppContext) -> Result<()> {
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), None, cx).await;
    let action_log = cx.new(|_| ActionLog::new(project.clone()));
    let model = Arc::new(FakeLanguageModel::default());
    let templates = Arc::new(Templates::new());
    
    let edit_agent = EditAgent::new(
        model.clone(),
        project.clone(),
        action_log.clone(),
        templates,
        EditFormat::XmlTags,
    )
    .with_consciousness(true)
    .with_deliberation(true);

    // Test with empty buffer
    let empty_buffer = cx.new(|cx| {
        Buffer::from_text(
            "",
            Some(LanguageRegistry::test(cx.background_executor())),
            cx,
        )
    });
    let snapshot = empty_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let context = edit_agent.analyze_file_context(&snapshot, &None);
    assert!(!context.is_empty());
    assert!(context.contains("File size: 0 lines"));

    // Test with very large file (should trigger warning)
    let large_content = "line\n".repeat(600);
    let large_buffer = cx.new(|cx| {
        Buffer::from_text(
            &large_content,
            Some(LanguageRegistry::test(cx.background_executor())),
            cx,
        )
    });
    let snapshot = large_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let context = edit_agent.analyze_file_context(&snapshot, &Some(std::path::PathBuf::from("large.txt")));
    assert!(context.contains("Large file - consider if changes could be broken into smaller pieces"));

    // Test with file containing TODO comments
    let todo_buffer = cx.new(|cx| {
        Buffer::from_text(
            "fn main() {\n    // TODO: implement this\n    println!(\"Hello\");\n}",
            Some(LanguageRegistry::test(cx.background_executor())),
            cx,
        )
    });
    let snapshot = todo_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let context = edit_agent.analyze_file_context(&snapshot, &Some(std::path::PathBuf::from("todo.rs")));
    assert!(context.contains("Contains TODO/FIXME comments"));
    assert!(context.contains("Contains main function"));

    Ok(())
}