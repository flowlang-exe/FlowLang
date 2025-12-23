# std:math

Mathematical operations and constants.

## Import

```flowlang
circle math from std:math
```

## Constants
- `PI`: 3.14159...
- `E`: 2.718...

## Functions

### `sin(x: Ember) -> Ember`
Calculate sine of x (in radians).

```flowlang
let result = math.sin(1.5708)  -- π/2 ≈ 1.0
```

### `cos(x: Ember) -> Ember`
Calculate cosine of x (in radians).

```flowlang
let result = math.cos(0)  -- 1.0
```

### `tan(x: Ember) -> Ember`
Calculate tangent of x (in radians).

```flowlang
let result = math.tan(0.785)
```

### `sqrt(x: Ember) -> Ember`
Calculate square root of x.

```flowlang
let result = math.sqrt(16)  -- 4.0
```

### `abs(x: Ember) -> Ember`
Calculate absolute value of x.

```flowlang
let result = math.abs(-42)  -- 42.0
```

### `round(x: Ember) -> Ember`
Round x to nearest integer.

```flowlang
let result = math.round(3.7)  -- 4.0
```

### `floor(x: Ember) -> Ember`
Round x down to integer.

```flowlang
let result = math.floor(3.7)  -- 3.0
```

### `ceil(x: Ember) -> Ember`
Round x up to integer.

```flowlang
let result = math.ceil(3.2)  -- 4.0
```

### `min(a: Ember, b: Ember) -> Ember`
Get the smaller of two numbers.

```flowlang
let m = math.min(10, 5) -- 5
```

### `max(a: Ember, b: Ember) -> Ember`
Get the larger of two numbers.

```flowlang
let m = math.max(10, 5) -- 10
```

### `pow(base: Ember, exp: Ember) -> Ember`
Calculate base raised to exponent.

```flowlang
let result = math.pow(2, 8)  -- 256.0
```
