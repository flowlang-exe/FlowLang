# std:cli

Command-line interface interactions.

## Import

```flowlang
circle cli from "std:cli"
```

## Functions

### `input(prompt: Silk) -> Silk`
Read user input from terminal.

```flowlang
let name = cli.input("Enter your name: ")
shout("Hello, " + name)
```

### `args() -> Constellation<Silk>`
Get command-line arguments passed to script.

```flowlang
let args = cli.args()
-- Run with: flowlang run script.flow arg1 arg2
```

### `confirm(prompt: Silk) -> Pulse`
Ask yes/no question.

```flowlang
let proceed = cli.confirm("Continue? (y/n): ")
in Stance (proceed) {
    shout("Proceeding...")
}
```

### `select(prompt: Silk, options: Constellation<Silk>) -> Silk`
Display menu and get user selection.

```flowlang
let choice = cli.select("Choose:", ["Option 1", "Option 2", "Option 3"])
shout("You selected: " + choice)
```

### `clear() -> Hollow`
Clear the terminal screen.

```flowlang
cli.clear()
```

### `exit(code: Ember) -> Hollow`
Exit program with status code.

```flowlang
cli.exit(0)  -- Success
cli.exit(1)  -- Error
```
