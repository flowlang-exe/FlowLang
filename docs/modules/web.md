# std:web ⚡

HTTP server request/response handling. Server handles keep the process alive automatically.

## Import

```flowlang
circle web from "std:web"
```

## Functions

### `serve(port: Ember, handler: Spell) -> Handle`
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

### Response Object (`res`)

The handler receives a response object with these methods:

#### Content Methods
- `res.json(data)`: Send JSON response (auto sets Content-Type)
- `res.html(content)`: Send HTML response
- `res.text(content)`: Send plain text response
- `res.send(data)`: Auto-detect type and send
- `res.file(path)`: Serve file from disk

#### Status Code Methods
- `res.ok(msg?)`: 200 OK
- `res.created(data?)`: 201 Created
- `res.noContent()`: 204 No Content
- `res.badRequest(msg?)`: 400 Bad Request
- `res.unauthorized(msg?)`: 401 Unauthorized
- `res.forbidden(msg?)`: 403 Forbidden
- `res.notFound(msg?)`: 404 Not Found
- `res.serverError(msg?)`: 500 Internal Server Error
- `res.status(code, body?)`: Custom status code

#### Other Methods
- `res.redirect(url)`: 302 Redirect
- `res.header(name, value)`: Set custom header

```flowlang
-- JSON response
return res.json({"name": "Goku", "power": 9001})

-- HTML response
return res.html("<h1>Hello World</h1>")

-- Serve file
return res.file("./public/index.html")
```
