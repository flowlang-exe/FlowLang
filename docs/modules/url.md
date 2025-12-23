# std:url ⚡

URL parsing and manipulation (like Node.js URL module).

## Import

```flowlang
circle url from "std:url"
```

## Functions

### `parse(urlString: Silk) -> Relic`
Parse a URL into its components.

```flowlang
let parsed = url.parse("https://example.com:8080/path?name=goku")
shout(parsed.protocol)  -- "https"
shout(parsed.hostname)  -- "example.com"
shout(parsed.port)      -- 8080
shout(parsed.pathname)  -- "/path"
shout(parsed.query.name) -- "goku"
```

### `parseQuery(queryString: Silk) -> Relic`
Parse a query string into an object.

```flowlang
let params = url.parseQuery("?id=123&sort=asc")
shout(params.id)    -- "123"
shout(params.sort)  -- "asc"
```

### `encode(text: Silk) -> Silk`
URL encode a string.

```flowlang
let encoded = url.encode("hello world")  -- "hello%20world"
```

### `decode(text: Silk) -> Silk`
URL decode a string.

```flowlang
let decoded = url.decode("hello%20world")  -- "hello world"
```

### `format(urlObject: Relic) -> Silk`
Format a URL object back into a string.

```flowlang
let urlStr = url.format({
    "protocol": "https",
    "hostname": "api.example.com",
    "pathname": "/users"
})
-- "https://api.example.com/users"
```
