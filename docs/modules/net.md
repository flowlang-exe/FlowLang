# std:net

Network module for making HTTP requests (Sync/Blocking).

## Import

```flowlang
circle net from "std:net"
```

## Functions

### `get(url: Silk) -> Silk`
Make HTTP GET request and return body text.

```flowlang
attempt {
    let response = net.get("https://api.example.com/data")
    let data = json.parse(response)
} rescue Rift as e {
    shout("Request failed: " + e)
}
```

### `post(url: Silk, body: Silk) -> Silk`
Make HTTP POST request.

```flowlang
let payload = json.stringify({"key": "value"})
let response = net.post("https://api.example.com/create", payload)
```

### `put(url: Silk, body: Silk) -> Silk`
Make HTTP PUT request.

```flowlang
let response = net.put("https://api.example.com/update/1", payload)
```

### `patch(url: Silk, body: Silk) -> Silk`
Make HTTP PATCH request.

```flowlang
let response = net.patch("https://api.example.com/update/1", payload)
```

### `delete(url: Silk) -> Silk`
Make HTTP DELETE request.

```flowlang
net.delete("https://api.example.com/item/1")
```

### `head(url: Silk) -> Silk`
Make HTTP HEAD request. Returns status line.

```flowlang
let status = net.head("https://example.com")
```

### `request(method: Silk, url: Silk, body?: Silk) -> Silk`
Make a generic HTTP request.

```flowlang
let res = net.request("OPTIONS", "https://example.com/api", "")
```
