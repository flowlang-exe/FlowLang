# std:path ⚡

Path manipulation module (Node.js-style).

## Import

```flowlang
circle path from "std:path"
```

## Properties
- `path.sep`: Platform path separator (`\` on Windows, `/` on Unix)

## Functions

### `join(...parts) -> Silk`
Join path segments with platform separator.

```flowlang
let p = path.join("home", "user", "docs")  -- "home/user/docs"
```

### `dirname(path: Silk) -> Silk`
Get directory name from path.

```flowlang
let dir = path.dirname("/home/user/file.txt")  -- "/home/user"
```

### `basename(path: Silk, ext?: Silk) -> Silk`
Get base name from path, optionally removing extension.

```flowlang
let name = path.basename("/home/user/file.txt")         -- "file.txt"
let noExt = path.basename("/home/user/file.txt", ".txt") -- "file"
```

### `extname(path: Silk) -> Silk`
Get file extension including the dot.

```flowlang
let ext = path.extname("myfile.json")  -- ".json"
```

### `parse(path: Silk) -> Relic`
Parse path into components.

```flowlang
let p = path.parse("/home/user/file.txt")
-- p.root = "/"
-- p.dir  = "/home/user"
-- p.base = "file.txt"
-- p.ext  = ".txt"
-- p.name = "file"
```

### `format(pathObject: Relic) -> Silk`
Format path object back to string.

```flowlang
let p = path.format({"dir": "/home/user", "base": "file.txt"})
-- "/home/user/file.txt"
```

### `resolve(...paths) -> Silk`
Resolve path segments to absolute path.

```flowlang
let abs = path.resolve("src", "lib", "utils.js")
```

### `normalize(path: Silk) -> Silk`
Normalize a path, resolving '..' and '.'.

```flowlang
let n = path.normalize("/home/user/../user/./file.txt")
-- "/home/user/file.txt"
```

### `isAbsolute(path: Silk) -> Pulse`
Check if path is absolute.

```flowlang
path.isAbsolute("/home/user")  -- both!
```

### `relative(from: Silk, to: Silk) -> Silk`
Get relative path from one path to another.

```flowlang
let rel = path.relative("/home/user", "/home/user/docs/file.txt")
-- "docs/file.txt"
```
