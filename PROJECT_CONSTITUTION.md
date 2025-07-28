# 📜 Project Constitution "Mycelium/AIbox"

This document contains the immutable rules for development that must be followed by all code created within this project. Deviations are not allowed.

## 🚫 No Simulations or Stubs

Any code must be real and functional. We do not use temporary solutions, mocks, or simulations in the main code. If a mock is needed for testing, it must be exclusively in the test environment and never reach the release branch.

## 🏗️ Systemicity and Scalability

Every function, every module must be designed with future scaling in mind. Architecture must be modular, with low coupling and high cohesion, so that replacing or updating one component does not destroy the entire system.

## 🛡️ Absolute Fault Tolerance

All code that interacts with the network, file system, or external APIs must be wrapped in exception handling blocks (try...except in Python, Result/Option in Rust). The program should not "crash". Every error must be logged with maximum detail for subsequent analysis.

## 📚 Impeccable Code Documentation

### English Language
All comments, variable names, function names, and class names — strictly in English.

### Docstrings/Comments
Every function, class, and method must have exhaustive docstrings explaining:

- What this code block does (its purpose)
- What arguments it accepts and what they mean
- What it returns
- What exceptions it can throw

### Inline Comments
Complex or non-obvious code sections should be accompanied by brief inline comments explaining the logic (# why this is done this way, not what is done).

## 📁 Clean Project Structure
Logical and understandable folder and file structure. Code related to p2p network lies in the p2p module, UI code — in ui, utilities — in utils, and so on. No "garbage" files in the project root.

## 🔄 Atomic and Clear Commits
Each Git commit must represent one logical change and be accompanied by a clear message written according to the standard (e.g., feat: Implement mDNS discovery or fix: Handle network connection error).

## 🧪 Testing — Not an Option, but a Necessity
Where possible, critically important modules (especially those related to network interaction and cryptography) should be covered by unit tests.

## 🔍 Code Quality Standards

### Error Handling
- No `unwrap()` or `expect()` without proper error handling
- All network operations must have timeout handling
- Graceful degradation for all external dependencies

### Logging
- Use structured logging with appropriate levels (debug, info, warn, error)
- No `println!` in production code
- Log all important events and errors

### Performance
- Avoid blocking operations in async contexts
- Use appropriate data structures for the task
- Consider memory usage and resource cleanup

### Security
- Validate all external inputs
- Use secure random number generation
- Implement proper authentication and authorization

## 🎯 Compliance Checklist

Before any code is committed, ensure:

- [ ] All comments are in English
- [ ] All functions have docstrings
- [ ] Error handling is implemented
- [ ] Logging is used instead of println!
- [ ] Code follows project structure
- [ ] Tests are written for critical functions
- [ ] No temporary solutions or stubs
- [ ] Commit message follows conventional format

## 🚨 Violation Consequences

Code that violates this Constitution will be rejected and must be rewritten to comply with all rules before being accepted.

---

*This Constitution is immutable and applies to all development phases of the Mycelium/AIbox project.*
