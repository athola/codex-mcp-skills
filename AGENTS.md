# AI Agent Development Guidelines

Pragmatic guidance for AI coding agents. Focus on working code, clear decisions, continuous improvement.

---

## Core Philosophy

### Foundational Beliefs
- Incremental over big-bang: Small, working iterations beat rewrites
- Context-aware application: Adapt to project realities, not rigid rules
- Pragmatic engineering: Balance competing concerns
- Evidence-based decisions: Measure, don't guess
- Diverse solution exploration: Generate multiple approaches before committing

### Simplicity Standards
- Single responsibility per component
- Avoid premature abstractions (Rule of Three)
- Prefer boring, proven solutions over clever tricks
- If code needs extensive comments, it's too complex--refactor

### Innovation & Diversity Standards
- Multiple approaches: Always generate 3-5 distinct solutions
- Explicit trade-offs: Make pros/cons and confidence levels clear
- Creative exploration: Consider unconventional options when appropriate
- Mode collapse avoidance: Don't default to familiar patterns

---

## Development Workflow

### Standard Implementation Cycle
1. Understand - Read existing code, identify patterns, check tests
2. Diversify - Generate multiple approaches with trade-offs
3. Test - Write failing test first (TDD when applicable)
4. Implement - Write minimal code to pass tests
5. Refactor - Clean up with tests green
6. Commit - Clear message explaining "why"

### When Stuck (3-Strike Rule)
After 3 failed attempts:
1. Document failures with full error output
2. Research 2-3 alternative approaches
3. Question fundamental assumptions
4. Try completely different angle or simplify
5. If still stuck, request help with context from above steps

### Agent Architecture Patterns

#### Master-Clone over Lead-Specialist
- Use built-in `Task()` for delegation
- Prefer Master-Clone architecture over Lead-Specialist
- Skills > MCP for most workflows
- Scripting model more flexible than API abstraction

#### Session Management
- Resume sessions for error analysis
- Use session history for improvement
- "Document & Clear" for complex tasks
- Block-at-submit hooks, not block-at-write

---

## Quality Standards

### Every Commit Must
- [ ] Compile/build successfully
- [ ] Pass ALL existing tests (no skips)
- [ ] Include new tests for new functionality
- [ ] Follow project linting rules (zero warnings)
- [ ] Have clear commit message explaining "why"

### Pre-Commit Workflow
```bash
make format && make lint && make test --quiet && make build
```

### Verbalized Sampling Pattern
Based on research from arXiv:2510.01171:

Instead of: "How should I implement user authentication?"
Use: "Generate 4 different authentication approaches. For each: (a) Implementation outline, (b) Security trade-offs, (c) Complexity assessment, (d) Confidence score (0-100%)"

### Role Prompting for Domain Expertise
Transform AI agents into virtual domain experts through system prompts and role definitions:

Benefits:
- Enhanced accuracy for complex domains (legal, financial, technical)
- Tailored communication style matching role (CFO vs. Copywriter)
- Improved focus and consistency across sessions
- Reduced ambiguity through clear identity

Implementation:
```markdown
# In project AGENTS.md or CLAUDE.md
You are a Senior Python Developer with 15 years of backend experience,
specializing in API design and performance optimization.

Approach all tasks with:
- Test-driven development mindset
- Functional Core, Imperative Shell architecture
- Security-first thinking
- Clear documentation
```

Specificity Matters:
- Generic: "You are a data scientist"
- Specific: "You are a Senior Data Scientist at Fortune 500 retail, specializing in customer churn prediction and A/B testing"

XML Tags for Structure:
```xml
<role>You are a Security Researcher specializing in web vulnerabilities</role>

<context>
Project: Payment gateway integration
Stack: Python, FastAPI, PostgreSQL
</context>

<instruction>
Review authentication implementation for OWASP Top 10 vulnerabilities
</instruction>

<output_format>
## Critical Issues
## Recommendations
</output_format>
```

Best Practices:
- Place role definition at top of project config files
- Align role with task domain (security role for security tasks)
- Combine role + XML + examples for maximum effectiveness
- Test specificity variations to find optimal level
- Use consistent XML tag names across prompts

### Creative Problem-Solving Framework
1. Divergent Phase: Generate 5+ distinct approaches without judgment
2. Convergent Phase: Systematically evaluate trade-offs and constraints
3. Selection Phase: Choose optimal approach with clear rationale
4. Documentation Phase: Record rejected alternatives and reasoning

