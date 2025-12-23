# std:string

String manipulation functions.

## Import

```flowlang
circle string from std:string
```

## Functions

### `len(s: Silk) -> Ember`
Get length of string.

```flowlang
let length = string.len("Hello")  -- 5.0
```

### `upper(s: Silk) -> Silk`
Convert string to uppercase.

```flowlang
let result = string.upper("hello")  -- "HELLO"
```

### `lower(s: Silk) -> Silk`
Convert string to lowercase.

```flowlang
let result = string.lower("WORLD")  -- "world"
```

### `trim(s: Silk) -> Silk`
Remove whitespace from both ends.

```flowlang
let result = string.trim("  hello  ")  -- "hello"
```

### `contains(s: Silk, sub: Silk) -> Pulse`
Check if string contains substring.

```flowlang
let has = string.contains("Hello World", "World") -- both!
```

### `split(s: Silk, delimiter: Silk) -> Constellation`
Split string by delimiter into array.

```flowlang
let parts = string.split("a,b,c", ",")  -- ["a", "b", "c"]
```

### `substring(s: Silk, start: Ember, end: Ember) -> Silk`
Extract substring from start to end index.

```flowlang
let sub = string.substring("Hello", 1, 4)  -- "ell"
```
