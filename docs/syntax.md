# 🌌 **FLOWLANG SPELLBOOK — SYNTAX REFERENCE**

*Every line is a pulse of mana, every loop a phase of existence.*

---

## 📝 **Comments — Whispering the Arc**

```flowlang
-- Single-line incantation
/* Multi-line chant
   that echoes in the void */
```

---

## ⚡ **Variables — Essences of Reality**

### Declaration

```flowlang
-- Mutable Essence
let power = 9000

-- Immutable Artifact
seal MAX_POWER = 10000

-- Typed Essence
let name: Silk = "Goku"
let level: Ember = 42
```

**Rules of Naming the Mana:**

* Start with letter or `_`
* Can contain alphanumeric and `_`
* Unicode runes accepted (`世界`, `こんにちは`)

---

## 🪐 **Data Types — FlowLang Essences**

| Type              | Description        | Example                           |
| ----------------- | ------------------ | --------------------------------- |
| **Ember**         | Number (int/float) | `42`, `3.14`                      |
| **Silk**          | String             | `"Hello"`, `"世界"`                 |
| **Pulse**         | Boolean            | `both!` (true), `either!` (false) |
| **Flux**          | Null               | `void`                            |
| **Hollow**        | No return          | `Hollow` functions                |
| **Constellation** | Array              | `[1,2,3]`                         |
| **Relic**         | Map/Object         | `{"key":"value"}`                 |
| **Spell**         | Function           | `cast Spell name() {...}`         |

---

### Literals & Arcane Values

```flowlang
-- Numbers
let n = 42
let pi = 3.14159
let neg = -10

-- Strings
let text = "Hello, World!"       -- double quotes
let literal = 'This ${won’t} interpolate'  -- single quotes
let greeting = `Hi ${name}!`     -- template literals (interpolation)

-- Booleans
let alive = both!
let dead = either!

-- Arrays
let numbers = [1, 2, 3]
let mixed = [42, "text", both!, [1, 2]]

-- Maps
let user = {"name": "Goku", "power": 9000}

-- Null Essence
let nothing = void
```

**Note:** `seal` = immutable, `let` = mutable.

---

## 🔮 **Operators — Mana Manipulation**

### Arithmetic

```flowlang
+ - * / % 
```

### Comparison

```flowlang
>> << >>= <<= is~ not~
```

### Logic

```flowlang
both!    -- AND
either!  -- OR
negate!  -- NOT
```

### Strings

```flowlang
let s = "Flow" + "Lang"  -- concat
let msg = `Hello ${name}`  -- interpolation
```

---

## 🌀 **Control Flow — Shaping Fate**

### Conditional

```flowlang
in Stance (power >> 9000) {
    shout("OVER 9000!!")
}
```

### Multi-branch

```flowlang
in Stance (score >> 90) { shout("A") }
shift Stance (score >> 80) { shout("B") }
shift Stance (score >> 70) { shout("C") }
otherwise { shout("F") }
```

### Switch-like

```flowlang
invoke Aura mood {
    when "happy" -> whisper("😊")
    when "sad" -> whisper("😢")
    otherwise -> roar("😐")
}
```

---

## 🔁 **Loops — Enter Phase**

### Count Loop

```flowlang
enter Phase i from 1 to 10 { shout(i) }
```

### For-each (Constellation)

```flowlang
let names = ["Goku","Vegeta"]
enter Phase name in names { shout(name) }
```

### While Loop

```flowlang
enter Phase until (done) { ... }
```

### Infinite Loop

```flowlang
enter Phase forever { break seal }
```

---

## ✨ **Functions — Spells**

### Block Spell

```flowlang
cast Spell greet(name) -> Silk {
    return "Hello, " + name + "!"
}
```

### Inline Spell (Lambda)

```flowlang
let double = cast Spell x -> x * 2
let add = cast Spell (a,b) -> a + b
let hi = cast Spell () -> "Hi!"
```

### Early Return

```flowlang
cast Spell checkPower(level) -> Silk {
    in Stance (level << 0) { shatter grand_seal }
    return "Valid"
}
```

---

## 📦 **Modules — Circles of Knowledge**

```flowlang
circle math from "std:math"
circle fs from "std:file"
circle json from "std:json"
```

**Exporting members:**

```flowlang
@export cast Spell publicSpell() -> Hollow { ... }
```

---

## ⏳ **Async/Await — Rituals of Time**

```flowlang
ritual fetchData ::
    shout("Fetching...")
    wait 2s
    return "data"
end

let data = await fetchData()
```

Parallel execution:

```flowlang
perform fetchData(), processData("quick")
```

---

## 💀 **Error Handling — The ERROR ARC**

```flowlang
rupture Rift "Connection lost"

attempt { rupture Glitch "Parse fail" } rescue Glitch as e {
    shout("Caught: " + e)
}

ward { rupture VoidTear "Absorbed" }

attempt { rupture Spirit "Error" } rescue Spirit as e {
    shout("Rethrow")
    rebound e
}
```

---

## 📢 **Output Functions**

```flowlang
whisper("quiet")
shout("normal")
roar("LOUD")
chant("highlighted")
```

---

## 🧬 **Constellation Methods**

```flowlang
let nums = [1,2,3,4,5]

nums.len()        -- 5
nums.push(6)      -- [1,2,3,4,5,6]
nums.pop()        -- 6
nums.slice(1,3)   -- [2,3]
nums.concat([7,8]) -- [1,2,3,4,5,6,7,8]
nums.reverse()    -- [5,4,3,2,1]
nums.join(", ")   -- "1,2,3,4,5"
```

### Functional Methods

```flowlang
nums.constellation(cast Spell x -> x*2)  -- map
nums.filter(cast Spell x -> x %2 is~ 0) -- filter
nums.reduce(cast Spell (acc,x)->acc+x,0) -- reduce
nums.find(cast Spell x -> x>>3)          -- first match
nums.every(cast Spell x -> x>>0)         -- all?
nums.some(cast Spell x -> x>>10)         -- any?
```

Method chaining for pipelines:

```flowlang
let result = [1,2,3,4,5,6,7,8,9,10]
    .filter(cast Spell x -> x%2 is~0)
    .constellation(cast Spell x -> x*10)
    .reduce(cast Spell (acc,x)->acc+x,0)
shout(result) -- 300
```

---

## 🧪 **Complete Example**

```flowlang
cast Spell factorial(Ember n) -> Ember {
    in Stance (n <<= 1) { return 1 }
    return n * factorial(n-1)
}

cast Spell main() -> Hollow {
    enter Phase i from 1 to 5 {
        shout(i + "! = " + factorial(i))
    }
}

main()
```

With error handling:

```flowlang
cast Spell divide(Ember a,Ember b) -> Ember {
    in Stance (b == 0) { rupture Rift "Division by zero!" }
    return a/b
}

attempt {
    let res = divide(10,0)
} rescue Rift as e {
    shout("Error: " + e)
} finally {
    shout("Cleanup complete")
}
```