# std:time

Time and date operations.

## Import

```flowlang
circle time from "std:time"
```

## Functions

### `now() -> Silk`
Get current time as RFC3339 string.

```flowlang
let t = time.now()
shout("Current time: " + t) -- "2024-01-15T14:30:45+00:00"
```

### `timestamp() -> Ember`
Get current Unix timestamp (seconds since epoch).

```flowlang
let ts = time.timestamp() -- 1705329045.0
```

### `format(format: Silk) -> Silk`
Get current time formatted with strftime string.

```flowlang
let formatted = time.format("%Y-%m-%d %H:%M:%S")
shout(formatted)  -- "2024-01-15 14:30:45"
```

### `sleep(seconds: Ember) -> Hollow`
Sleep for specified seconds (blocking).

```flowlang
shout("Waiting...")
time.sleep(2.0)  -- Sleep 2 seconds
shout("Done!")
```
