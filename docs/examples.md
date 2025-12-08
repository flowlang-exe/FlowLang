# 🌟 **FLOWLANG EXAMPLES — ARCANE EDITION**

### *“If your code breaks, it’s not a bug. It’s a you problem.”* 😏🔥

---

## 🌕 **Hello World — The Ritual of Beginnings**

Even ancient sorcerers start somewhere. Usually with disappointment, but here we shout into the void:

```flowlang
shout("Hello, World!")
```

Run it like you're casting your first spell:

```bash
flowlang run hello.flow
```

**🗡️ Note:**
Congrats, you printed a string. You're basically a senior developer now.

---

## 🔮 **Variables and Types — Binding Your Chaotic Energy**

### 📘 **Basic Variables (Essence Anchors)**

Your brain: unstable.
Your variables: hopefully less unstable.

```flowlang
let name = "Naruto"
let age = 17
let isNinja = both!
seal MAX_CHAKRA = 10000

let power: Ember = 9000
let title: Silk = "Hokage"
```

**🗡️ Note:**
Your variables are more organized than your sleep schedule.

---

### 🌌 **Arrays & Relics — Constellations & Artifacts**

Data structures forged in starfire and duct tape.

```flowlang
let numbers = [1, 2, 3, 4, 5]
names = ["Goku", "Vegeta", "Gohan"]

numbers.push(6)
```

**🗡️ Note:**
Nice. You discovered arrays. Try not to index out of bounds like last time.

---

### ✨ **String Interpolation — Weaving Silk Runes**

```flowlang
let message = `${name} is ${age} years old`
```

**🗡️ Note:**
FlowLang can interpolate. Too bad you can't interpret social cues.

---

## ⚔️ **Control Flow — Bending Reality with If & Stance**

### **Stance Magic**

```flowlang
in Stance (power >> 9000) {
    roar("IT'S OVER 9000!")
} otherwise {
    shout("Power level normal")
}
```

**🗡️ Note:**
Conditional logic: because your life already has enough chaos.

---

### **Aura Invocation (Switch)**

```flowlang
invoke Aura command {
    when "attack" -> shout("⚔️  Attacking!")
}
```

**🗡️ Note:**
Your switch-case is cleaner than your room.

---

## 🪄 **Functions — Spellcraft for the Digitally Inept**

```flowlang
cast Spell greet(name: Silk) -> Silk {
    return `Hello, ${name}!`
}
```

**🗡️ Note:**
Look at you, returning values like a responsible spellcaster.

---

### 🔁 **Recursive Spells (Forbidden Arts)**

```flowlang
cast Spell factorial(n: Ember) -> Ember {
    in Stance (n <<= 1) return 1
    return n * factorial(n - 1)
}
```

**🗡️ Note:**
May your recursion not summon a stack overflow demon.
(again.)

---

## ♾️ **Loops — Phases of Eternal Suffering**

```flowlang
enter Phase i from 1 to 5 {
    shout("Count: " + i)
}
```

**🗡️ Note:**
At least your loop stops. Unlike your intrusive thoughts.

---

## 💀 **Error Handling — Wrangling the Rifts**

```flowlang
attempt {
    rupture Rift "Connection failed"
} rescue Rift as e {
    shout("Error: " + e)
}
```

**🗡️ Note:**
Your code now handles errors.
If only you handled your emotions the same way.

---

## 🌙 **Async/Await — Celestial Rituals**

```flowlang
ritual loadData ::
    wait 2s
    return "data"
end
```

**🗡️ Note:**
Good use of async. Finally something you're good at:
**waiting.**

---

## 📦 **Modules — Summoning External Wisdom**

```flowlang
circle math from "std:math"
```

**🗡️ Note:**
Importing modules because you're afraid to write the code yourself?
Same.

---

## 📚 **Standard Library — Tools for the Arcane Engineer**

### 🗂️ File I/O

```flowlang
fs.write("output.txt", "Hello, FlowLang!")
```

**🗡️ Note:**
Wow. A file write. One step closer to deleting your system32 by mistake.

---

### 🌐 HTTP Requests

```flowlang
let data = net.get("https://api.github.com/users/octocat")
```

**🗡️ Note:**
Careful. One more request and GitHub might rate limit your whole existence.

---

## 🎮 **Complete Applications — Full Arcane Constructions**

### 🧮 **Calculator**

A practical example for people who failed math twice.

```flowlang
cast Spell divide(a, b) {
    in Stance (b == 0) rupture Rift "Division by zero!"
}
```

**🗡️ Note:**
Even FlowLang knows dividing by zero is stupid. Why don’t you?

---

### 📜 **To-Do List API — For Your 0% Productivity**

```flowlang
let todos = await fetchTodos()
```

**🗡️ Note:**
A to-do list. Bold of you to assume you'll actually do anything.

---