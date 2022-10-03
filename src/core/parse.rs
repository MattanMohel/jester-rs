use std::ops::{Deref, DerefMut};

use super::{
    id::Id, 
    node::Node, 
    obj::Obj, 
    rc_cell::RcCell, 
    type_id::TypeId,
    env::{Mod, Env}, 
    err::{Err, ErrType::*, AsResult}, 
};

/// delimiters
const DELMS: [char; 5] = [' ', '\n', '\t', '\"', '#'];
/// operators
const OPERS: [char; 6] = ['(', ')', '\'', ',', '@', '"'];

#[derive(Clone, PartialEq)]
enum Tok {
    Sym(String),
    Beg,
    End,
    Esc,
    Quote,
    Apply,
}

use Tok::*;

pub struct Parser {
    src: String,
    tokens: Vec<Tok>,
    module: Mod
}

impl Parser {
    /// Creates Parser from filepath
    pub fn from_file(env: &Env, name: &str, path: String) -> Err<Self> {
        let src = std::fs::read_to_string(path.clone())?;
        Self::new(env, name.to_string(), src)
    }

    /// Creates Parser from String
    pub fn from_src(env: &Env, name: &str, src: &str) -> Err<Self> {
        Self::new(env, name.to_string(), src.to_string())
    }

    /// TODO
    fn new(env: &Env, name: String, src: String) -> Err<Self> {    
        let mut parser = {
            Self {
                src,
                tokens: Vec::new(),
                module: Mod::new(name)
            }
        };
        
        parser.module.import(env.prelude());

        parser.to_toks();   
        let exec = parser.to_ast()?;
        parser.module.add_exec(exec);
        Ok(parser)
    }

    /// Lexically separate src into a Vec of Toks  
    fn to_toks(&mut self) {
        // lexical buffer
        let mut buf = String::new();

        for (i, ch) in self.src.chars().enumerate() {
            if DELMS.contains(&ch) || OPERS.contains(&ch) {
                if !buf.is_empty() {
                    self.tokens.push(Tok::Sym(buf.clone()));      
                    buf.clear();
                }

                match ch {
                    '(' => self.tokens.push(Beg),   
                    ')' => self.tokens.push(End),
                    ',' => self.tokens.push(Esc),     
                    '\'' => self.tokens.push(Quote),     
                    '@' => self.tokens.push(Apply),     
                    _ => ()
                }
            }
            else {
                buf.push(ch);
                if i + 1 == self.src.len() {
                    self.tokens.push(Sym(buf.clone()));
                }
            }
        }
    }

    /// Organizes the Vec of Toks into an abstract syntax tree of Nodes
    fn to_ast(&mut self) -> Err<Obj> {
        let mut curr_node = Node::default();
        let mut prev_nodes = Vec::new();
        
        for tok in self.tokens.iter() {
            match tok {
                Beg => {
                    prev_nodes.push(curr_node);
                    curr_node = Node::default();
                }

                End => {
                    if let Some(mut prev) = prev_nodes.pop() {
                        let sym = Self::gen_symbol();
                        let val = self.module.add_sym(sym, curr_node.into_obj())?;

                        prev.push(RcCell::from(val));
                        curr_node = prev;
                    }
                }

                Sym(sym) => {
                    if !self.module.has_sym(&sym) {
                        self.module.add_sym(sym.clone(), Obj::from(sym))?;
                    }

                    let obj = RcCell::from(Obj::Sym(self.module.sym(&sym)?));           
                    curr_node.push(obj);
                }
                _ => ()
            }
        }

        Ok(Obj::Lst(curr_node))
    }

    pub(crate) fn into(self) -> Mod {
        self.module
    }

    /// Creates a unique identifier
    fn gen_symbol() -> String {
        format!("G#{}", Id::next_id())
    }   


    // fn extract_operators(&mut self) {
    //     let clone = self.tokens.clone();

    //     for (i, tok) in clone.iter().enumerate().filter(|(_, tok)| **tok == Quote) {
    //         let mut depth = 0;
    //         let mut index = i;

    //         loop {
    //             index += 1;
    //             let beg_depth = depth;

    //             for (j, tok) in clone.iter().enumerate().skip(index) {
    //                 match tok {
    //                     Beg => depth += 1,
    //                     End => depth -= 1,
    //                     Esc => {
    //                         index += 1;
    //                         break
    //                     }
    //                     _ => ()
    //                 }

    //                 if depth == beg_depth {
    //                     self.tokens.insert(i, Quote);
    //                     self.tokens.insert(i, Beg);
    //                     self.tokens.insert(index, End);

    //                     index = j;
    //                     break
    //                 }
    //             }

    //             if depth == 0 {
    //                 break
    //             }
    //         }
    //     }

    //     for (i, tok) in clone.iter().enumerate().filter(|tok| tok == Quote) {
    //          let mut depth = 0;
    //                 let mut index = i;
        
    //                 loop {
    //                     index += 1;
    //                     let beg_depth = depth;
        
    //                     for (j, tok) in 
    //                     self.tokens
    //                         .iter()
    //                         .enumerate()
    //                         .skip(index)
    //                     {
    //                         match tok.typ {
    //                             Beg => depth += 1,
    //                             End => depth -= 1,
    //                             Esc => {
    //                                 index += 1;
    //                                 break
    //                             }
    //                             _ => ()
    //                         }
        
    //                         if depth == beg_depth {
    //                             self.insert_tok(i, Quote);
    //                             self.insert_tok(i, Beg);
    //                             self.insert_tok(index, End);

    //                             index = j;
    //                             break
    //                         }
    //                     }
        
    //                     if depth == 0 {
    //                         break
    //                     }
    //                 }
    //         match tok.typ {
    //             Quote => {
                   
    //             }
    //             Apply => {
    //                 let mut depth = 1;
        
    //                 for (j, tok) in 
    //                 self.tokens
    //                     .iter()
    //                     .enumerate()
    //                     .rev()
    //                     .skip(self.tokens.len() - i) 
    //                 {
    //                     match tok.typ {
    //                         Tok::Beg => depth -= 1,
    //                         Tok::End => depth += 1,
    //                         _ => ()
    //                     }
        
    //                     if depth == 0 {
    //                         self.insert_tok(j, Apply);

    //                         break
    //                     }
    //                 }
    //             }

    //             _ => ()
    //         }
    //     }
    // }
}