use super::{id::Id, err::{Err, ErrType::*, AsResult}};
use TokType::*;

/// delimiters
const DELMS: [char; 5] = [' ', '\n', '\t', '\"', '#'];
/// operators
const OPERS: [char; 6] = ['(', ')', '\'', ',', '@', '"'];

#[derive(Clone, PartialEq)]
enum TokType {
    Sym(String),
    Beg,
    End,
    Esc,
    Quote,
    Apply,
}

#[derive(Clone)]
struct Tok {
    pub typ: TokType,
    pub id: Id
}

pub struct Parser {
    tokens: Vec<Tok>
}

impl Parser {
    // fn from_source(src: &String) -> Self {
    //     let mut 
    // }

    fn tok_id_index(&self, id: &Id) -> Option<usize> {
        self.tokens.iter().position(|tok| tok.id == *id)
    }

    fn push_tok(&mut self, typ: TokType) {
        self.tokens.push(
            Tok { 
                typ: typ, 
                id: Id::new() 
            }
        )
    }

    fn insert_tok(&mut self, index: usize, typ: TokType) {
        self.tokens.insert(
            index,
            Tok { 
                typ: typ, 
                id: Id::new() 
            }
        )
    }

    fn parse_tokens(&mut self, src: &String) {
        // extract tokens from src 
        let mut buf = String::new();

        for (i, ch) in src.chars().enumerate() {
            if DELMS.contains(&ch) || OPERS.contains(&ch) {
                if !buf.is_empty() {
                    self.push_tok(TokType::Sym(buf.clone()));      
                    buf.clear();
                }

                match ch {
                    '(' => self.push_tok(Beg),   
                    ')' => self.push_tok(End),
                    ',' => self.push_tok(Esc),     
                    '\'' => self.push_tok(Quote),     
                    '@' => self.push_tok(Apply),     
                    _ => ()
                }
            }
            else {
                buf.push(ch);
                if i + 1 == src.len() {
                    self.push_tok(Sym(buf.clone()))
                }
            }
        }

        self.extract_operators();
    }

    fn extract_operators(&mut self) {
        for (i, tok) in 
        self.tokens
            .clone()
            .iter()
            .enumerate() 
        {
            match tok.typ {
                Quote => {
                    let mut depth = 0;
                    let mut index = i;
        
                    loop {
                        index += 1;
                        let beg_depth = depth;
        
                        for (j, tok) in 
                        self.tokens
                            .iter()
                            .enumerate()
                            .skip(index)
                        {
                            match tok.typ {
                                Beg => depth += 1,
                                End => depth -= 1,
                                Esc => {
                                    index += 1;
                                    break
                                }
                                _ => ()
                            }
        
                            if depth == beg_depth {
                                self.insert_tok(i, Quote);
                                self.insert_tok(i, Beg);
                                self.insert_tok(index, End);

                                index = j;
                                break
                            }
                        }
        
                        if depth == 0 {
                            break
                        }
                    }
                }
                Apply => {
                    let mut depth = 1;
        
                    for (j, tok) in 
                    self.tokens
                        .iter()
                        .enumerate()
                        .rev()
                        .skip(self.tokens.len() - i) 
                    {
                        match tok.typ {
                            TokType::Beg => depth -= 1,
                            TokType::End => depth += 1,
                            _ => ()
                        }
        
                        if depth == 0 {
                            self.insert_tok(j, Apply);

                            break
                        }
                    }
                }

                _ => ()
            }
        }
    }
}