---

# 🌌 **FLOWLANG — LANGUAGE SPECIFICATION (v1.0)**

*A mystical scripting language forged for dramatic energy control, now with Anime Arc v4 features.*

---

# 1. 🧬 **CORE DESIGN PHILOSOPHY**

FlowLang is built on three fundamental principles:

1. **Expressiveness** — Code should feel like casting magic.
2. **Flow** — Control structures represent spiritual motion: *Stances, Phases, Auras*.
3. **Cinematic Errors** — Error messages are written like anime dialogue.
4. **Mystical Modularity** — Code can summon external powers via **Summon Circles**.
5. **Ritualized Asynchronous Behavior** — Async code is written as **Rituals**.
6. **Combo Chains** — Chain operations like magical combos.

FlowLang is dynamically typed with optional **Essences**.

---

# 2. 🔮 **SYNTAX OVERVIEW**

FlowLang uses lightweight, flow-centric syntax:

* Statements end automatically (no semicolons).
* Blocks use `{ ... }`.
* Identifiers are Unicode-friendly (Japanese allowed).
* Operators are replaced with runes.
* Imports, rituals, and sigils are integrated.

Example:

```
circle core
circle battle as 🔥

@experimental
cast Spell greet(Silk name) -> Hollow {
    whisper("Greetings, " + name)
}
```

---

# 3. ✨ **TYPE SYSTEM (Essences)**

## 3.1 **Primitive Essence Types**

| Essence    | Meaning            |
| ---------- | ------------------ |
| **Ember**  | numbers            |
| **Silk**   | strings            |
| **Pulse**  | boolean true/false |
| **Flux**   | any type           |
| **Hollow** | void return        |

## 3.2 **Complex Types**

| Essence              | Meaning       |
| -------------------- | ------------- |
| **Constellation<T>** | arrays        |
| **Relic<K,V>**       | objects/maps  |
| **Spell**            | function type |

Examples:

```
let x: Ember = 108
let message: Silk = "Flow begins."
let items: Constellation<Silk> = ["a", "b", "c"]
```

---

# 4. ⚔️ **OPERATORS**

### 4.1 Mathematical

| Meaning  | Operator |
| -------- | -------- |
| add      | **+**    |
| subtract | **-**    |
| multiply | `*`      |
| divide   | **/**    |
| modulo   | **%**    |

### 4.2 Comparison

| Meaning   | Operator |
| --------- | -------- |
| equal     | **is~**  |
| not equal | **not~** |
| greater   | **>>**    |
| less      | **<<**    |

### 4.3 Logical

| Meaning | Operator    |
| ------- | ----------- |
| and     | **both!**   |
| or      | **either!** |
| not     | **negate!** |

Example:

```
if (power ≫ 9000 both! mode is~ "berserk") { ... }
```

---

# 5. 🌀 **CONTROL FLOW SYSTEM**

## 5.1 **Stances (Conditionals)**

```
in Stance (power ≫ 5000) {
    shout("Strong aura detected.")
}
shift Stance (power ≫ 9000) {
    roar("UNSTOPPABLE!")
}
abandon Stance {
    whisper("Not enough energy.")
}
```

## 5.2 **Auras (Pattern/Alias Switching)**

```
invoke Aura mood {
    when "calm" -> whisper("The waters are still.")
    when "angry" -> shout("THE STORM AWAKENS!")
    otherwise -> whisper("Unknown emotional signature.")
}
```

## 5.3 **Phases (Loops)**

### Phase-count (for-loop)

```
enter Phase i from 0 to 5 {
    chant("Step " + i)
}
```

### Phase-until (while-loop)

```
enter Phase until (enemy.defeated) {
    strike(enemy)
}
```

### Phase-forever (infinite)

```
enter Phase forever {
    drift()
}
```

---

# 6. ✨ **SPELLS (Functions)**

### 6.1 Spell Declaration

```
cast Spell ignite(Ember amount, Silk target) -> Hollow {
    shout(target + " burns with " + amount + " Embers!")
}
```

### 6.2 Returning Values

```
cast Spell multiply(Ember a, Ember b) -> Ember {
    return a * b
}
```

---

# 7. 📦 **VARIABLES**

| Keyword  | Meaning           |
| -------- | ----------------- |
| **let**  | mutable essence   |
| **seal** | immutable essence |

Examples:

```
let count = 10
seal name = "Kaito"
```

Optional typing:

```
let heat: Ember = 88
```

---

# 8. 🔮 **SUMMON CIRCLES (MODULE IMPORTS)**

```
circle core
circle battle from "battle.flow"
circle ui from "./ui"
circle math as Σ
```

Use:

```
let result = Σ.summon(5, 10)
```

---

# 9. 🏷️ **SIGILS (METADATA TAGS)**

Attach metadata to spells or variables.

```
@internal
@experimental
@spiritBound
```

Example:

```
@experimental
cast Spell slash(target: Hero) -> Ember {
    return 20
}
```

---

# 10. 🔮 **RITUALS (ASYNC TASKS)**

Async behavior uses **rituals**.

```
ritual loadData ::
    wait 3s
    return "done"
end
```

Call ritual:

```
await loadData()
```

Parallel rituals:

```
perform loadData(), fetchUser(), sync()
```

---

# 11. ✨ **COMBO CHAINS (PIPEFLOW)**

Chain operations like magical combos.

```
hero >> buff >> strike(beast) >> recover !!
nums >> map n => n*2
     >> filter n => n>10
     >> reduce (a,b) => a+b
     !!
```

---

# 12. 📝 **COMMENTS**

```
-- Single-line comment
/* Multi-line
   comment block */
```

---

# 13. 🔥 **ERROR SYSTEM (Anime Dialogue Mode)**

* **Syntax Error:**
  *"Your rune sequence… it’s broken! Reform the pattern, before the flow collapses!"*

* **Type Error:**
  *"You try to bind a Silk to an Ember… They do not resonate."*

* **Undefined Variable:**
  *"You speak a name foreign to this realm… No essence responds."*

* **Out of Range:**
  *"You reach beyond the stars of this Constellation… Only void awaits you there."*

* **Division by Zero:**
  *"STOP! To divide by the Hollow is to tear reality apart!"*

---

# 14. 🛠️ **SAMPLE PROGRAM**

```
circle core
circle battle as 🔥

@experimental
cast Spell main() -> Hollow {

    let power: Ember = 9200

    in Stance (power ≫ 9000) {
        roar("It’s… OVER 9000!!!")
    }

    enter Phase i from 1 to 3 {
        chant("Charging… " + i)
    }

    let mood: Silk = "angry"

    invoke Aura mood {
        when "calm" -> whisper("Peaceful aura detected.")
        when "angry" -> shout("Hostile spirit unleashed!")
        otherwise -> whisper("Mystery emotion…")
    }

    seal result = multiply(8, 9)
    shout("Result: " + result)
}

cast Spell multiply(Ember a, Ember b) -> Ember {
    return a * b
}

ritual saveResult ::
    wait 1s
    return "Saved"
end

await saveResult()
```

---
