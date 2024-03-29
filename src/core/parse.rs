use super::{
    node::Node, 
    obj::Obj, 
    type_id::TypeId,
    env::Env, 
    err::Err,
    lex::{
        Tok,
        Expr,
        CONTROLS,
        OPERATORS,
        METAS,
        TokType::{self, *}
    }
};

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
        lexer.expand_ops();

        let tree = lexer.to_syntax_tree(env);

        tree
            .iter()
            .progn(|obj| env.eval(obj.as_ref()))
    }

    fn add_tok(&mut self, tok_type: TokType) {
        let tok = Tok {
            tok_type,
            id: self.toks.len()
        };

        self.toks.push(tok);
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
        // whether parsing comment
        let mut com = false;
        // whether parsing string
        let mut str  = false;

        for (i, ch) in src.chars().enumerate() {                             
            match ch {
                '\n' if !str => com = false,
                ';' if !str => com = !com,
                '"' if !com => {     
                    if str {
                        lex.push(ch);
                    }
                    str = !str;
                }   
                _ => ()
            }

            if com {
                continue;
            }

            let meta = METAS.contains(&ch);
            let op = OPERATORS.contains(&ch);
            let cntrl = CONTROLS.contains(&ch);

            if !str && (cntrl || op || meta) {     
                if !lex.is_empty() {
                    self.add_tok(Sym(lex.clone()));
                    lex.clear();
                }

                if meta {
                    self.add_tok(Sym(ch.to_string()));
                    continue;
                }

                match ch {
                    '('  => self.add_tok(Beg),   
                    ')'  => self.add_tok(End),
                    ','  => self.add_tok(Esc),     
                    '\'' => self.add_tok(Qte), 
                    _ => ()
                }
            }
            else {
                lex.push(ch);

                if !lex.is_empty() && i + 1 == src.len() {
                    self.add_tok(Sym(lex.clone()));                
                }
            }
        }
    }

    fn get_exprs(&mut self) {
        // is token quoted?
        let mut qte = false;
        // is token escaped?
        let mut esc = false;

        for tok in self.toks.iter() {
            match tok.tok_type {
                Beg => {
                    self.exprs.push(
                        Expr { 
                            beg_id: tok.id as isize, 
                            end_id: -1, // uninit state 
                            elems: Vec::new(),
                            esc,
                            qte,
                    });

                    qte = false;
                    esc = false;
                }

                End => {
                    self.exprs.last_mut().unwrap().end_id = tok.id as isize;

                    if self.exprs.len() > 1 {
                        let expr = self.exprs.pop().unwrap();

                        self.exprs
                            .last_mut()
                            .unwrap()
                            .elems
                            .push(expr);
                    }
                }
                
                Sym(_) => {
                    let expr = Expr {
                        beg_id: tok.id as isize,
                        end_id: tok.id as isize,
                        elems: Vec::new(),
                        esc,
                        qte
                    };

                    if self.exprs.is_empty() {
                        self.exprs.push(expr);
                    }
                    else {
                        self.exprs
                            .last_mut()
                            .unwrap()
                            .elems
                            .push(expr);
                    }

                    qte = false;
                    esc = false;
                }

                Esc   => esc = true,

                Qte => qte = true
            }
        }
    }

    fn expand_ops(&mut self) {
        for expr in self.exprs.iter() {
            expr.map_each(&mut |i| {
                if i.qte {
                    i.map_chosen(&mut |j| {
                        if j.esc {
                            return false;
                        }

                        if !j.any(|k| k.esc) {
                            // expand quote for expression j
                            let beg = self.toks
                                .iter()
                                .position(|rhs| j.beg_id == rhs.id as isize)
                                .unwrap();

                            let end = self.toks
                                .iter()
                                .position(|rhs| j.end_id == rhs.id as isize)
                                .unwrap();

                            let id = self.toks.len();

                            self.toks.insert(end + 1, End.to_tok(id + 0));
                            self.toks.insert(beg, Sym("quote".to_string()).to_tok(id + 1));
                            self.toks.insert(beg, Beg.to_tok(id + 2));

                            return false;
                        }

                        true
                    })
                }
            })
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
        let mut depth: isize = 0;
        
        for tok in self.toks.iter() {
            match &tok.tok_type {
                Beg => {
                    depth += 1;
                    
                    pre_node.push(cur_node);     
                    cur_node = Node::default();
                }

                End => {
                    depth -= 1;
                    if depth < 0 {
                        panic!("too many left parentheses!");
                    }
                    
                    if let Some(mut parent) = pre_node.pop() {
                        let obj = env.gen_sym(cur_node.as_obj());
                        parent.push(obj.into());
                        cur_node = parent;
                    }
                }

                Sym(sym) => {
                    match Obj::parse_literal(sym) {
                        Some(literal) => cur_node.push(literal.into()),
                        _ => {
                            if !env.has_sym(&sym) {
                                env.add_sym(&sym, Obj::Nil(()));
                            }

                            let obj = env.get_sym(&sym).unwrap();
                            cur_node.push(Obj::Sym(obj).into()); 
                        }
                    }
                }
                
                _ => ()
            }
        }

        if depth != 0 {
            panic!("imbalanced parentheses!")
        }

        cur_node
    }

}

impl Env {
    pub fn add_from_string(&mut self, src: &str) -> Err<Obj> {
        Lexer::new(self, &src.to_string())
    }
    
    pub fn add_from_file(&mut self, path: &str) -> Err<Obj> {
        let src = std::fs::read_to_string(path.to_string()).expect("couldn't read file!");
        Lexer::new(self, &src)
    }
}