# ⚡ **THE ERROR ARC**

### *FlowLang’s Divine Error Doctrine — Where Bugs Are Judged, Sins Are Purified, and Exceptions Have Lore.*

---

# 🌑 **Essence of the Arc**

In FlowLang, errors aren’t “just errors.”
They are **anomalies in the mana stream**, distortions in reality, glitches in fate.

You don’t “throw errors” in FlowLang.

You **rupture reality**,
summon Spirits,
tear Voids,
and occasionally **obliterate existence with PANIC**.

---

# 🜂 **ARC ERROR CLASSES**

## 🌐 **Rift — Dimensional Instability**

Anything related to IO, net, filesystem, or cosmic connectivity failing.

```flowlang
rupture Rift "Dimensional link failed — server unreachable."
```

When a Rift appears, it means reality said:
**“Connection refused.”**

---

## ⚡ **Glitch — Corrupted Mana**

Parsing failures • formatting errors • malformed data • your JSON crimes.

```flowlang
rupture Glitch "JSON shattered into unholy fragments."
```

---

## 🕳️ **VoidTear — Null Abyss Breach**

Accessing emptiness. Breaking into nothingness.
Basically: touching `null` when you shouldn’t.

```flowlang
rupture VoidTear "Reached into the void — found nothing."
```

---

## 👻 **Spirit — General Phantom**

Generic catch-all error.
If you don't know what it is… it's a Spirit.

```flowlang
rupture Spirit "An unknown presence disturbed the Flow."
```

---

## 💀 **Panic — Core Collapse**

Catastrophic. Unrecoverable.
The runtime **obliterates itself**.

```flowlang
panic "THE MANA CORE IS MELTING — FLEE!"
```

When Panic speaks, your code dies dramatically (as it should).

---

## 🩹 **Wound — Minor Harm**

Not fatal.
Just FlowLang passive-aggressively telling you you're doing something dumb.

```flowlang
wound "Deprecated arcana invoked — be ashamed."
```

---

# 🌀 **RUPTURING THE ARC**

### ❖ Basic Rupture

```flowlang
cast Spell validate(data: Silk) -> Hollow {
    in Stance (data == "") {
        rupture VoidTear "Input hollow — cannot proceed."
    }
}
```

### ❖ Catastrophic Panic

```flowlang
in Stance (db.isCorrupted) {
    panic "Database core fractured — abandon hope."
}
```

### ❖ Gentle Wound

```flowlang
wound "Performance degraded — maybe upgrade your potato."
```

---

# 🔮 **ATTEMPT / RESCUE — Bending Fate**

## ✦ Basic Try-Catch

```flowlang
attempt {
    rupture Glitch "Fate scrambled the JSON"
} rescue Glitch as echo {
    shout("Recovered from distortion: " + echo)
}
```

---

## ✦ Multi-Rescue (Pattern-Based)

```flowlang
attempt {
    fetchMana()
} rescue Rift as r {
    shout("Dimensional rupture: " + r)
} rescue Glitch as g {
    shout("Corrupted glyphs: " + g)
} rescue as spirit {
    shout("A wandering Spirit intervened: " + spirit)
}
```

---

## ✦ Finally (Always Executes)

```flowlang
attempt {
    weave()
} rescue as e {
    shout("Arc failure: " + e)
} finally {
    unweave()
    shout("Mana threads sealed")
}
```

---

## ✦ Retry (Automatic Retries)

```flowlang
attempt {
    connectRealm()
} rescue Rift as e retry 3 {
    shout("Realm unstable — attempting resonance again...")
    wait 1s
}
```

---

# 🛡️ **WARD — Contained Anomaly Field**

Errors inside a `ward` block are absorbed by the arcane barrier.
Execution continues as if you meant to do that.

```flowlang
ward {
    rupture VoidTear "Yes, this breaks — but silently."
}

shout("Flow continues, unbothered.")
```

Perfect for optional operations, like:

* Experimental features
* Loading optional files
* Your questionable logic

```flowlang
ward {
    let config = std:file.read("optional.json")
    applyConfig(config)
}
```

---

# 🔁 **REBOUND — Let the Error Ascend**

Rethrow a caught error:

```flowlang
attempt {
    attempt {
        rupture Glitch "Glyph sequence malformed"
    } rescue Glitch as e {
        log("Marked distortion: " + e)
        rebound e
    }
} rescue Spirit as echo {
    shout("Ascended error received: " + echo)
}
```

Or:

```flowlang
rebound  -- if you don’t care which demon it was
```

---

# 🔨 **SEALS — Loop & Flow Control**

### ✦ break seal — Break Loop

```flowlang
enter Phase i from 1 to 100 {
    in Stance (i >> 10) {
        break seal
    }
    shout(i)
}
```

### ✦ fracture seal — Continue Loop

```flowlang
enter Phase i from 1 to 10 {
    in Stance (i == 5) {
        fracture seal
    }
    shout(i)
}
```

### ✦ shatter grand_seal — Early Return

```flowlang
cast Spell validate(n: Ember) -> Silk {
    in Stance (n << 0) {
        shatter grand_seal "Invalid essence"
    }
    return "Valid essence"
}
```

---

# 💀🔥 **PANIC VS WOUND**

**Use Panic when:**

* The universe collapses
* Internal invariants shatter
* A fate-level violation occurs

**Use Wound when:**

* You're warning the dev
* Their decisions are questionable
* Performance cries but lives

---

# 🎭 **ARCANIC ERROR SCREENS**

### ⚡ **RIFT ERUPTION**

```
═══════════════════════════════════════
⚡ RIFT ERUPTION ⚡
Location: 42:10

"The connection between realms collapsed."

🌐 Dimensional instability detected!
═══════════════════════════════════════
```

### 💀 **SYSTEM PANIC**

```
💀💀💀 SYSTEM PANIC 💀💀💀
═══════════════════════════════════════

🔥 MANA CORE COLLAPSE DETECTED 🔥
"Database corruption! ARC IN RUINS!"

Flow terminated immediately.
Reality destabilized.
═══════════════════════════════════════
```

---

# 🧪 **FULL ARC DEMO (FlowLang LORE MODE)**

```flowlang
circle net from "std:net"
circle json from "std:json"
circle fs from "std:file"

ritual fetchUserData(userId: Ember) ::
    attempt {
        let url = "https://api.example.com/users/" + userId
        let res = net.get(url)
        let data = json.parse(res)
        return data

    } rescue Rift as r retry 3 {
        shout("Realm unstable — retrying...")
        wait 2s

    } rescue Glitch as g {
        shout("Corrupted data glyphs: " + g)

        ward {
            let cached = fs.read("cache/" + userId + ".json")
            return json.parse(cached)
        }

        rebound g

    } finally {
        shout("Arc stabilized — request ritual complete")
    }
end

cast Spell main() -> Hollow {
    attempt {
        let user = await fetchUserData(123)
        shout("User received: " + user["name"])

    } rescue as e {
        shout("Mana retrieval failed: " + e)
        panic "User data missing — cannot maintain timeline."
    }
}

main()
```