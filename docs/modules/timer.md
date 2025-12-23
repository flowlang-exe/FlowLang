# std:timer ⚡

Asynchronous timer functions for intervals and timeouts. Timers keep the FlowLang process alive until cleared.

## Import

```flowlang
circle timer from "std:timer"
```

## Functions

### `interval(ms: Ember, callback: Spell) -> Handle`
Create a repeating timer that calls the callback every `ms` milliseconds. Returns a Handle that can be used to cancel the timer.

> **Note:** Callbacks execute during `wait` statements or after script completion while in the event loop.

```flowlang
let count = 0

cast Spell tick() -> Hollow {
    count = count + 1
    shout("🔔 Tick #" + count)
}

let handle = timer.interval(1000, tick)  -- Every 1 second

wait 5s  -- Ticks will execute during wait
timer.clear(handle)  -- Stop the timer
```

### `timeout(ms: Ember, callback: Spell) -> Handle`
Create a one-shot timer that calls the callback after `ms` milliseconds. The timer automatically clears after execution.

```flowlang
cast Spell done() -> Hollow {
    shout("⏰ Timer finished!")
}

timer.timeout(3000, done)  -- Fire once after 3 seconds
wait 4s  -- Wait for timeout to execute
```

### `clear(handle: Handle) -> Pulse`
Cancel a timer by its handle. Returns `both!` if the timer was successfully cancelled, `none!` if already cleared.

```flowlang
let handle = timer.interval(500, myCallback)
wait 2s
let cleared = timer.clear(handle)
shout("Timer cleared: " + cleared)  -- "Timer cleared: both!"
```
