pub struct StateFn<T: PartialEq>(pub fn(&mut Lexer<T>) -> Option<StateFn<T>>);


#[derive(Debug, PartialEq)]
pub struct Item<'a, T> {
    pub typ: T,
    pub val: &'a str
}


pub struct Lexer<'a, T: PartialEq> {
    input: &'a str,
    start: usize,
    pos: usize,
    width: usize,
    items: Vec<Item<'a, T>>
}


impl<'a, T: PartialEq> Lexer<'a, T> {
    fn new(input: &'a str) -> Lexer<'a, T> {
        Lexer {
            input: input,
            start: 0,
            pos: 0,
            width: 0,
            items: Vec::new()
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
        self.input[self.pos..].chars().next()
            .map(|ch| {
                self.width = ch.len_utf8();
                self.pos += self.width;
                ch
            })
            .or_else(|| {
                self.width = 0;
                None
            })
    }

    pub fn ignore(&mut self) {
        self.start = self.pos;
    }

    pub fn peek(&mut self) -> Option<char> {
        let ch = self.next();
        self.backup();
        ch
    }

    pub fn backup(&mut self) {
        self.pos -= self.width;
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
            val: &self.input[self.start .. self.pos]
        };
        self.items.push(item);
        self.start = self.pos;
    }

    pub fn emit_nonempty(&mut self, typ: T) {
        if self.pos > self.start {
            self.emit(typ);
        }
    }

    pub fn current(&self) -> &str {
        &self.input[self.start .. self.pos]
    }

    pub fn remaining_input(&self) -> &str {
        &self.input[self.pos ..]
    }
}


pub fn lex<'a, T: PartialEq>(input: &'a str, start_state: fn(&mut Lexer<T>) -> Option<StateFn<T>>) -> Vec<Item<'a, T>> {
    let mut l = Lexer::new(input);
    l.run(start_state);

    l.items
}
