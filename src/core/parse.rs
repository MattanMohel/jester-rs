use super::{err::{Err, ErrType::*, AsResult}, node::Node, rc_cell::RcCell};
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
    tokens: Vec<Tok>
}

impl Parser {                
    fn to_toks(&mut self, src: &String) {
        // lexical buffer
        let mut buf = String::new();

        for (i, ch) in src.chars().enumerate() {
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
                if i + 1 == src.len() {
                    self.tokens.push(Sym(buf.clone()));
                }
            }
        }
    }

    fn to_ast(&mut self) {
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
                        // add prev to env
                        //prev.push(RcCell::from(/*symbol*/))
                        curr_node = prev;
                    }
                }
                Sym(src) => {

                }
                _ => ()
            }
        }
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