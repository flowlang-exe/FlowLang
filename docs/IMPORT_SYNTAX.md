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

## 📦 Package Imports

### Direct URL Import (Recommended)
Import directly from GitHub. **Auto-installs** on first use:

```flow
circle module from "github.com/flowlang-exe/module@main"
circle crypto from "github.com/user/crypto@v1.0.0"
circle utils from "github.com/user/repo@a13f92d"
```

Supported hosts: `github.com`, `gitlab.com`, `bitbucket.org`

Format: `<host>/<owner>/<repo>@<branch|tag|commit>`

> **Note:** First import downloads the package. Subsequent runs use local cache.

### Alias Import (`pkg:`)
Use a package alias defined in `config.flowlang.json`:

```flow
circle http from "pkg:http"
circle db from "pkg:database"
```

> ⚠️ **Important:** `pkg:` only works if the alias is defined in your config's `packages` section:
> ```json
> {
>   "packages": {
>     "http": "github.com/flowlang-exe/http@main",
>     "database": "github.com/user/db@v2.0.0"
>   }
> }
> ```

### CLI Commands

| Command | Description |
|---------|-------------|
| `flowlang add <url>` | Add package to config and install |
| `flowlang install` | Install all packages from config |

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
| Standard lib | `circle <name> from "std:<lib>"` | `circle math from "std:math"` |
| Local file | `circle <name> from "<path>"` | `circle utils from "./utils.flow"` |
| GitHub URL | `circle <name> from "<url>@<ref>"` | `circle m from "github.com/user/repo@main"` |
| Package alias | `circle <name> from "pkg:<alias>"` | `circle http from "pkg:http"` |
| With alias | `circle <name> from "<path>" as <alias>` | `circle color from "std:color" as c` |
| Selective | `circle {members} from "<path>"` | `circle {add, PI} from "math.flow"` |

**Remember:** `from` → `as` (alphabetical order!)

