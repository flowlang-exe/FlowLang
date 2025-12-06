# FlowLang Episode Test Examples

This directory contains test examples for all 23 episodes of the FlowLang ERROR ARC system.

## Season 1: Fatal Errors (Episodes 1-15)

### Episode 1: THE SHATTERED SIGIL
**File:** `ep01_syntax_error.flow`
**Error:** Syntax Error
**Trigger:** Missing closing brace

### Episode 3: THE UNBOUND VARIABLE
**File:** `ep03_undefined_var.flow`
**Error:** Undefined Variable
**Trigger:** Using undefined variable `x`

### Episode 5: DIVISION OF FATE
**File:** `ep05_division_by_zero.flow`
**Error:** Division by Zero
**Trigger:** Dividing by 0

### Episode 8: LOCKED DOORS, SILENT FILES
**File:** `ep08_file_not_found.flow`  
**Error:** File Not Found (Rift)
**Trigger:** Reading nonexistent file

### Episode 15: WHEN THE CORE COLLAPSES
**File:** `ep15_panic.flow`
**Error:** Unhandled Panic
**Trigger:** Explicit panic statement

## Running Examples

To run an episode example:
```bash
flowlang run examples/episodes/ep01_syntax_error.flow
```

To see with trace enabled:
```bash
flowlang run --trace examples/episodes/ep03_undefined_var.flow
```

## Expected Behavior

Each example should display:
- Episode banner with number and title
- Error details
- Scene location
- "Next Time" teaser message
