# std:os

Operating system information and environment.

## Import

```flowlang
circle os from "std:os"
```

## Functions

### `name() -> Silk`
Get OS name (windows, linux, macos).

```flowlang
let osName = os.name()
shout("Running on: " + osName)
```

### `arch() -> Silk`
Get system architecture (x86_64, aarch64, etc.).

```flowlang
let arch = os.arch()
```

### `family() -> Silk`
Get OS family (windows, unix).

```flowlang
let family = os.family()
```

### `version() -> Silk`
Get OS version.

```flowlang
let version = os.version()
```

### `env(name: Silk) -> Silk | Hollow`
Get environment variable.

```flowlang
let path = os.env("PATH")
let home = os.env("HOME")
```

### `set_env(name: Silk, value: Silk) -> Hollow`
Set environment variable.

```flowlang
os.set_env("MY_VAR", "value")
```

### `cwd() -> Silk`
Get current working directory.

```flowlang
let dir = os.cwd()
shout("Working in: " + dir)
```

### `home_dir() -> Silk | Hollow`
Get user's home directory.

```flowlang
let home = os.home_dir()
```

### `pid() -> Ember`
Get current process ID.

```flowlang
let processId = os.pid()
```
