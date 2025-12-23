# std:color

Terminal colors and text styling.

## Import

```flowlang
circle color from "std:color"
```

## Functions

### Basic Colors
All take `text: Silk` and return `Silk` wrapped in ANSI codes.

- `red(text)`
- `green(text)`
- `blue(text)`
- `yellow(text)`
- `magenta(text)`
- `cyan(text)`
- `white(text)`
- `black(text)`

### Bright Colors
- `bright_red(text)`
- `bright_green(text)`
- `bright_blue(text)`
- `bright_yellow(text)`
- `bright_magenta(text)`
- `bright_cyan(text)`

### Text Styles
- `bold(text)`
- `italic(text)`
- `underline(text)`
- `dimmed(text)`
- `strikethrough(text)`

### Example

```flowlang
shout(color.red("Error!"))
shout(color.green("Success!"))
shout(color.bold(color.blue("Important")))
```
