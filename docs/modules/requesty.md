# std:requesty

Advanced HTTP client for making requests (Sync/Blocking).

## Import

```flowlang
circle requesty from "std:requesty"
```

## Functions

All functions return a `Response` object (Relic) or throw an error.

### `request(url: Silk, options?: Relic) -> Relic`
Universal request function.

```flowlang
let res = requesty.request("https://api.example.com/data", {
    "method": "POST",
    "headers": {"Authorization": "Bearer token"},
    "json": {"key": "value"},
    "timeout": 5000
})
```

### Shortcuts

- `get(url: Silk, options?: Relic)`
- `post(url: Silk, body?: Silk|Relic, options?: Relic)`
- `put(url: Silk, body?: Silk|Relic, options?: Relic)`
- `delete(url: Silk, options?: Relic)`
- `patch(url: Silk, body?: Silk|Relic, options?: Relic)`
- `head(url: Silk, options?: Relic)`
- `options(url: Silk, options?: Relic)`

**Note**: For `post`, `put`, `patch`, the second argument can be a generic options object (if it contains `body` or `json` keys) OR a direct body string.

```flowlang
-- Quick POST with string body
requesty.post("https://api.com/post", "raw body data")

-- Quick POST with JSON (using options)
requesty.post("https://api.com/post", {
    "json": {"foo": "bar"}
})
```

### Options Object

| Property | Type | Description |
|----------|------|-------------|
| `method` | Silk | HTTP Method (GET, POST, etc.) |
| `headers` | Relic | Request headers `{"Key": "Value"}` |
| `body` | Silk | Raw request body string |
| `json` | Flux | Object to serialize as JSON body (sets Content-Type automatically) |
| `timeout` | Ember | Request timeout in milliseconds |

### Response Object

The returned Relic contains:

| Property | Type | Description |
|----------|------|-------------|
| `status` | Ember | HTTP Status Code (e.g., 200, 404) |
| `statusText` | Silk | Status text (e.g., "OK", "Not Found") |
| `headers` | Relic | Response headers |
| `text` | Silk | Raw response body as string |
| `json` | Flux | Parsed JSON body (Hollow if parsing failed) |

### Example

```flowlang
attempt {
    let res = requesty.get("https://jsonplaceholder.typicode.com/todos/1")
    
    shout("Status: " + res.status)
    shout("Type: " + res.headers["content-type"])
    
    -- Auto-parsed JSON
    if (res.json != hollow) {
        shout("Title: " + res.json["title"])
    }
} rescue Rift as e {
    shout("Request failed: " + e)
}
```
