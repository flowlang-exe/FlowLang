# 🌌 **FlowLang Essence System**

*— where your variables have souls, and your mistakes have consequences.*

FlowLang doesn’t use boring “types.”
No.
We use **Essences** — the spiritual form your data takes before it inevitably disappoints you.

You get dynamic typing by default, because FlowLang believes in *freedom*.
But if you enable **Strict Mode**, FlowLang will immediately stop believing in you.

---

## 🌟 **The Essences**

Every value carries an **Essence**, which defines what it *is*…
(or what it *tries* to be).

| Essence              | Meaning                              | Rust Vibes      | Example                                |
| -------------------- | ------------------------------------ | --------------- | -------------------------------------- |
| **Ember**            | Numbers (hot, volatile)              | `f64`           | `42`, `3.14`, `-999`                   |
| **Silk**             | Strings (soft, smooth, gentle lies)  | `String`        | `"Flow"`, `'Lang'`, `` `Magic ${x}` `` |
| **Pulse**            | Boolean life force                   | `bool`          | `both!` (true), `either!` (false)      |
| **Flux**             | “I’ll accept anything just pls work” | `Value`         | Literally anything                     |
| **Hollow**           | Nothingness. Void. Your motivation.  | `()`            | `void`                                 |
| **Constellation<T>** | Arrays that orbit a single type      | `Vec<T>`        | `[1, 2, 3]`                            |
| **Relic<K, V>**      | Sacred key-value artifacts           | `HashMap<_, _>` | `{ "name": "Flow" }`                   |
| **Spell**            | Functions / enchantments             | `Function`      | `cast Spell foo() {}`                  |

### 🌌 Essence Compatibility Rules

* **Flux** = “yeah whatever bro”
* **Hollow** = “nothing? cool”
* **Constellation<Flux>** lets you mix whatever chaos you want
* **Relic keys must be Silk**
  (FlowLang is allergic to non-string keys)

---

## 🧪 **Type Markings (Annotations)**

If you want FlowLang to judge your data, you can **annotate** it.

### 📦 Variables

```flowlang
let count: Ember = 10
let name: Silk = "FlowLang"
let is_fixed: Pulse = both!
```

Silk also supports three moods:

```flowlang
let a: Silk = "double"
let b: Silk = 'single'
let c: Silk = `template of chaos ${name}`
```

Complex spirits:

```flowlang
let scores: Constellation<Ember> = [99, 80, 60]

let conf: Relic<Silk, Flux> = {
    "debug": both!,
    "tries": 3
}
```

---

## 🪄 **Spells (Functions)**

```flowlang
cast Spell add(Ember a, Ember b) -> Ember {
    return a + b
}
```

Void return spells:

```flowlang
cast Spell greet(Silk name) -> Hollow {
    shout("Hi " + name)
}
```

---

## ⚔️ **Runtime Essence Checking**

FlowLang checks types **only if you write them**.
(If you don’t, FlowLang assumes you’re a risk-taker.)

### ❌ Wrong Essence

```flowlang
let x: Ember = "Hi" 
-- Error: Expected Ember, found Silk. 
```

### ❌ Wrong Spell Input

```flowlang
cast Spell square(Ember n) -> Ember {
    return n * n
}

square("5") 
-- Error: Silk ≠ Ember. 
```

Even outside strict mode, FlowLang still slaps you if you lie with annotations.

---

# 🔒 **Strict Mode**

*For devs who say “safety first” and then push to prod at 3 AM.*

Enable it:

```json
{
  "project": {
    "type_required": true
  }
}
```

### In Strict Mode:

❌ No annotation? no code.
❌ No type? no vibes.

```flowlang
let x = 10 
-- Error: Strict mode says “type plz” 

let x: Ember = 10 
-- Approved 😇
```

Spells must be fully blessed:

```flowlang
cast Spell foo(a) { ... }
-- Error: Undefined essence. Heresy.

cast Spell foo(Flux a) -> Hollow { ... }
-- Accepted.
```

---

# 🏺 **Relic Crafting (Maps)**

```flowlang
let user: Relic<Silk, Flux> = {
    "name": "Alice",
    "age": 22,
    "admin": both!
}
```

Retrieve:

```flowlang
let name = user["name"]
```

---

# 🔮 **Sigils (Custom Types)**

*Define the sacred shape of your data.*

### ✨ Creating a Sigil

```flowlang
sigil Note {
    id: Silk,
    title: Silk,
    content: Silk,
    createdAt: Silk,
    hash: Silk
}
```

Another:

```flowlang
sigil ServerConf {
    port: Ember,
    file: Silk,
    debug: Pulse
}
```

### ✨ Using a Sigil

```flowlang
cast Spell createNote(Silk title, Silk content)
 -> Relic<Silk, Flux> {

    return {
        "id": id.generate(),
        "title": title,
        "content": content,
        "createdAt": time.now(),
        "hash": crypto.sha256(content)
    }
}
```

**Sigils don't enforce structure yet** —
but they act as documentation, guidance, and warnings to future you.

---

# ⭐ Final Vibe Summary

FlowLang Essences give your program a soul.
Strict mode removes your soul.
Flux accepts your chaos.
Hollow accepts your regrets.
Pulse determines whether your code even wants to live.
And Spells?
They’re the only magic holding your project together.
