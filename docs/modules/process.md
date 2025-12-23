# std:process ⚡

Process and command execution.

## Import

```flowlang
circle proc from "std:process"
```

## Functions

### `exec(command: Silk) -> Silk`
Execute a shell command and return stdout. Throws on error.

```flowlang
let output = proc.exec("ls -la")
shout(output)
```

### `run(program: Silk, args?: Constellation<Silk>) -> Ember`
Run a program connected to stdout/stderr. Returns exit code.

```flowlang
let code = proc.run("git", ["status"])
```

### `output(program: Silk, args?: Constellation<Silk>) -> Relic`
Run a program and capture output.

```flowlang
let result = proc.output("git", ["status"])
shout(result.stdout)
shout(result.stderr)
shout(result.code)   -- Exit code
shout(result.success) -- both! if code is 0
```
