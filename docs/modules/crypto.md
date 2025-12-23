# std:crypto

Cryptography functions for hashing and encoding.

## Import

```flowlang
circle crypto from "std:crypto"
```

## Functions

### `md5(text: Silk) -> Silk`
Calculate MD5 hash.

```flowlang
let hash = crypto.md5("password")
-- Returns: 5f4dcc3b5aa765d61d8327deb882cf99
```

### `sha256(text: Silk) -> Silk`
Calculate SHA256 hash.

```flowlang
let hash = crypto.sha256("secret")
-- Returns: 2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b
```

### `sha512(text: Silk) -> Silk`
Calculate SHA512 hash.

```flowlang
let hash = crypto.sha512("data")
```

### `base64_encode(text: Silk) -> Silk`
Encode text to Base64.

```flowlang
let encoded = crypto.base64_encode("Hello")
-- Returns: SGVsbG8=
```

### `base64_decode(encoded: Silk) -> Silk`
Decode Base64 text.

```flowlang
let decoded = crypto.base64_decode("SGVsbG8=")
-- Returns: Hello
```

### `hex_encode(text: Silk) -> Silk`
Encode text to hexadecimal.

```flowlang
let hex = crypto.hex_encode("ABC")
-- Returns: 414243
```

### `hex_decode(hex: Silk) -> Silk`
Decode hexadecimal to text.

```flowlang
let text = crypto.hex_decode("414243")
-- Returns: ABC
```
