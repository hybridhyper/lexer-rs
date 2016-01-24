pub struct StateFn<T: PartialEq>(pub fn(&mut Lexer<T>) -> Option<StateFn<T>>);


#[derive(Debug, PartialEq)]
pub struct Item<'a, T> {
    pub typ: T,
    pub val: &'a str,
    pub col: usize,
    pub lineno: usize,
}

#[derive(Clone)]
struct LexerPosition {
    raw: usize,
    col: usize,
    lineno: usize,
    is_newline: bool,
}

impl Default for LexerPosition {
    fn default() -> LexerPosition {
        LexerPosition {
            raw: 0,
            col: 1,
            lineno: 1,
            is_newline: false,
        }
    }
}

pub struct Lexer<'a, T: PartialEq> {
    input: &'a str,
    pos: LexerPosition,
    prev_pos: LexerPosition,
    start_pos: LexerPosition,
    width: usize,
    items: Vec<Item<'a, T>>
}

impl<'a, T: PartialEq> Lexer<'a, T> {
    fn new(input: &'a str) -> Lexer<'a, T> {
        Lexer {
            input: input,
            pos: LexerPosition::default(),
            prev_pos: LexerPosition::default(),
            start_pos: LexerPosition::default(),
            width: 0,
            items: Vec::new(),
        }
    }

    fn run(&mut self, start_state: fn(&mut Lexer<T>) -> Option<StateFn<T>>) {
        let mut state = start_state;

        loop {
            match state(self) {
                Some(StateFn(next)) => state = next,
                None => break,
            }
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.input[self.pos.raw..].chars().next()
            .map(|ch| {
                self.prev_pos = self.pos.clone();
                self.width = ch.len_utf8();
                self.pos.raw += self.width;
                self.pos.is_newline = ch == '\n';

                if self.pos.is_newline {
                    self.pos.lineno += 1;
                    self.pos.col = 1;
                } else {
                    self.pos.col += 1;
                }

                ch
            })
            .or_else(|| {
                self.prev_pos = self.pos.clone();
                self.width = 0;
                None
            })
    }

    pub fn ignore(&mut self) {
        self.start_pos = self.pos.clone();
    }

    pub fn peek(&mut self) -> Option<char> {
        let ch = self.next();
        self.backup();
        ch
    }

    pub fn backup(&mut self) {
        self.pos = self.prev_pos.clone();
        self.width = 0;
    }

    pub fn accept(&mut self, valid: &str) -> bool {
        match self.next() {
            Some(ch) if valid.contains(ch) => true,
            _ => {
                self.backup();
                false
            }
        }
    }

    pub fn accept_run(&mut self, valid: &str) -> bool {
        let mut found = false;
        while let Some(ch) = self.next() {
            if !valid.contains(ch) {
                break;
            };
            found = true;
        };
        self.backup();
        found
    }

    pub fn emit(&mut self, typ: T) {
        let item = Item {
            typ: typ,
            val: &self.input[self.start_pos.raw .. self.pos.raw],
            col: self.start_pos.col,
            lineno: self.start_pos.lineno,
        };
        self.items.push(item);
        self.start_pos = self.pos.clone();
    }

    pub fn emit_nonempty(&mut self, typ: T) {
        if self.pos.raw > self.start_pos.raw {
            self.emit(typ);
        }
    }

    pub fn current(&self) -> &str {
        &self.input[self.start_pos.raw .. self.pos.raw]
    }

    pub fn remaining_input(&self) -> &str {
        &self.input[self.pos.raw ..]
    }
}


pub fn lex<'a, T: PartialEq>(input: &'a str, start_state: fn(&mut Lexer<T>) -> Option<StateFn<T>>) -> Vec<Item<'a, T>> {
    let mut l = Lexer::new(input);
    l.run(start_state);

    l.items
}
