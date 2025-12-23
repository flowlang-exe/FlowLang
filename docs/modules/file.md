# std:file

File system operations.

## Import

```flowlang
circle fs from "std:file"
```

## Functions

### `read(path: Silk) -> Silk`
Read entire file as string.

```flowlang
attempt {
    let content = fs.read("data.txt")
    shout(content)
} rescue Rift as e {
    shout("Error reading file: " + e)
}
```

### `write(path: Silk, content: Silk) -> Pulse`
Write string to file (overwrites existing). Returns `both!` on success.

```flowlang
fs.write("output.txt", "Hello, World!")
```

### `exists(path: Silk) -> Pulse`
Check if file or directory exists.

```flowlang
in Stance (fs.exists("config.json")) {
    shout("Config found!")
}
```

### `delete(path: Silk) -> Pulse`
Delete file or directory (recursively).

```flowlang
fs.delete("temp.txt")
```

### `list(path: Silk) -> Constellation<Silk>`
List files in directory.

```flowlang
let files = fs.list("./data")
```

### `create_dir(path: Silk) -> Pulse`
Create directory and parents.

```flowlang
fs.create_dir("./logs/today")
```
