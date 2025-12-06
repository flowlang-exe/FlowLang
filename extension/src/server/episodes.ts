// FlowLang Episode Error Messages - Anime-themed diagnostics

export interface EpisodeInfo {
    number: number;
    season: number;
    title: string;
    sceneContext: string;
    nextTime: string;
    isWarning: boolean;
}

// Episode database matching src/error/episodes.rs
const EPISODES: Record<string, EpisodeInfo> = {
    // Season 1: Fatal Errors
    'Syntax': {
        number: 1,
        season: 1,
        title: 'THE SHATTERED SIGIL',
        sceneContext: 'Parsing',
        nextTime: 'Can the caster inscribe the sigil correctly before the ritual collapses?',
        isWarning: false,
    },
    'Undefined': {
        number: 3,
        season: 1,
        title: 'THE UNBOUND VARIABLE',
        sceneContext: 'Looping',
        nextTime: 'Will the caster declare the variable before summoning it?',
        isWarning: false,
    },
    'UndefinedDeclaration': {
        number: 2,
        season: 1,
        title: 'THE NAME THAT NEVER WAS',
        sceneContext: 'Declaration',
        nextTime: 'Will a name be forged... or forgotten again?',
        isWarning: false,
    },
    'Type': {
        number: 4,
        season: 1,
        title: 'THE FORBIDDEN TYPE',
        sceneContext: 'Casting',
        nextTime: 'Can the Flow align incompatible energies… or must one be sacrificed?',
        isWarning: false,
    },
    'DivisionByZero': {
        number: 5,
        season: 1,
        title: 'DIVISION OF FATE',
        sceneContext: 'Arithmetic',
        nextTime: 'When destiny divides by nothing… what remains?',
        isWarning: false,
    },
    'VoidTear': {
        number: 6,
        season: 1,
        title: 'THE EMPTY AURA',
        sceneContext: 'Invocation',
        nextTime: 'Will the caster fill the void… or be consumed by it?',
        isWarning: false,
    },
    'OutOfRange': {
        number: 7,
        season: 1,
        title: 'BOUNDARIES OF THE UNKNOWN',
        sceneContext: 'Traversal',
        nextTime: 'Can limits be respected before the Flow shatters?',
        isWarning: false,
    },
    'FileNotFound': {
        number: 8,
        season: 1,
        title: 'LOCKED DOORS, SILENT FILES',
        sceneContext: 'IO Read',
        nextTime: 'Will the lost archive be discovered… or sealed forever?',
        isWarning: false,
    },
    'Permission': {
        number: 9,
        season: 1,
        title: 'THE SEALED SANCTUM',
        sceneContext: 'System Call',
        nextTime: 'Will access be granted… or must a new path be forged?',
        isWarning: false,
    },
    'Timeout': {
        number: 10,
        season: 1,
        title: 'THE ETERNAL WAIT',
        sceneContext: 'Awaiting',
        nextTime: 'Can the caster break free from timeless stasis?',
        isWarning: false,
    },
    'Recursion': {
        number: 11,
        season: 1,
        title: 'THE CIRCULAR CURSE',
        sceneContext: 'Summoning',
        nextTime: 'Will the loop be broken—or become their tomb?',
        isWarning: false,
    },
    'Stack': {
        number: 12,
        season: 1,
        title: 'THE CHAOTIC OVERFLOW',
        sceneContext: 'Deep Invocation',
        nextTime: 'How deep can the Flow spiral before all collapses?',
        isWarning: false,
    },
    'Module': {
        number: 13,
        season: 1,
        title: 'THE UNWRITTEN REALM',
        sceneContext: 'Importing',
        nextTime: 'Will the missing realm finally be summoned into existence?',
        isWarning: false,
    },
    'Promise': {
        number: 14,
        season: 1,
        title: 'THE BROKEN PROMISE',
        sceneContext: 'Async Spell',
        nextTime: 'Will the vow be fulfilled… or abandoned by destiny?',
        isWarning: false,
    },
    'Panic': {
        number: 15,
        season: 1,
        title: 'WHEN THE CORE COLLAPSES',
        sceneContext: 'Final Execution',
        nextTime: 'Is this the end of the Flow… or the start of a new season?',
        isWarning: false,
    },

    // Season 2: Warnings
    'Unused': {
        number: 20,
        season: 2,
        title: 'THE GHOST FILE',
        sceneContext: 'Code Quality',
        nextTime: 'Will the caster banish the ghosts… or be haunted forever?',
        isWarning: true,
    },
    'InfiniteLoop': {
        number: 21,
        season: 2,
        title: 'THE RESTLESS LOOP',
        sceneContext: 'Control Flow',
        nextTime: 'Can the cycle be broken before time collapses?',
        isWarning: true,
    },
    'Deprecated': {
        number: 18,
        season: 2,
        title: 'THE ECHOING SPELL',
        sceneContext: 'Legacy Code',
        nextTime: 'Will the caster evolve… or cling to old magic?',
        isWarning: true,
    },

    // Season 3: Module Errors
    'ModuleNotFound': {
        number: 26,
        season: 3,
        title: 'MISSING IN ACTION',
        sceneContext: 'Module Resolution',
        nextTime: '404: Character not cast in this episode.',
        isWarning: false,
    },
    'InvalidImport': {
        number: 28,
        season: 3,
        title: 'IDENTITY THEFT',
        sceneContext: 'Import Resolution',
        nextTime: 'Not everyone you call is your friend.',
        isWarning: false,
    },

    // Sealed reassignment
    'SealedReassign': {
        number: 4,
        season: 1,
        title: 'THE FORBIDDEN TYPE',
        sceneContext: 'Sealed Variable',
        nextTime: 'The seal cannot be broken. Choose a mutable vessel.',
        isWarning: false,
    },

    // Default
    'Unknown': {
        number: 0,
        season: 1,
        title: 'THE UNKNOWN DISTURBANCE',
        sceneContext: 'Mystery',
        nextTime: 'What chaos awaits in the shadows?',
        isWarning: false,
    },
};

