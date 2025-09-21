# Agentic Editing

Zed's agentic editing feature enhances the AI's consciousness and deliberation when making code changes, resulting in more thoughtful and context-aware modifications.

## What is Agentic Editing?

Agentic editing makes the AI more "conscious" by:

1. **Context Analysis**: The AI analyzes the file structure, patterns, and purpose before making changes
2. **Deliberation**: Complex edits go through a deliberation phase where the AI thinks through implications
3. **Consciousness Logging**: Transparent decision-making process shown to users
4. **Safe Modifications**: Enhanced prompts that encourage minimal, careful changes

## Features

### Consciousness Mode
When enabled, the AI:
- Analyzes file type and coding patterns
- Detects indentation style and maintains consistency  
- Identifies file purpose (library code, tests, main entry point)
- Considers potential impacts before making changes

### Deliberation Process
For complex edits, the AI:
- Takes time to plan the changes
- Considers risks and side effects
- Ensures changes align with existing code style
- Provides reasoning for decisions

### Enhanced Prompting
Agentic prompts encourage the AI to:
- Think step-by-step through changes
- Explain reasoning behind modifications
- Consider the broader context of the codebase
- Make minimal, surgical changes

## Configuration

You can control agentic editing through your Zed settings:

```json
{
  "agent": {
    "agentic_editing_enabled": true,
    "agentic_deliberation_enabled": true
  }
}
```

### Settings

- `agentic_editing_enabled` (default: `true`): Enable consciousness and context analysis
- `agentic_deliberation_enabled` (default: `true`): Enable deliberation phase for complex edits

## User Interface

When agentic editing is active, you'll see:

- 🧠 **Deliberation status**: Shows when the AI is thinking through changes
- 📝 **Context analysis**: Brief summary of what the AI understands about the file
- 💭 **Consciousness logs**: The AI's reasoning and decision-making process

## Examples

### Before (Standard Editing)
```
Make the function return an error instead of panicking
```

### After (Agentic Editing)
```
🧠 Deliberating...
📝 Context: Rust source file - focus on memory safety, ownership, and correctness
💭 Agent: I need to replace the panic with proper error handling using Result<T, E>. 
This will be safe because I'm maintaining the function signature compatibility while 
improving error handling as requested.
```

## Benefits

1. **More Thoughtful Changes**: AI considers context and implications
2. **Safer Modifications**: Reduced risk of breaking changes
3. **Transparent Process**: See the AI's reasoning
4. **Better Code Quality**: Maintains existing patterns and style
5. **Educational**: Learn from AI's decision-making process

## Compatibility

Agentic editing works with all existing editing features:
- Inline assistant
- Edit file tool
- Code completion
- All supported programming languages

The feature is backward compatible and can be disabled if preferred.