use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords - Control Flow
    InStance,      // in Stance
    ShiftStance,   // shift Stance
    AbandonStance, // abandon Stance
    InvokeAura,    // invoke Aura
    When,          // when
    Otherwise,     // otherwise
    EnterPhase,    // enter Phase
    From,          // from
    To,            // to
    In,            // in (for for-each loops)
    Until,         // until
    Forever,       // forever
    
    // Keywords - Functions & Variables
    CastSpell,     // cast Spell
    Ritual,        // ritual
    Await,         // await
    Perform,       // perform
    Wait,          // wait
    Let,           // let
    Seal,          // seal
    Return,        // return
    
    // Keywords - Modules
    Circle,        // circle
    As,            // as
    End,           // end
    
    // Keywords - Built-in Functions
    Whisper,       // whisper
    Shout,         // shout
    Roar,          // roar
    Chant,         // chant
    Drift,         // drift
    Strike,        // strike
    
    // ⚔️ ERROR ARC - Error Handling Keywords
    Panic,         // panic
    Wound,         // wound
    Attempt,       // attempt
    Rescue,        // rescue
    Rebound,       // rebound
    Ward,          // ward
    Break,         // break
    Fracture,      // fracture
    Shatter,       // shatter
    Finally,       // finally
    Retry,         // retry
    GrandSeal,     // grand_seal
    Rupture,       // rupture (throw error)
    
    // Type Keywords (Essences)
    Ember,         // Ember (number)
    Silk,          // Silk (string)
    Pulse,         // Pulse (boolean)
    Flux,          // Flux (any)
    Hollow,        // Hollow (void)
    Constellation, // Constellation (array)
    Relic,         // Relic (object/map)
    Spell,         // Spell (function type)
    SigilDef,      // sigil (type definition keyword)
    
    // Operators - Comparison
    IsEqual,       // is~
    NotEqual,      // not~
    Greater,       // >>
    Less,          // <<
    GreaterEq,     // >>=
    LessEq,        // <<=
    
    // Operators - Logical
    Both,          // both!
    Either,        // either!
    Negate,        // negate!
    
    // Operators - Arithmetic
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    
    // Operators - Combo Chain
    ChainOp,       // >> (also used for greater, context-dependent)
    ChainEnd,      // !!
    
    // Delimiters
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    LeftBracket,   // [
    RightBracket,  // ]
    Comma,         // ,
    Dot,           // .
    Colon,         // :
    DoubleColon,   // ::
    Arrow,         // ->
    FatArrow,      // =>
    Equals,        // =
    
    // Literals
    Number(f64),
    String(String),
    StringPart(String),
    InterpolationStart, // ${
    True,
    False,
    
    // Identifiers
    Identifier(String),
    
    // Sigils (decorators)
    Sigil(String), // @experimental, @internal, etc.
    
    // Special
    Newline,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Number(n) => write!(f, "Number({})", n),
            TokenKind::String(s) => write!(f, "String(\"{}\")", s),
            TokenKind::StringPart(s) => write!(f, "StringPart(\"{}\")", s),
            TokenKind::InterpolationStart => write!(f, "InterpolationStart"),
            TokenKind::Identifier(s) => write!(f, "Identifier({})", s),
            TokenKind::Sigil(s) => write!(f, "Sigil(@{})", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            kind,
            lexeme,
            line,
            column,
        }
    }
}