export function getEpisodeForError(errorType: string): EpisodeInfo {
    return EPISODES[errorType] || EPISODES['Unknown'];
}

export function formatEpisodeMessage(episode: EpisodeInfo, message: string): string {
    const symbol = episode.isWarning ? '⚠️' : '✦';
    const episodeNum = episode.number.toString().padStart(2, '0');

    // Include the "NEXT TIME" hint for that anime feel
    return `${symbol} EP${episodeNum} "${episode.title}" — ${message}\n\n📺 ${episode.nextTime}`;
}

export function formatEpisodeHover(episode: EpisodeInfo): string {
    return `NEXT TIME: ${episode.nextTime}`;
}

// Map error types based on message content
export function detectEpisodeType(message: string): string {
    const lowerMsg = message.toLowerCase();

    // Division by zero
    if (lowerMsg.includes('division') || lowerMsg.includes('divide by zero') || lowerMsg.includes('divide by 0')) {
        return 'DivisionByZero';
    }
    // Out of range / index errors
    if (lowerMsg.includes('out of range') || lowerMsg.includes('index') || lowerMsg.includes('bounds') || lowerMsg.includes('boundary')) {
        return 'OutOfRange';
    }
    // File not found
    if (lowerMsg.includes('file not found') || lowerMsg.includes('no such file') || lowerMsg.includes('cannot find file')) {
        return 'FileNotFound';
    }
    // Undefined variable
    if (lowerMsg.includes('undefined') || lowerMsg.includes('not defined')) {
        if (lowerMsg.includes('function') || lowerMsg.includes('spell')) {
            return 'UndefinedDeclaration';
        }
        return 'Undefined';
    }
    // Type mismatch
    if (lowerMsg.includes('type mismatch') || lowerMsg.includes('type')) {
        return 'Type';
    }
    // Sealed reassignment
    if (lowerMsg.includes('sealed') || lowerMsg.includes('reassign') || lowerMsg.includes('cannot reassign')) {
        return 'SealedReassign';
    }
    // Syntax errors
    if (lowerMsg.includes('syntax') || lowerMsg.includes('expected')) {
        return 'Syntax';
    }
    // Module errors
    if (lowerMsg.includes('module') || lowerMsg.includes('import')) {
        return 'ModuleNotFound';
    }
    // Void/null access
    if (lowerMsg.includes('void') || lowerMsg.includes('null') || lowerMsg.includes('empty')) {
        return 'VoidTear';
    }
    // Timeout
    if (lowerMsg.includes('timeout') || lowerMsg.includes('timed out')) {
        return 'Timeout';
    }
    // Recursion
    if (lowerMsg.includes('recursion') || lowerMsg.includes('stack overflow')) {
        return 'Recursion';
    }
    // Permission
    if (lowerMsg.includes('permission') || lowerMsg.includes('denied') || lowerMsg.includes('access')) {
        return 'Permission';
    }
    // Argument errors
    if (lowerMsg.includes('argument')) {
        return 'Type';
    }
    return 'Unknown';
}
