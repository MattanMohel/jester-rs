use std::fmt::Debug;


/// `Jester-rs` delimeters
pub(crate) const CONTROLS: [char; 5] = [
    ' ',  // space
    '\n', // new line
    '\r', // curso new line
    '\t', // tab
    ';'   // comment
];
    
/// `Jester-rs` operators
pub(crate) const OPERATORS: [char; 5] = [
    '(',  // s-expression beg 
    ')',  // s-expression end
    '\'', // quote 
    '\"', // string
    ','   // quote escape
];

pub(crate) const METAS: [char; 4] = [
    '[',
    ']',
    '{',
    '}'
];

/// Represents a lexical token
/// 
/// ## Example
/// ```
/// (+ 1 2 3)
/// ```
/// 
/// breaks into `Tok`s: 
/// ```
/// ['(', '+', '1', '2', '3', ')']
/// ```
#[derive(Clone, PartialEq)]
pub(crate) struct Tok {
    pub tok_type: TokType,
    pub id: usize
}

impl Debug for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tok_type)
    }
}

/// Represents the type of a token
/// 
/// ## Example
/// ```
/// (+ 1 2 3)
/// ```
/// 
/// breaks into `TokType`s: 
/// ```
/// [Beg, Sym, Sym, Sym, Sym, End]
/// ```
#[derive(Clone, PartialEq)]
pub(crate) enum TokType {
    Sym(String),
    Beg,
    End,
    Esc,
    Qte,
}

impl Debug for TokType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokType::Sym(_) => write!(f, "Sym"),
            TokType::Beg    => write!(f, "Beg"),
            TokType::End    => write!(f, "End"),
            TokType::Esc    => write!(f, "Esc"),
            TokType::Qte    => write!(f, "Qte")
        }
    }
}

impl TokType {
    pub(crate) fn to_tok(self, id: usize) -> Tok {
        Tok { 
            tok_type: self, 
            id 
        }
    }
}

/// Represents an `s-expression`
#[derive(Clone)]
pub(crate) struct Expr {
    pub beg_id: isize,
    pub end_id: isize,
    pub qte: bool,
    pub esc: bool,
    pub elems: Vec<Expr>
}

impl Expr {
    pub fn map_each<F>(&self, map: &mut F)
    where 
        F: FnMut(&Expr) {
        
        map(self);

        for expr in self.elems.iter() {
            expr.map_each(map);
        }
    }


    pub fn map_chosen<F>(&self, map: &mut F)
    where 
        F: FnMut(&Expr) -> bool 
    {
        if !map(self) {
            return
        }

        for expr in self.elems.iter() {
            expr.map_chosen(map);
        }
    }

    pub fn any<F>(&self, f: F) -> bool
    where 
        F: Fn(&Expr) -> bool + Copy
    {    
        if f(self) {
            return true
        }

        for expr in self.elems.iter() {
            if expr.any(f) {
                return true
            }
        }

        false
    }
}