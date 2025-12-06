# FlowLang Standard Library

Complete reference for FlowLang's standard library modules.

## Table of Contents

- [Core Module](#core-module)
- [std:math](#stdmath)
- [std:string](#stdstring)
- [std:array](#stdarray)
- [std:file](#stdfile)
- [std:json](#stdjson)
- [std:net](#stdnet)
- [std:time](#stdtime)
- [std:timer](#stdtimer) ⚡
- [std:web](#stdweb) ⚡
- [std:url](#stdurl) ⚡
- [std:stream](#stdstream) ⚡
- [std:cli](#stdcli)
- [std:color](#stdcolor)
- [std:crypto](#stdcrypto)
- [std:os](#stdos)

## Core Module

Built-in functions available without imports.

### Output Functions

#### `whisper(message: Silk)`
Output message quietly (low volume).

```flowlang
whisper("Debug info...")
```

#### `shout(message: Silk)`
Output message normally (standard output).

```flowlang
shout("Hello, World!")
```

#### `roar(message: Silk)`
Output message loudly (emphasized).

```flowlang
roar("IMPORTANT MESSAGE!")
```

#### `chant(message: Silk)`
Output message with highlighting.

```flowlang
chant("✨ Special announcement ✨")
```

---

## std:math

Mathematical operations and constants.

### Import

```flowlang
circle math from std:math
```

### Functions

#### `sin(x: Ember) -> Ember`
Calculate sine of x (in radians).

```flowlang
let result = math.sin(1.5708)  -- π/2 ≈ 1.0
```

#### `cos(x: Ember) -> Ember`
Calculate cosine of x (in radians).

```flowlang
let result = math.cos(0)  -- 1.0
```

#### `sqrt(x: Ember) -> Ember`
Calculate square root of x.

```flowlang
let result = math.sqrt(16)  -- 4.0
```

#### `abs(x: Ember) -> Ember`
Calculate absolute value of x.

```flowlang
let result = math.abs(-42)  -- 42.0
```

#### `round(x: Ember) -> Ember`
Round x to nearest integer.

```flowlang
let result = math.round(3.7)  -- 4.0
```

#### `floor(x: Ember) -> Ember`
Round x down to integer.

```flowlang
let result =  math.floor(3.7)  -- 3.0
```

#### `ceil(x: Ember) -> Ember`
Round x up to integer.

```flowlang
let result =  math.ceil(3.2)  -- 4.0
```

#### `pow(base: Ember, exp: Ember) -> Ember`
Calculate base raised to exponent.

```flowlang
let result =  math.pow(2, 8)  -- 256.0
```

---

##  string

String manipulation functions.

### Import

```flowlang
circle  string
```

### Functions

#### `len(s: Silk) -> Ember`
Get length of string.

```flowlang
let length =  string.len("Hello")  -- 5.0
```

#### `upper(s: Silk) -> Silk`
Convert string to uppercase.

```flowlang
let result =  string.upper("hello")  -- "HELLO"
```

#### `lower(s: Silk) -> Silk`
Convert string to lowercase.

```flowlang
let result =  string.lower("WORLD")  -- "world"
```

#### `trim(s: Silk) -> Silk`
Remove whitespace from both ends.

```flowlang
let result =  string.trim("  hello  ")  -- "hello"
```

#### `split(s: Silk, delimiter: Silk) -> Constellation`
Split string by delimiter into array.

```flowlang
let parts =  string.split("a,b,c", ",")  -- ["a", "b", "c"]
```

#### `substring(s: Silk, start: Ember, end: Ember) -> Silk`
Extract substring from start to end index.

```flowlang
let sub =  string.substring("Hello", 1, 4)  -- "ell"
```

---

##  array

Array manipulation functions.

### Import

```flowlang
circle array from std:array
```

### Functions

#### `len(arr: Constellation) -> Ember`
Get length of array.

```flowlang
let size =  array.len([1, 2, 3])  -- 3.0
```

#### `push(arr: Constellation, value: any) -> Constellation`
Add element to end of array.

```flowlang
let newArr =  array.push([1, 2], 3)  -- [1, 2, 3]
```

#### `pop(arr: Constellation) -> any`
Remove and return last element.

```flowlang
let last =  array.pop([1, 2, 3])  -- 3
```

#### `contains(arr: Constellation, value: any) -> Pulse`
Check if array contains value.

```flowlang
let found =  array.contains([1, 2, 3], 2)  -- both!
```

#### `join(arr: Constellation, delimiter: Silk) -> Silk`
Join array elements into string.

```flowlang
let str =  array.join(["a", "b", "c"], ", ")  -- "a, b, c"
```

---

## std:file

File system operations.

### Import

```flowlang
circle fs from "std:file"
```

### Functions

#### `read(path: Silk) -> Silk`
Read entire file as string.

```flowlang
attempt {
    let content = fs.read("data.txt")
    shout(content)
} rescue Rift as e {
    shout("Error reading file: " + e)
}
```

#### `write(path: Silk, content: Silk) -> Hollow`
Write string to file (overwrites existing).

```flowlang
fs.write("output.txt", "Hello, World!")
```

#### `append(path: Silk, content: Silk) -> Hollow`
Append string to file.

```flowlang
fs.append("log.txt", "New entry\n")
```

#### `exists(path: Silk) -> Pulse`
Check if file or directory exists.

```flowlang
in Stance (fs.exists("config.json")) {
    shout("Config found!")
}
```

#### `delete(path: Silk) -> Hollow`
Delete file.

```flowlang
fs.delete("temp.txt")
```

#### `list(path: Silk) -> Constellation`
List files in directory.

```flowlang
let files = fs.list("./data")
```

---

## std:json

JSON parsing and serialization.

### Import

```flowlang
circle json from "std:json"
```

### Functions

#### `parse(json: Silk) -> Relic`
Parse JSON string into object.

```flowlang
let data = json.parse('{"name": "Goku", "level": 9000}')
shout(data["name"])  -- "Goku"
```

#### `stringify(value: any) -> Silk`
Convert value to JSON string.

```flowlang
let obj = {"name": "Naruto", "rank": "Hokage"}
let jsonStr = json.stringify(obj)
shout(jsonStr)  -- '{"name":"Naruto","rank":"Hokage"}'
```

---

## std:net

Network module for making HTTP requests.

### Import

```flowlang
circle net from "std:net"
```

### Functions

#### `get(url: Silk) -> Silk`
Make HTTP GET request.

```flowlang
attempt {
    let response = net.get("https://api.example.com/data")
    let data = json.parse(response)
    shout(data["message"])
} rescue Rift as e {
    shout("Request failed: " + e)
}
```

#### `post(url: Silk, body: Silk) -> Silk`
Make HTTP POST request.

```flowlang
let payload = json.stringify({"key": "value"})
let response = net.post("https://api.example.com/create", payload)
```

#### `put(url: Silk, body: Silk) -> Silk`
Make HTTP PUT request.

```flowlang
let response = net.put("https://api.example.com/update/1", payload)
```

#### `delete(url: Silk) -> Silk`
Make HTTP DELETE request.

```flowlang
net.delete("https://api.example.com/item/1")
```

---

## std:time

Time and date operations.

### Import

```flowlang
circle time from "std:time"
```

### Functions

#### `now() -> Ember`
Get current Unix timestamp (seconds since epoch).

```flowlang
let timestamp = time.now()
shout("Current time: " + timestamp)
```

#### `format(timestamp: Ember, format: Silk) -> Silk`
Format timestamp as string.

```flowlang
let t = time.now()
let formatted = time.format(t, "%Y-%m-%d %H:%M:%S")
shout(formatted)  -- "2024-01-15 14:30:45"
```

#### `sleep(duration: Ember) -> Hollow`
Sleep for specified milliseconds.

```flowlang
shout("Waiting...")
time.sleep(2000)  -- Sleep 2 seconds
shout("Done!")
```

#### `timestamp() -> Ember`
Get current timestamp in milliseconds.

```flowlang
let ms = time.timestamp()
```

---

## std:timer

Asynchronous timer functions for intervals and timeouts. Timers keep the FlowLang process alive until cleared.

### Import

```flowlang
circle timer from "std:timer"
```

### Functions

#### `interval(ms: Ember, callback: Spell) -> Handle`
Create a repeating timer that calls the callback every `ms` milliseconds. Returns a Handle that can be used to cancel the timer.

> **Note:** Callbacks execute during `wait` statements or after script completion while in the event loop.

```flowlang
let count = 0

cast Spell tick() -> Hollow {
    count = count + 1
    shout("🔔 Tick #" + count)
}

let handle = timer.interval(1000, tick)  -- Every 1 second

wait 5s  -- Ticks will execute during wait
timer.clear(handle)  -- Stop the timer
```

#### `timeout(ms: Ember, callback: Spell) -> Handle`
Create a one-shot timer that calls the callback after `ms` milliseconds. The timer automatically clears after execution.

```flowlang
cast Spell done() -> Hollow {
    shout("⏰ Timer finished!")
}

timer.timeout(3000, done)  -- Fire once after 3 seconds
wait 4s  -- Wait for timeout to execute
```

#### `clear(handle: Handle) -> Pulse`
Cancel a timer by its handle. Returns `both!` if the timer was successfully cancelled, `none!` if already cleared.

```flowlang
let handle = timer.interval(500, myCallback)
wait 2s
let cleared = timer.clear(handle)
shout("Timer cleared: " + cleared)  -- "Timer cleared: both!"
```

### Complete Example

```flowlang
circle timer from "std:timer"

shout("⏱️  Timer Demo")

let ticks = 0

cast Spell onTick() -> Hollow {
    ticks = ticks + 1
    shout("Tick " + ticks)
}

cast Spell onDone() -> Hollow {
    shout("🎉 Timeout complete!")
}

-- Create interval (repeats every 500ms)
let intervalHandle = timer.interval(500, onTick)

-- Create timeout (fires once after 2 seconds)
timer.timeout(2000, onDone)

-- Wait for callbacks to execute
wait 3s

-- Clean up
timer.clear(intervalHandle)
shout("Final tick count: " + ticks)
```

---

## std:cli

Command-line interface interactions.

### Import

```flowlang
circle cli from "std:cli"
```

### Functions

#### `input(prompt: Silk) -> Silk`
Read user input from terminal.

```flowlang
let name = cli.input("Enter your name: ")
shout("Hello, " + name)
```

#### `args() -> Constellation<Silk>`
Get command-line arguments passed to script.

```flowlang
let args = cli.args()
-- Run with: flowlang run script.flow arg1 arg2
```

#### `confirm(prompt: Silk) -> Pulse`
Ask yes/no question.

```flowlang
let proceed = cli.confirm("Continue? (y/n): ")
in Stance (proceed) {
    shout("Proceeding...")
}
```

#### `select(prompt: Silk, options: Constellation<Silk>) -> Silk`
Display menu and get user selection.

```flowlang
let choice = cli.select("Choose:", ["Option 1", "Option 2", "Option 3"])
shout("You selected: " + choice)
```

#### `clear() -> Hollow`
Clear the terminal screen.

```flowlang
cli.clear()
```

#### `exit(code: Ember) -> Hollow`
Exit program with status code.

```flowlang
cli.exit(0)  -- Success
cli.exit(1)  -- Error
```

---

## std:color

Terminal colors and text styling.

### Import

```flowlang
circle color from "std:color"
```

### Functions

#### Basic Colors
- `red(text: Silk) -> Silk`
- `green(text: Silk) -> Silk`
- `blue(text: Silk) -> Silk`
- `yellow(text: Silk) -> Silk`
- `magenta(text: Silk) -> Silk`
- `cyan(text: Silk) -> Silk`
- `white(text: Silk) -> Silk`
- `black(text: Silk) -> Silk`

#### Bright Colors
- `bright_red(text: Silk) -> Silk`
- `bright_green(text: Silk) -> Silk`
- `bright_blue(text: Silk) -> Silk`
- `bright_yellow(text: Silk) -> Silk`
- `bright_magenta(text: Silk) -> Silk`
- `bright_cyan(text: Silk) -> Silk`

#### Text Styles
- `bold(text: Silk) -> Silk`
- `italic(text: Silk) -> Silk`
- `underline(text: Silk) -> Silk`
- `dimmed(text: Silk) -> Silk`
- `strikethrough(text: Silk) -> Silk`

### Example

```flowlang
shout(color.red("Error!"))
shout(color.green("Success!"))
shout(color.bold(color.blue("Important")))
```

---

## std:crypto

Cryptography functions for hashing and encoding.

### Import

```flowlang
circle crypto from "std:crypto"
```

### Functions

#### `md5(text: Silk) -> Silk`
Calculate MD5 hash.

```flowlang
let hash = crypto.md5("password")
-- Returns: 5f4dcc3b5aa765d61d8327deb882cf99
```

#### `sha256(text: Silk) -> Silk`
Calculate SHA256 hash.

```flowlang
let hash = crypto.sha256("secret")
-- Returns: 2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b
```

#### `sha512(text: Silk) -> Silk`
Calculate SHA512 hash.

```flowlang
let hash = crypto.sha512("data")
```

#### `base64_encode(text: Silk) -> Silk`
Encode text to Base64.

```flowlang
let encoded = crypto.base64_encode("Hello")
-- Returns: SGVsbG8=
```

#### `base64_decode(encoded: Silk) -> Silk`
Decode Base64 text.

```flowlang
let decoded = crypto.base64_decode("SGVsbG8=")
-- Returns: Hello
```

#### `hex_encode(text: Silk) -> Silk`
Encode text to hexadecimal.

```flowlang
let hex = crypto.hex_encode("ABC")
-- Returns: 414243
```

#### `hex_decode(hex: Silk) -> Silk`
Decode hexadecimal to text.

```flowlang
let text = crypto.hex_decode("414243")
-- Returns: ABC
```

---

## std:os

Operating system information and environment.

### Import

```flowlang
circle os from "std:os"
```

### Functions

#### `name() -> Silk`
Get OS name (windows, linux, macos).

```flowlang
let osName = os.name()
shout("Running on: " + osName)
```

#### `arch() -> Silk`
Get system architecture (x86_64, aarch64, etc.).

```flowlang
let arch = os.arch()
```

#### `family() -> Silk`
Get OS family (windows, unix).

```flowlang
let family = os.family()
```

#### `version() -> Silk`
Get OS version.

```flowlang
let version = os.version()
```

#### `env(name: Silk) -> Silk | Hollow`
Get environment variable.

```flowlang
let path = os.env("PATH")
let home = os.env("HOME")
```

#### `set_env(name: Silk, value: Silk) -> Hollow`
Set environment variable.

```flowlang
os.set_env("MY_VAR", "value")
```

#### `cwd() -> Silk`
Get current working directory.

```flowlang
let dir = os.cwd()
shout("Working in: " + dir)
```

#### `home_dir() -> Silk | Hollow`
Get user's home directory.

```flowlang
let home = os.home_dir()
```

#### `pid() -> Ember`
Get current process ID.

```flowlang
let processId = os.pid()
```

---

## Complete Example

```flowlang
circle fs from "std:file"
circle json from "std:json"
circle net from "std:net"
circle time from "std:time"

cast Spell main() -> Hollow {
    -- Read config from file
    attempt {
        let configText = fs.read("config.json")
        let config = json.parse(configText)
        
        -- Make API request
        let apiUrl = config["api_url"]
        let response = net.get(apiUrl)
        let data = json.parse(response)
        
        -- Save results
        let timestamp = time.now()
        let output = json.stringify({
            "timestamp": timestamp,
            "data": data
        })
        fs.write("results.json", output)
        
        shout("✅ Data fetched and saved successfully!")
        
    } rescue Rift as e {
        shout("❌ Network error: " + e)
    } rescue Glitch as e {
        shout("❌ JSON parse error: " + e)
    }
}

main()
```

---

## std:web ⚡

HTTP server request/response handling. Server handles keep the process alive automatically.

### Import

```flowlang
circle web from "std:web"
```

### Functions

#### `serve(port: Ember, handler: Spell) -> Handle`
Start an HTTP server. Handler receives `(req, res)` arguments.

```flowlang
cast Spell handler(req, res) {
    return res.json({"message": "Hello!"})
}

web.serve(3000, handler)
```

---

### Request Object (`req`)

The handler receives a request object with these properties:

| Property | Type | Description |
|----------|------|-------------|
| `req.method` | Silk | HTTP method ("GET", "POST", etc.) |
| `req.path` | Silk | Full path with query string |
| `req.pathname` | Silk | Path without query string |
| `req.url` | Silk | Full URL |
| `req.query` | Relic | Parsed query parameters |
| `req.headers` | Relic | Request headers (lowercase keys) |
| `req.cookies` | Relic | Parsed cookies |
| `req.body` | Silk | Request body content |
| `req.ip` | Silk | Client IP address |
| `req.host` | Silk | Host header |
| `req.protocol` | Silk | Protocol ("http" or "https") |

#### Accessing Headers and Cookies

```flowlang
-- Access specific header
let auth = req.headers["authorization"]
let contentType = req.headers["content-type"]

-- Access cookies
let session = req.cookies["session"]
let token = req.cookies["auth_token"]
```

---

### Response Object (`res`)

The handler receives a response object with these methods:

#### Content Methods

| Method | Description |
|--------|-------------|
| `res.json(data)` | Send JSON response (auto sets Content-Type) |
| `res.html(content)` | Send HTML response |
| `res.text(content)` | Send plain text response |
| `res.send(data)` | Auto-detect type and send |
| `res.file(path)` | Serve file from disk |

```flowlang
-- JSON response
return res.json({"name": "Goku", "power": 9001})

-- HTML response
return res.html("<h1>Hello World</h1>")

-- Serve file
return res.file("./public/index.html")
```

#### Status Code Methods

| Method | Status | Description |
|--------|--------|-------------|
| `res.ok(msg?)` | 200 | OK |
| `res.created(data?)` | 201 | Created |
| `res.noContent()` | 204 | No Content |
| `res.badRequest(msg?)` | 400 | Bad Request |
| `res.unauthorized(msg?)` | 401 | Unauthorized |
| `res.forbidden(msg?)` | 403 | Forbidden |
| `res.notFound(msg?)` | 404 | Not Found |
| `res.serverError(msg?)` | 500 | Internal Server Error |
| `res.status(code, body?)` | Custom | Custom status code |

```flowlang
return res.created({"id": 123})
return res.notFound("User not found")
return res.status(418, "I'm a teapot")
```

#### Other Methods

| Method | Description |
|--------|-------------|
| `res.redirect(url)` | 302 Redirect |
| `res.header(name, value)` | Set custom header |

```flowlang
return res.redirect("/login")
return res.header("X-Custom", "value")
```

---

### Complete Example

```flowlang
circle web from "std:web"

let users = [
    {"id": 1, "name": "Goku"},
    {"id": 2, "name": "Vegeta"}
]

cast Spell handler(req, res) {
    let path = req.pathname
    
    in Stance (path is~ "/") {
        return res.html("<h1>API Server</h1>")
    }
    
    in Stance (path is~ "/api/users") {
        return res.json(users)
    }
    
    in Stance (path is~ "/api/debug") {
        return res.json({
            "method": req.method,
            "url": req.url,
            "ip": req.ip,
            "headers": req.headers
        })
    }
    
    return res.notFound("Route not found")
}

web.serve(3000, handler)
```

---


## std:url ⚡

URL parsing and manipulation (like Node.js URL module).

### Import

```flowlang
circle url from "std:url"
```

### Functions

#### `parse(urlString: Silk) -> Relic`
Parse a URL into its components.

```flowlang
let parsed = url.parse("https://example.com:8080/path?name=goku")
shout(parsed.protocol)  -- "https"
shout(parsed.hostname)  -- "example.com"
shout(parsed.port)      -- 8080
shout(parsed.pathname)  -- "/path"
shout(parsed.query.name) -- "goku"
```

#### `parseQuery(queryString: Silk) -> Relic`
Parse a query string into an object.

```flowlang
let params = url.parseQuery("?id=123&sort=asc")
shout(params.id)    -- "123"
shout(params.sort)  -- "asc"
```

#### `encode(text: Silk) -> Silk`
URL encode a string.

```flowlang
let encoded = url.encode("hello world")  -- "hello%20world"
```

#### `decode(text: Silk) -> Silk`
URL decode a string.

```flowlang
let decoded = url.decode("hello%20world")  -- "hello world"
```

#### `format(urlObject: Relic) -> Silk`
Format a URL object back into a string.

```flowlang
let urlStr = url.format({
    "protocol": "https",
    "hostname": "api.example.com",
    "pathname": "/users"
})
-- "https://api.example.com/users"
```

---

## std:stream ⚡ NEW

File streaming and IO operations.

### Import

```flowlang
circle stream from "std:stream"
```

### Functions

#### `readFile(path: Silk) -> Relic`
Read a file with metadata.

```flowlang
let file = stream.readFile("./index.html")
shout(file.content)   -- File contents
shout(file.size)      -- File size in bytes
shout(file.mimeType)  -- "text/html"
```

#### `readText(path: Silk) -> Silk`
Read a file as text.

```flowlang
let content = stream.readText("./data.txt")
```

#### `readBytes(path: Silk) -> Constellation`
Read a file as bytes (array of numbers).

```flowlang
let bytes = stream.readBytes("./image.png")
```

#### `writeFile(path: Silk, content: Silk) -> Pulse`
Write content to a file.

```flowlang
stream.writeFile("./output.txt", "Hello World!")
```

#### `exists(path: Silk) -> Pulse`
Check if a file exists.

```flowlang
in Stance (stream.exists("./config.json")) {
    shout("Config found!")
}
```

#### `stat(path: Silk) -> Relic`
Get file statistics.

```flowlang
let info = stream.stat("./myfile.txt")
shout(info.size)    -- File size
shout(info.isFile)  -- both! or none!
shout(info.isDir)   -- both! or none!
```

#### `mimeType(path: Silk) -> Silk`
Get MIME type for a file.

```flowlang
let mime = stream.mimeType("./style.css")  -- "text/css"
```

---

## std:path ⚡

Path manipulation module (Node.js-style).

### Import

```flowlang
circle path from "std:path"
```

### Properties

| Property | Description |
|----------|-------------|
| `path.sep` | Platform path separator (`\` on Windows, `/` on Unix) |

### Functions

#### `join(...parts) -> Silk`
Join path segments with platform separator.

```flowlang
let p = path.join("home", "user", "docs")  -- "home/user/docs"
```

#### `dirname(path: Silk) -> Silk`
Get directory name from path.

```flowlang
let dir = path.dirname("/home/user/file.txt")  -- "/home/user"
```

#### `basename(path: Silk, ext?: Silk) -> Silk`
Get base name from path, optionally removing extension.

```flowlang
let name = path.basename("/home/user/file.txt")         -- "file.txt"
let noExt = path.basename("/home/user/file.txt", ".txt") -- "file"
```

#### `extname(path: Silk) -> Silk`
Get file extension including the dot.

```flowlang
let ext = path.extname("myfile.json")  -- ".json"
```

#### `parse(path: Silk) -> Relic`
Parse path into components.

```flowlang
let p = path.parse("/home/user/file.txt")
-- p.root = "/"
-- p.dir  = "/home/user"
-- p.base = "file.txt"
-- p.ext  = ".txt"
-- p.name = "file"
```

#### `format(pathObject: Relic) -> Silk`
Format path object back to string.

```flowlang
let p = path.format({"dir": "/home/user", "base": "file.txt"})
-- "/home/user/file.txt"
```

#### `resolve(...paths) -> Silk`
Resolve path segments to absolute path.

```flowlang
let abs = path.resolve("src", "lib", "utils.js")
-- Returns absolute path from CWD
```

#### `normalize(path: Silk) -> Silk`
Normalize a path, resolving '..' and '.'.

```flowlang
let n = path.normalize("/home/user/../user/./file.txt")
-- "/home/user/file.txt"
```

#### `isAbsolute(path: Silk) -> Pulse`
Check if path is absolute.

```flowlang
path.isAbsolute("/home/user")  -- both!
path.isAbsolute("./src")       -- none!
```

#### `relative(from: Silk, to: Silk) -> Silk`
Get relative path from one path to another.

```flowlang
let rel = path.relative("/home/user", "/home/user/docs/file.txt")
-- "docs/file.txt"
```

### Example

```flowlang
circle path from "std:path"

let filename = "README.md"
let fullPath = path.join("docs", filename)

shout("Full path: " + fullPath)
shout("Directory: " + path.dirname(fullPath))
shout("Extension: " + path.extname(fullPath))
shout("Separator: " + path.sep)
```
