# std:array

Array manipulation functions.

## Import

```flowlang
circle array from std:array
```

## Functions

### `len(arr: Constellation) -> Ember`
Get length of array.

```flowlang
let size = array.len([1, 2, 3])  -- 3.0
```

### `push(arr: Constellation, value: any) -> Constellation`
Return new array with element added to end.

```flowlang
let newArr = array.push([1, 2], 3)  -- [1, 2, 3]
```

### `pop(arr: Constellation) -> Constellation`
Return new array without the last element.

```flowlang
let newArr = array.pop([1, 2, 3])  -- [1, 2]
```

### `contains(arr: Constellation, value: any) -> Pulse`
Check if array contains value.

```flowlang
let found = array.contains([1, 2, 3], 2)  -- both!
```

### `join(arr: Constellation, delimiter: Silk) -> Silk`
Join array elements into string.

```flowlang
let str = array.join(["a", "b", "c"], ", ")  -- "a, b, c"
```
