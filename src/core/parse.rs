use super::{
    err::{
        Err, 
        ErrType::*, 
        AsResult
    }, 
    node::Node, 
    rc_cell::RcCell, 
    id::Id, 
    context::{
        Mod, 
        Env
    }, 
    object::Obj, type_id::TypeId};
use Tok::*;

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

pub struct Parser {
    source: String,
    file: Option<String>,
    tokens: Vec<Tok>,
    module: Mod
}

impl Parser {
    /// Create a new Parser with 
    /// content read from a file
    pub fn parse_file(env: &Env, name: &str, path: String) -> Err<Self> {
        Self::new(env, name.to_string(), std::fs::read_to_string(path.clone())?, Some(path))
    }
    /// Create a new Parse with 
    /// content read from a string
    pub fn parse_string(env: &Env, name: &str, src: String) -> Err<Self> {
        Self::new(env, name.to_string(), src.to_string(), None)
    }
    /// Create a new Parser, extracting the 
    /// tokens and abstract syntax tree from source
    fn new(env: &Env, name: String, src: String, path: Option<String>) -> Err<Self> {
        let mut parser = 
            Parser {
                file: path,
                source: src,
                tokens: Vec::new(),
                module: Mod::new(name, env.prelude())
            };

        parser.to_toks();
        parser.to_ast();
        Ok(parser)
    }
    /// Lexically separates src into a Vec of Toks  
    fn to_toks(&mut self) {
        // lexical buffer
        let mut buf = String::new();

        for (i, ch) in self.source.chars().enumerate() {
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
                if i + 1 == self.source.len() {
                    self.tokens.push(Sym(buf.clone()));
                }
            }
        }
    }
    /// Utilizes the Vec of Toks to organize the 
    /// src into an abstract syntax tree of Nodes
    fn to_ast(&mut self) -> Err {
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
                        let val = self.module.add_sym(sym.as_str(), curr_node.into_obj())?;

                        prev.push(RcCell::from(val));
                        curr_node = prev;
                    }
                }
                Sym(sym) => {
                    if !self.module.has_sym(&sym) {
                        //self.module.add_sym(sym, val)
                    }
                }
                _ => ()
            }
        }

        Ok(())
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