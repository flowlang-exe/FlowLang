# std:json

JSON parsing and serialization.

## Import

```flowlang
circle json from "std:json"
```

## Functions

### `parse(json: Silk) -> Flux`
Parse JSON string into a FlowLang value (Relic, Array, Silk, Ember, Pulse, or Hollow).

```flowlang
let data = json.parse('{"name": "Goku", "level": 9000}')
shout(data["name"])  -- "Goku"
```

### `stringify(value: Flux) -> Silk`
Convert a FlowLang value to JSON string.

```flowlang
let obj = {"name": "Naruto", "rank": "Hokage"}
let jsonStr = json.stringify(obj)
shout(jsonStr)  -- '{"name":"Naruto","rank":"Hokage"}'
```
