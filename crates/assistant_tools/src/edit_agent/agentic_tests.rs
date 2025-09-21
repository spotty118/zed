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
    let action_log = cx.new(|_| ActionLog::new());
    let model = Arc::new(FakeLanguageModel::new());
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
    
    Ok(())
}

#[gpui::test]
async fn test_agentic_editing_disabled(cx: &mut TestAppContext) -> Result<()> {
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), None, cx).await;
    let action_log = cx.new(|_| ActionLog::new());
    let model = Arc::new(FakeLanguageModel::new());
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
    
    Ok(())
}