# FlowLang Examples

Practical examples demonstrating FlowLang features and patterns.

## Table of Contents

- [Hello World](#hello-world)
- [Variables and Types](#variables-and-types)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Loops](#loops)
- [Error Handling](#error-handling)
- [Async/Await](#asyncawait)
- [Modules](#modules)
- [Standard Library](#standard-library)
- [Complete Applications](#complete-applications)

## Hello World

```flowlang
shout("Hello, World!")
```

Run:
```bash
flowlang run hello.flow
```

## Variables and Types

### Basic Variables

```flowlang
-- Mutable variable
let name = "Naruto"
let age = 17
let isNinja = both!

-- Immutable constant
seal MAX_CHAKRA = 10000

-- Type annotations
let power: Ember = 9000
let title: Silk = "Hokage"
```

### Arrays and Objects

```flowlang
-- Array (Constellation)
let numbers = [1, 2, 3, 4, 5]
let names = ["Goku", "Vegeta", "Gohan"]

-- Access elements
shout(numbers[0])  -- 1
shout(names[2])    -- "Gohan"

-- Array methods
let length = numbers.len()
shout(length)  -- 5

let newArr = numbers.push(6)
shout(newArr)  -- [1, 2, 3, 4, 5, 6]

let sliced = numbers.slice(1, 4)
shout(sliced)  -- [2, 3, 4]

-- Method chaining
let result = [1, 2, 3].push(4).push(5)
shout(result)  -- [1, 2, 3, 4, 5]

-- Nested arrays
let matrix = [[1, 2], [3, 4]]

-- Relic (Map)
let user = {
    "name": "Goku",
    "power": 9000,
    "saiyan": both!
}

shout(user["name"]) -- "Goku"
```

### String Interpolation

```flowlang
-- Template literals with backticks
let name = "Naruto"
let age = 17
let message = `${name} is ${age} years old`
shout(message)  -- "Naruto is 17 years old"

-- Expressions in interpolation
let power = 9000
let status = `Power level: ${power + 200}`
shout(status)  -- "Power level: 9200"

-- Double quotes (no interpolation)
let literal = "Hello ${name}"  -- Literal text
shout(literal)  -- "Hello ${name}"

-- Single quotes (no interpolation)
let literal2 = 'Value: ${age}'  -- Literal text
shout(literal2)  -- "Value: ${age}"
```

## Control Flow

### If-Else

```flowlang
let power = 9200

in Stance (power >> 9000) {
    roar("IT'S OVER 9000!")
} otherwise {
    shout("Power level normal")
}
```

### If-Else If-Else

```flowlang
let mood = "excited"

in Stance (mood == "happy") {
    whisper("😊")
} shift Stance (mood == "excited") {
    roar("🎉")
} shift Stance (mood == "sad") {
    whisper("😢")
} otherwise {
    shout("😐")
}
```

### Switch Statement

```flowlang
let command = "attack"

invoke Aura command {
    when "attack" -> shout("⚔️  Attacking!")
    when "defend" -> shout("🛡️  Defending!")
    when "heal" -> shout("💚 Healing!")
    otherwise -> shout("Unknown command")
}
```

## Functions

### Simple Function

```flowlang
cast Spell greet(name: Silk) -> Silk {
    return `Hello, ${name}!`  -- Using template literal
}

let message = greet("Luffy")
shout(message)  -- "Hello, Luffy!"
```

### Multiple Parameters

```flowlang
cast Spell add(a: Ember, b: Ember) -> Ember {
    return a + b
}

let sum = add(5, 10)
shout(`Sum: ${sum}`)  -- "Sum: 15"
```

### Void Return

```flowlang
cast Spell logMessage(msg: Silk) -> Hollow {
    shout("[LOG] " + msg)
}

logMessage("Application started")
```

### Recursive Function

```flowlang
cast Spell factorial(n: Ember) -> Ember {
    in Stance (n <<= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

shout(factorial(5))  -- 120
```

### Inline Spells

```flowlang
-- Single expression
let double = cast Spell x -> x * 2
shout(double(10))  -- 20

-- Multiple parameters
let add = cast Spell (a, b) -> a + b
shout(add(5, 3))  -- 8

-- Block syntax
let greet = cast Spell (name) {
    return "Hello, " + name
}
shout(greet("World"))
```

## Loops

### Count Loop (For)

```flowlang
enter Phase i from 1 to 5 {
    shout("Count: " + i)
}
```

### While Loop

```flowlang
let count = 0

enter Phase until (count >> 5) {
    shout("Count: " + count)
    count = count + 1
}
```

### Infinite Loop with Break

```flowlang
let i = 0

enter Phase forever {
    in Stance (i >> 10) {
        break seal
    }
    shout(i)
    i = i + 1
}
```

### Loop with Continue

```flowlang
enter Phase i from 1 to 10 {
    in Stance (i == 5) {
        fracture seal  -- Skip 5
    }
    shout(i)
}
```

## Array Methods

### Array Length

```flowlang
let numbers = [10, 20, 30, 40, 50]
let count = numbers.len()
shout(`Array has ${count} elements`)  -- "Array has 5 elements"
```

### Adding Elements

```flowlang
let arr = [1, 2, 3]
let newArr = arr.push(4)
shout(`Original: ${arr}`)    -- [1, 2, 3]
shout(`New: ${newArr}`)       -- [1, 2, 3, 4]

-- Chain multiple pushes
let result = arr.push(4).push(5).push(6)
shout(result)  -- [1, 2, 3, 4, 5, 6]
```

### Removing Elements

```flowlang
let stack = [10, 20, 30]
let last = stack.pop()
shout(`Popped: ${last}`)      -- "Popped: 30"
shout(`Remaining: ${stack}`)  -- "Remaining: [10, 20]"
```

### Slicing Arrays

```flowlang
let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
let subset = numbers.slice(2, 7)
shout(subset)  -- [2, 3, 4, 5, 6]

-- Get first 3 elements
let first3 = numbers.slice(0, 3)
shout(first3)  -- [0, 1, 2]
```

### Complex Array Operations

```flowlang
-- Build array with method chaining
let data = [1, 2, 3]
    .push(4)
    .push(5)
    .slice(1, 4)
shout(data)  -- [2, 3, 4]

-- Process array in loop
let scores = [85, 92, 78, 95, 88]
enter Phase i from 0 to scores.len() - 1 {
    let score = scores[i]
    shout(`Score ${i + 1}: ${score}`)
}
```

## Error Handling

### Basic Try-Catch

```flowlang
attempt {
    rupture Rift "Connection failed"
} rescue Rift as e {
    shout("Error: " + e)
}
```

### Multiple Error Types

```flowlang
circle json from "std:json"

attempt {
    let data = json.parse(invalidJson)
} rescue Glitch as e {
    shout("Parse error: " + e)
} rescue as e {
    shout("Unknown error: " + e)
}
```

### Finally Block

```flowlang
attempt {
    let file = openFile("data.txt")
    processData(file)
} rescue as e {
    shout("Error: " + e)
} finally {
    closeFile(file)
    shout("Cleanup done")
}
```

### Error Containment

```flowlang
ward {
    rupture VoidTear "Optional operation failed"
}
shout("Program continues")
```

## Async/Await

### Simple Ritual

```flowlang
ritual loadData ::
    shout("Loading...")
    wait 2s
    shout("Done!")
    return "data"
end

cast Spell main() -> Hollow {
    let result = await loadData()
    shout("Got: " + result)
}

main()
```

### Parallel Execution

```flowlang
ritual task1 ::
    wait 1s
    return "Task 1 complete"
end

ritual task2 ::
    wait 1s
    return "Task 2 complete"
end

cast Spell main() -> Hollow {
    shout("Starting parallel tasks...")
    perform task1(), task2()
    shout("All done!")
}

main()
```

## Modules

    
    let cubed = math.cube(3)
    shout("Cube: " + cubed)     -- 27
    
    -- Cannot access private members
    -- math.validateNumber(5)   -- Error: undefined
    -- math.INTERNAL_PRECISION  -- Error: undefined

utils.increment()
utils.increment()
shout(utils.getCount())  -- 2

utils.reset()
shout(utils.getCount())  -- 0

-- Private members are hidden
-- utils.counter           -- Error: undefined
-- utils.incrementInternal() -- Error: undefined
```

## Standard Library

### File I/O

```flowlang
circle fs from "std:file"

-- Write file
fs.write("output.txt", "Hello, FlowLang!")

-- Read file
attempt {
    let content = fs.read("output.txt")
    shout(content)
} rescue as e {
    shout("Error reading file: " + e)
}
```

### JSON Processing

```flowlang
circle json from "std:json"

-- Create object
let user = {
    "name": "Goku",
    "level": 9000,
    "rank": "Saiyan"
}

-- Convert to JSON
let jsonStr = json.stringify(user)
shout(jsonStr)

-- Parse JSON
let parsed = json.parse(jsonStr)
shout("Name: " + parsed["name"])
```

### HTTP Requests

```flowlang
circle net from "std:net"
circle json from "std:json"

attempt {
    let response = net.get("https://api.github.com/users/octocat")
    let data = json.parse(response)
    shout("Username: " + data["login"])
} rescue Rift as e {
    shout("Network error: " + e)
}
```

## Complete Applications

### Calculator

```flowlang
cast Spell add(a: Ember, b: Ember) -> Ember { return a + b }
cast Spell subtract(a: Ember, b: Ember) -> Ember { return a - b }
cast Spell multiply(a: Ember, b: Ember) -> Ember { return a * b }
cast Spell divide(a: Ember, b: Ember) -> Ember {
    in Stance (b == 0) {
        rupture Rift "Division by zero!"
    }
    return a / b
}

cast Spell calculate(op: Silk, a: Ember, b: Ember) -> Ember {
    invoke Aura op {
        when "add" -> return add(a, b)
        when "sub" -> return subtract(a, b)
        when "mul" -> return multiply(a, b)
        when "div" -> return divide(a, b)
        otherwise -> rupture Spirit "Unknown operation: " + op
    }
}

cast Spell main() -> Hollow {
    attempt {
        shout("10 + 5 = " + calculate("add", 10, 5))
        shout("10 * 5 = " + calculate("mul", 10, 5))
        shout("10 / 0 = " + calculate("div", 10, 0))
    } rescue as e {
        shout("Error: " + e)
    }
}

main()
```

### To-Do List API

```flowlang
circle net from "std:net"
circle json from "std:json"
circle fs from "std:file"

seal API_URL = "https://jsonplaceholder.typicode.com/todos"

ritual fetchTodos ::
    attempt {
        let response = net.get(API_URL)
        return json.parse(response)
    } rescue Rift as e retry 3 {
        shout("Retrying...")
        wait 1s
    }
end

ritual saveTodos(todos: Constellation) ::
    let jsonStr = json.stringify(todos)
    fs.write("todos.json", jsonStr)
    shout("Saved to todos.json")
end

cast Spell main() -> Hollow {
    attempt {
        shout("Fetching todos...")
        let todos = await fetchTodos()
        
        let count = todos.len()
        shout(`Total todos: ${count}`)
        
        -- Show first 5
        enter Phase i from 0 to 4 {
            let todo = todos[i]
            shout(todo["title"])
        }
        
        await saveTodos(todos)
        
    } rescue as e {
        shout(`Failed: ${e}`)
    }
}

main()
```

### Web Scraper with Cache

```flowlang
circle net from "std:net"
circle fs from "std:file"
circle json from "std:json"

cast Spell getCachedOrFetch(url: Silk) -> Silk {
    let cacheFile = "cache.json"
    
    -- Try cache first
    ward {
        in Stance (fs.exists(cacheFile)) {
            let cached = fs.read(cacheFile)
            shout("Using cache")
            return cached
        }
    }
    
    -- Fetch if not in cache
    attempt {
        shout("Fetching from: " + url)
        let data = net.get(url)
        fs.write(cacheFile, data)
        return data
        
    } rescue Rift as e {
        shout("Network error: " + e)
        rebound
    }
}

cast Spell main() -> Hollow {
    attempt {
        let data = getCachedOrFetch("https://api.github.com/users/octocat")
        let user = json.parse(data)
        shout("User: " + user["name"])
        
    } rescue as e {
        shout("Error: " + e)
    }
}

main()
```

## Tips and Patterns

### Error Handling Pattern

```flowlang
cast Spell riskyOperation() -> Silk {
    attempt {
        -- Try operation
        return performOperation()
    } rescue SpecificError as e {
        -- Handle specific error
        logError(e)
        return fallbackValue()
    } finally {
        -- Always cleanup
        cleanup()
    }
}
```

### Null Checking Pattern

```flowlang
cast Spell processValue(value: any) -> Hollow {
    in Stance (value == void) {
        rupture VoidTear "Value cannot be null"
    }
    -- Process value
}
```

### Loop with Early Exit

```flowlang
cast Spell findItem(items: Constellation, target: Silk) -> Ember {
    enter Phase i from 0 to items.len() - 1 {
        in Stance (items[i] == target) {
            return i
        }
    }
    return -1  -- Not found
}
```
