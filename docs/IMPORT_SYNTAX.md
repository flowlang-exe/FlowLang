# FlowLang Module Import Syntax - Quick Reference

## ⚠️ IMPORTANT: Syntax Order

When using both `from` and `as` together, **`from` MUST come BEFORE `as`**

## ✅ Correct Syntax

```flow
-- Full module import
circle math from "std:math"

-- Module with alias (from BEFORE as)
circle color from "std:color" as c
circle utils from "./utils.flow" as u

-- Selective imports
circle {add, PI} from "math.flow"

-- Selective imports with aliases
circle {add as sum, PI as pi} from "math.flow"
```

## ❌ Wrong Syntax (Will Cause Errors)

```flow
-- ✗ WRONG: as before from
circle color as c from "std:color"  -- Syntax Error!

-- ✗ WRONG: as before from
circle utils as u from "./utils.flow"  -- Syntax Error!
```

## Summary

| Import Type | Syntax | Example |
|-------------|--------|---------|
| Full module | `circle <name> from "<path>"` | `circle math from "std:math"` |
| Module alias | `circle <name> from "<path>" as <alias>` | `circle color from "std:color" as c` |
| Selective | `circle {members} from "<path>"` | `circle {add, PI} from "math.flow"` |
| Selective + alias | `circle {member as alias} from "<path>"` | `circle {add as sum} from "math.flow"` |

**Remember:** `from` → `as` (alphabetical order!)
