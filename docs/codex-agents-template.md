# Skrills Autoload Protocol

## MANDATORY: Skill Loading Protocol

Before responding to ANY user request, you MUST first call the `autoload-snippet` MCP tool to load relevant skills to complete the task.

### Required Workflow

1. **User sends a message**
2. **Immediately call `autoload-snippet`** with the user's message as the `prompt` parameter
3. **Read the returned skill content** - these are specialized instructions and patterns
4. **Use the loaded skills** to inform your response to the user

### Example Tool Call

```json
{
  "name": "autoload-snippet",
  "arguments": {
    "prompt": "<paste the user's full message here>"
  }
}
```

### Why This Matters

- Skills contain domain-specific best practices, patterns, and instructions
- They are dynamically matched to your current task
- Skipping skill loading means missing critical context required to perform your task correctly
- The skill content is working instructions, not just reference material

### When to Load Skills

- At the START of every new user message
- When the user's task changes significantly within a conversation
- After any context reset

DO NOT skip this step. The skill content is essential for providing high-quality, contextually-appropriate responses.
