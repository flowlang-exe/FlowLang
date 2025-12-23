# std:stream ⚡

File streaming and IO operations.

## Import

```flowlang
circle stream from "std:stream"
```

## Functions

### `readFile(path: Silk) -> Relic`
Read a file with metadata.

```flowlang
let file = stream.readFile("./index.html")
shout(file.content)   -- File contents
shout(file.size)      -- File size in bytes
shout(file.mimeType)  -- "text/html"
```

### `readText(path: Silk) -> Silk`
Read a file as text.

```flowlang
let content = stream.readText("./data.txt")
```

### `readBytes(path: Silk) -> Constellation<Ember>`
Read a file as bytes (array of numbers).

```flowlang
let bytes = stream.readBytes("./image.png")
```

### `writeFile(path: Silk, content: Silk) -> Pulse`
Write content to a file.

```flowlang
stream.writeFile("./output.txt", "Hello World!")
```

### `exists(path: Silk) -> Pulse`
Check if a file exists.

```flowlang
in Stance (stream.exists("./config.json")) {
    shout("Config found!")
}
```

### `stat(path: Silk) -> Relic`
Get file statistics.

```flowlang
let info = stream.stat("./myfile.txt")
shout(info.size)    -- File size
shout(info.isFile)  -- both! or none!
shout(info.isDir)   -- both! or none!
shout(info.modified) -- Timestamp
```

### `mimeType(path: Silk) -> Silk`
Get MIME type for a file.

```flowlang
let mime = stream.mimeType("./style.css")  -- "text/css"
```
