// Episode information for anime-themed error messages
#[derive(Debug, Clone)]
pub struct EpisodeInfo {
    pub number: u8,
    pub season: u8,
    pub title: &'static str,
    pub scene_context: &'static str,
    pub next_time: &'static str,
    pub is_warning: bool,
}

// Stack frame for call trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub ritual_name: String,
    pub line: usize,
    pub is_async: bool,
    pub is_rescued: bool,
    pub file: String,
}

impl EpisodeInfo {
    pub fn banner(&self) -> String {
        let symbol = if self.is_warning { "⚠️" } else { "✦" };
        format!(
            "{} EPISODE {:02} — \"{}\" {}",
            symbol, self.number, self.title, symbol
        )
    }
}

// Episode database - Season 1: Fatal Errors
pub fn get_episode_for_error(error_name: &str, context: &str) -> EpisodeInfo {
    match error_name {
        "Syntax" => EpisodeInfo {
            number: 1,
            season: 1,
            title: "THE SHATTERED SIGIL",
            scene_context: "Parsing",
            next_time: "Can the caster inscribe the sigil correctly before the ritual collapses?",
            is_warning: false,
        },
        "Undefined" => {
            if context.contains("declared") || context.contains("Identifier") {
                EpisodeInfo {
                    number: 2,
                    season: 1,
                    title: "THE NAME THAT NEVER WAS",
                    scene_context: "Declaration",
                    next_time: "Will a name be forged... or forgotten again?",
                    is_warning: false,
                }
            } else {
                EpisodeInfo {
                    number: 3,
                    season: 1,
                    title: "THE UNBOUND VARIABLE",
                    scene_context: "Looping",
                    next_time: "Will the caster declare the variable before summoning it?",
                    is_warning: false,
                }
            }
        }
        "Type" => EpisodeInfo {
            number: 4,
            season: 1,
            title: "THE FORBIDDEN TYPE",
            scene_context: "Casting",
            next_time: "Can the Flow align incompatible energies… or must one be sacrificed?",
            is_warning: false,
        },
        "DivisionByZero" => EpisodeInfo {
            number: 5,
            season: 1,
            title: "DIVISION OF FATE",
            scene_context: "Arithmetic",
            next_time: "When destiny divides by nothing… what remains?",
            is_warning: false,
        },
        "VoidTear" => EpisodeInfo {
            number: 6,
            season: 1,
            title: "THE EMPTY AURA",
            scene_context: "Invocation",
            next_time: "Will the caster fill the void… or be consumed by it?",
            is_warning: false,
        },
        "OutOfRange" => EpisodeInfo {
            number: 7,
            season: 1,
            title: "BOUNDARIES OF THE UNKNOWN",
            scene_context: "Traversal",
            next_time: "Can limits be respected before the Flow shatters?",
            is_warning: false,
        },
        "Rift" => {
            if context.contains("File") || context.contains("not found") {
                EpisodeInfo {
                    number: 8,
                    season: 1,
                    title: "LOCKED DOORS, SILENT FILES",
                    scene_context: "IO Read",
                    next_time: "Will the lost archive be discovered… or sealed forever?",
                    is_warning: false,
                }
            } else if context.contains("Permission") || context.contains("denied") {
                EpisodeInfo {
                    number: 9,
                    season: 1,
                    title: "THE SEALED SANCTUM",
                    scene_context: "System Call",
                    next_time: "Will access be granted… or must a new path be forged?",
                    is_warning: false,
                }
            } else if context.contains("timeout") || context.contains("Timeout") {
                EpisodeInfo {
                    number: 10,
                    season: 1,
                    title: "THE ETERNAL WAIT",
                    scene_context: "Awaiting",
                    next_time: "Can the caster break free from timeless stasis?",
                    is_warning: false,
                }
            } else {
                // Default Rift episode
                EpisodeInfo {
                    number: 8,
                    season: 1,
                    title: "LOCKED DOORS, SILENT FILES",
                    scene_context: "Network",
                    next_time: "Will the connection be restored… or lost to the void?",
                    is_warning: false,
                }
            }
        }
        "Recursion" => EpisodeInfo {
            number: 11,
            season: 1,
            title: "THE CIRCULAR CURSE",
            scene_context: "Summoning",
            next_time: "Will the loop be broken—or become their tomb?",
            is_warning: false,
        },
        "Stack" => EpisodeInfo {
            number: 12,
            season: 1,
            title: "THE CHAOTIC OVERFLOW",
            scene_context: "Deep Invocation",
            next_time: "How deep can the Flow spiral before all collapses?",
            is_warning: false,
        },
        "Module" => EpisodeInfo {
            number: 13,
            season: 1,
            title: "THE UNWRITTEN REALM",
            scene_context: "Importing",
            next_time: "Will the missing realm finally be summoned into existence?",
            is_warning: false,
        },
        "Promise" => EpisodeInfo {
            number: 14,
            season: 1,
            title: "THE BROKEN PROMISE",
            scene_context: "Async Spell",
            next_time: "Will the vow be fulfilled… or abandoned by destiny?",
            is_warning: false,
        },
        "Panic" => EpisodeInfo {
            number: 15,
            season: 1,
            title: "WHEN THE CORE COLLAPSES",
            scene_context: "Final Execution",
            next_time: "Is this the end of the Flow… or the start of a new season?",
            is_warning: false,
        },
        
        // Season 2: Warnings
        "Performance" => EpisodeInfo {
            number: 16,
            season: 2,
            title: "THE SLUGGISH FLOW",
            scene_context: "Execution",
            next_time: "Can the caster optimize their ritual before mana runs dry?",
            is_warning: true,
        },
        "Memory" => EpisodeInfo {
            number: 17,
            season: 2,
            title: "THE SWELLING AURA",
            scene_context: "Resource Management",
            next_time: "Will control be maintained—or will chaos spill forth?",
            is_warning: true,
        },
        "Deprecated" => EpisodeInfo {
            number: 18,
            season: 2,
            title: "THE ECHOING SPELL",
            scene_context: "Legacy Code",
            next_time: "Will the caster evolve… or cling to old magic?",
            is_warning: true,
        },
        "AsyncOverload" => EpisodeInfo {
            number: 19,
            season: 2,
            title: "THE FRAGMENTED THREADS",
            scene_context: "Parallel Execution",
            next_time: "Can the threads be woven—or will the tapestry tear?",
            is_warning: true,
        },
        "Unused" => EpisodeInfo {
            number: 20,
            season: 2,
            title: "THE GHOST FILE",
            scene_context: "Code Quality",
            next_time: "Will the caster banish the ghosts… or be haunted forever?",
            is_warning: true,
        },
        "InfiniteLoop" => EpisodeInfo {
            number: 21,
            season: 2,
            title: "THE RESTLESS LOOP",
            scene_context: "Control Flow",
            next_time: "Can the cycle be broken before time collapses?",
            is_warning: true,
        },
        "Power" => EpisodeInfo {
            number: 22,
            season: 2,
            title: "THE DIMMING FLAME",
            scene_context: "System Resources",
            next_time: "Can the ritual finish before the flame dies?",
            is_warning: true,
        },
        "Timeout" => EpisodeInfo {
            number: 23,
            season: 2,
            title: "THE WANING SIGNAL",
            scene_context: "Network Communication",
            next_time: "Will the message cross realms—or be lost forever?",
            is_warning: true,
        },
        
        // Season 3: Module System Errors
        "PrivateAccess" => EpisodeInfo {
            number: 24,
            season: 3,
            title: "FORBIDDEN ACCESS",
            scene_context: "Module Import",
            next_time: "If it ain't exported, you ain't invited.",
            is_warning: false,
        },
        "CircularDependency" => EpisodeInfo {
            number: 25,
            season: 3,
            title: "THE SNAKE EATING ITS TAIL",
            scene_context: "Module Loading",
            next_time: "Two modules. One loop. No survivors.",
            is_warning: false,
        },
        "ModuleNotFound" => EpisodeInfo {
            number: 26,
            season: 3,
            title: "MISSING IN ACTION",
            scene_context: "Module Resolution",
            next_time: "404: Character not cast in this episode.",
            is_warning: false,
        },
        "DuplicateExport" => EpisodeInfo {
            number: 27,
            season: 3,
            title: "DOUBLE AGENT",
            scene_context: "Export Declaration",
            next_time: "There can only be one.",
            is_warning: false,
        },
        "InvalidImport" => EpisodeInfo {
            number: 28,
            season: 3,
            title: "IDENTITY THEFT",
            scene_context: "Import Resolution",
            next_time: "Not everyone you call is your friend.",
            is_warning: false,
        },
        "ImportStyleMismatch" => EpisodeInfo {
            number: 29,
            season: 3,
            title: "WRONG DELIVERY",
            scene_context: "Import Syntax",
            next_time: "Order mismatch. Delivery cancelled.",
            is_warning: false,
        },
        "UndefinedExport" => EpisodeInfo {
            number: 30,
            season: 3,
            title: "GHOST EXPORT",
            scene_context: "Export Validation",
            next_time: "You can't export what never lived.",
            is_warning: false,
        },
        "EmptyModule" => EpisodeInfo {
            number: 31,
            season: 3,
            title: "EMPTY STAGE",
            scene_context: "Module Loading",
            next_time: "All hype. No content.",
            is_warning: true,
        },
        "VersionIncompatibility" => EpisodeInfo {
            number: 32,
            season: 3,
            title: "VERSION WAR",
            scene_context: "Module Compatibility",
            next_time: "Old code. New pain.",
            is_warning: false,
        },
        "ReExportConflict" => EpisodeInfo {
            number: 33,
            season: 3,
            title: "CLASH OF RE-EXPORTS",
            scene_context: "Re-Export Resolution",
            next_time: "When stars collide, only errors remain.",
            is_warning: false,
        },
        
        // Default/Unknown
        _ => EpisodeInfo {
            number: 0,
            season: 1,
            title: "THE UNKNOWN DISTURBANCE",
            scene_context: "Mystery",
            next_time: "What chaos awaits in the shadows?",
            is_warning: false,
        },
    }
}
