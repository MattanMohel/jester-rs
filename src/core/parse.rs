use super::{
    id::Id, 
    node::Node, 
    obj::Obj, 
    rc_cell::RcCell, 
    type_id::TypeId,
    env::Env, 
    err::Err 
};

/// `Jester-rs` delimeters
const DELIMETERS: [char; 4] = [
    ' ',  // space
    '\n', // new line
    '\t', // tab
    '\"'  // string
];

/// `Jester-rs` operators
const OPERATORS: [char; 5] = [
    '(',  // s-expression beg 
    ')',  // s-expression end
    '\'', // quote 
    ',',  // exclude
    '.',  // qualifier
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
struct Tok {
    tok_type: TokType,
    id: Id
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
enum TokType {
    Sym(String),
    Beg,
    End,
    Esc,
    Quote,
}

impl Into<Tok> for TokType {
    fn into(self) -> Tok {
        Tok { 
            tok_type: self, 
            id: Id::new() 
        }
    }
}

use TokType::*;

struct Expr {
    beg: Id,
    end: Id,
    elems: Vec<Expr>
}

struct Lexer {
    toks: Vec<Tok>,
    exprs: Vec<Expr>
}

impl Lexer {
    pub fn new(env: &mut Env, src: &String) -> Err<Obj> {
        let mut lexer = Lexer {
            toks: Vec::new(),
            exprs: Vec::new(),
        };

        lexer.get_toks(src);
        lexer.get_exprs();

        // - expand operators - //

        let tree = lexer.to_syntax_tree(env);
        tree
            .iter()
            .progn(|obj| env.eval(obj.as_ref()))
    }

    fn expr_end(&self, tok_beg: &Tok) -> &Tok {
        let beg = self.toks
            .iter()
            .position(|rhs| tok_beg.id == rhs.id)
            .expect("token not found!");
        
        let mut depth = 0;

        for tok in self.toks.iter().skip(beg) {
            match tok.tok_type {
                Beg => depth += 1,
                End => depth -= 1,
                _ => ()
            }

            if depth == 0 {
                return tok
            }
        }

        panic!("incomplete expression!")
    }

    fn expand_opers(&mut self) {}

    fn get_exprs(&mut self) {
        for tok in self.toks.iter() {
            match tok.tok_type {
                Beg => {
                    let end = self.expr_end(&tok);

                    self.exprs.push(
                        Expr { 
                            beg: tok.id.clone(), 
                            end: end.id.clone(), 
                            elems: Vec::new() 
                    })
                }
                End => {     
                    if self.exprs.len() > 1 {
                        let expr = self.exprs.pop().unwrap();
                        self.exprs
                            .last_mut()
                            .unwrap()
                            .elems.push(expr);
                    }
                }
                Sym(_) => {
                    let expr = Expr {
                        beg: tok.id.clone(),
                        end: tok.id.clone(),
                        elems: Vec::new(),
                    };

                    if self.exprs.is_empty() {
                        self.exprs.push(expr);
                    }
                    else {
                        self.exprs
                            .last_mut()
                            .unwrap()
                            .elems.push(expr);
                    }
                }
                _ => ()
            }
        }
    }

    /// Linearly extract `Tok`s 
    /// 
    /// ## Note
    /// - A special character is either a `delimeter` or `operator`
    /// - each special character is exactly 1 character
    /// 
    /// ## Explanation
    /// for each character:
    /// ```
    /// if delimeter or operator:
    /// ``` 
    /// - push buffer then special character as `Tok`s 
    /// - reset buffer
    /// ```
    /// else:
    /// ```
    /// - push character to current buffer
    fn get_toks(&mut self, src: &String) {
        // lexical buffer
        let mut lex = String::new();

        for (i, ch) in src.chars().enumerate() {
            if DELIMETERS.contains(&ch) || OPERATORS.contains(&ch) {     
                if !lex.is_empty() {
                    self.toks.push(Sym(lex.clone()).into());
                    lex.clear();
                }

                match ch {
                    '(' =>  self.toks.push(Beg.into()),   
                    ')' =>  self.toks.push(End.into()),
                    ',' =>  self.toks.push(Esc.into()),     
                    '\'' => self.toks.push(Quote.into()),     
                    _ => ()
                }
            }
            else {
                lex.push(ch);
                if i + 1 == src.len() {
                    self.toks.push(Sym(lex.clone()).into());
                }
            }
        }
    }

    /// Convert `Vec<Tok>` into syntax tree
    /// 
    /// ## Note
    /// `Jester-rs` represents code as recursive linked lists of `Obj`
    /// and, for effeciency, the linked lists are represented by `Vec<RcCell<Obj>>`
    ///
    /// ## Example
    /// 
    /// ```
    /// 1: (set x (+ 5 5))
    /// 2: (= x 10)   
    /// 3: 
    /// 4: x 
    /// ```
    /// translates to...
    ///
    /// ```
    /// (...) --> (...) --> 'x'
    ///   \         \__ '=' --> 'x' --> '10'
    ///    \ 
    ///     \__ 'set' --> 'x' --> (...)
    ///                             \__ '+' --> '5' --> '5'
    /// ```
    fn to_syntax_tree(&mut self, env: &mut Env) -> Node {
        let mut cur_node = Node::default();
        let mut pre_node = Vec::new();
        
        for tok in self.toks.iter() {
            match &tok.tok_type {
                Beg => {
                    pre_node.push(cur_node);     
                    cur_node = Node::default();
                }

                End => {
                    if let Some(mut parent) = pre_node.pop() {
                        let obj = env.add_sym(&Self::gen_symbol(), cur_node.into_obj());
                        parent.push(RcCell::from(obj));

                        cur_node = parent;
                    }
                }

                Sym(sym) => {
                    if !env.has_sym(&sym) {
                        env.add_sym(&sym, Obj::from(sym));
                    }

                    // store `RcCell`
                    let obj = env.get_sym(&sym).unwrap();
                    // convert `RcCell` to `Sym`
                    let cell = RcCell::from(Obj::Sym(obj)); 

                    cur_node.push(cell);
                }
                _ => ()
            }
        }

        cur_node
    }

    /// Creates a unique identifier
    fn gen_symbol() -> String {
        format!("G#{}", Id::next_id())
    }   
}

impl Env {
    pub fn add_from_string(&mut self, src: &str) {
        Lexer::new(self, &src.to_string());
    }
    
    pub fn add_from_file(&mut self, path: String) {
        let src = std::fs::read_to_string(path.clone()).expect("couldn't read file!");
        Lexer::new(self, &src);
    }
}