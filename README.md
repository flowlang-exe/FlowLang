# 🌌 FlowLang

### *— The Mystical Scripting Language of the Ancients —*

> "Code isn't just logic... it's **Mana**. And you? You are the **Caster**."

FlowLang is an **anime-themed, fantasy-infused** programming language built in Rust. It replaces boring syntax with dramatic spells, rituals, and emotional auras.

---

## 📜 **The Grimoire**

Why write `function` when you can `cast Spell`?  
Why write `if` when you can take a `Stance`?  
Why loop when you can `enter Phase`?

FlowLang isn't just a language; it's a **roleplay**.

### ✨ **Arcane Features**

- **🧙‍♂️ Spellcasting**: Define functions with `cast Spell`.
- **⚔️ Battle Stances**: Control flow with `in Stance` (If) and `shift Stance` (Else If).
- **🌀 Auras**: Match patterns with `invoke Aura`.
- **⏳ Phases**: Loop through reality with `enter Phase`.
- **🎭 Dramatic Errors**: Errors aren't just bugs; they are *ruptures in the timeline*.
- **📦 Flora**: The... *intense* package manager who loves you a bit too much. 🌸🔪

---

## 🕯️ **Summoning Ritual (Installation)**

You need **Rust** (the metal of the gods) to forge the binary.

```bash
# Clone the sacred texts
git clone https://github.com/flowlang-exe/flowlang.git
cd flowlang

# Forge the artifact
cargo build --release

# The power is now yours...
# ./target/release/flowlang
```

---

## 🔮 **Casting Spells (Usage)**

To execute a scroll (run a script):

```bash
cargo run -- run <path-to-scroll>
```

Or invoke the binary directly:

```bash
flowlang run examples/hello.flow
```

### 🗣️ **The Incantations**

| Boring Term | FlowLang Incantation | Meaning |
|:---:|:---:|:---|
| `print` | `shout()` / `whisper()` | Project your voice into the void |
| `function` | `cast Spell` | Weave logic into reality |
| `return` | `return` | *Even magic has standards* |
| `null` | `Hollow` | The empty void |
| `string` | `Silk` | Smooth text threads |
| `number` | `Ember` | Burning numerical power |
| `bool` | `Pulse` | The heartbeat of truth |
| `array` | `Constellation` | Stars aligned in data |
| `import` | `circle ... from ...` | Summon power from other realms |

---

## 📜 **Ancient Scrolls (Examples)**

### **The First Awakening**

```flowlang
-- casting a simple spell
cast Spell main() -> Hollow {
    shout("🌌 The Mana flows through me! 🌌")
    whisper("...but I am still learning.")
}

main()
```

### **The Power Check Stance**

```flowlang
cast Spell check_power(Ember level) -> Pulse {
    shout("Scouter reading: " + level)

    in Stance (level >> 9000) {
        roar("IT'S OVER 9000!!!")
        return true
    } 
    shift Stance (level >> 5000) {
        shout("Impressive... but not enough.")
        return false
    }
    abandon Stance {
        whisper("Pathetic.")
        return false
    }
}
```

### **Invoking an Aura**

```flowlang
let element: Silk = "fire"

invoke Aura element {
    when "fire" -> roar("🔥 FLAME ON!")
    when "water" -> whisper("🌊 The tides turn...")
    otherwise -> shout("✨ Unknown energy signature.")
}
```

---

## 🌸 **Flora: Please... Notice Me Senpai...**

FlowLang comes with **Flora**, the yandere package manager.
She manages your dependencies. **Aggressively.**

```bash
flora add github.com/user/repo@main
```

> *"I'll take care of everything, senpai... You don't need anyone else."*
---

## 🤝 **Joining the Covenant**

Do you wish to contribute to the forbidden arts?
Fork the repository, weave your code, and submit a Pull Request.

**Warning**: Messy code will be purified.

---

<p align="center">
  <b>May your Mana never deplete! 🔷</b>
</p>