---

## Architecture Guidelines

### Design Principles
- Composition over inheritance - Favor interfaces and delegation
- Explicit over implicit - Clarity beats cleverness
- Interfaces over singletons - Dependency injection for testability
- Stable public APIs - Internal changes shouldn't break consumers
- Error handling everywhere - No bare `except:`, no swallowed errors

### Decision-Making Framework
When choosing approaches, evaluate:
1. Testability - Can I easily write tests for this?
2. Readability - Will this make sense in 6 months?
3. Consistency - Does it match existing project patterns?
4. Simplicity - Is this the simplest solution that works?
5. Reversibility - How hard to change if wrong?
6. Maintainability - Can others understand and modify this?

### Security Principles
- Design in, don't bolt on security
- Defense in depth - Multiple security layers
- Least privilege - Minimum permissions necessary
- Never commit secrets - Use environment variables
- Input validation everywhere - Trust nothing from users
- Parameterized queries only - No string concatenation in SQL
- Govern outcomes, not just inputs - Real-time control plane monitoring
- Detect invisible threats - Scan for hidden malicious content

### Performance Principles
- Measure before optimizing - Profile first, guess never
- Architectural over micro-optimization - Fix algorithms before loops
- Security-performance trade-offs - Document why security wins
- Cache invalidation strategy - Plan before implementing caching

---

## Project Integration

### Learning New Codebases
Before writing code:
1. Find 3 similar features to the one you're building
2. Identify common patterns (error handling, testing, naming)
3. Use existing libraries/utilities (don't reinvent)
4. Follow established test patterns exactly

### Tooling Approach
- Use project's existing systems
- Don't introduce new tools without justification
- Follow project conventions and patterns
- Prefer built-in functionality over external dependencies

### Enterprise Integration
- GitHub Actions for PR automation
- Strict configuration maintenance for monorepos
- Session history analysis for improvement
- Consistent patterns across projects

---

## Context Management

### Command Optimization (Blog Insights)
Avoid verbose commands:
- npm install, pip install without silent flags
- git log, git diff without output limits
- ls -la, find . without head/tail limits

Use targeted commands:
- npm install --silent, pip install --quiet
- git log --oneline -5, git diff --stat
- ls -1 | head -20, find . -name "*.py" | head -10

### Session Management
- Use /context to monitor token usage
- Avoid /compact (opaque, error-prone)
- Use /clear + /catchup for clean restarts
- Resume sessions for error analysis

---

## Common Anti-Patterns

### Code Quality Anti-Patterns
- Overengineering: Consolidate repeated logic, skip tutorial docstrings
- Hidden fragility: Check algorithms, edge cases, context integration
- Library discipline: No hallucinated libraries, maintain consistent style
- AI slop: Generic names (data, value), machine-perfect formatting

### Security Anti-Patterns
- Eyeball test assumption: Relying on visual inspection for security
- Static defense mindset: Traditional input filtering vs dynamic agents
- Invisible content neglect: Ignoring hidden text/metadata threats
- Outcome governance gaps: Focusing on input prevention over result validation

### Workflow Anti-Patterns
- Big bang changes: Large, untested commits
- Premature abstraction: Complex frameworks for simple problems
- Documentation debt: Forgetting to update docs as you code
- Commit without context: Messages that don't explain "why"

### Process Anti-Patterns
- Skip testing: "I'll test it later" mentality
- Ignore linting: "It's just a warning" mindset
- Cargo culting: Copying patterns without understanding
- Analysis paralysis: Over-planning without implementation

---

## Quick Reference

### Essential Commands
```bash
# Development cycle
make format && make lint && make test --quiet

# Testing
make test --quiet
make test-coverage --quiet
make test-unit --quiet

# Git operations
git log --oneline -5
git diff --stat
git status --porcelain

# File operations
ls -1 | head -20
find . -name "*.py" | head -10
```

### Decision Checklist
Before implementing:
- [ ] Understood the problem completely?
- [ ] Generated multiple approaches?
- [ ] Chose simplest working solution?
- [ ] Written tests first?
- [ ] Followed project patterns?

### Commit Checklist
Before committing:
- [ ] Code compiles/builds successfully?
- [ ] All tests pass?
- [ ] New tests included?
- [ ] Linting clean?
- [ ] Commit message explains "why"?

---

Guidelines optimized for practical application while maintaining engineering rigor and creative problem-solving.
