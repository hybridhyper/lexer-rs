use std::str::Chars;
use std::iter::Peekable;


pub struct StateFn<T: PartialEq>(pub fn(&mut Lexer<T>) -> Option<StateFn<T>>);


#[derive(Debug, PartialEq)]
pub struct Item<'a, T> {
    pub typ: T,
    pub val: &'a str
}


pub struct Lexer<'a, T: PartialEq> {
    input: &'a str,
    chars_iter: Peekable<Chars<'a>>,
    start: usize,
    pos: usize,
    items: Vec<Item<'a, T>>
}


impl<'a, T: PartialEq> Lexer<'a, T> {
    fn new(input: &'a str) -> Lexer<'a, T> {
        Lexer {
            input: input,
            chars_iter: input.chars().peekable(),
            start: 0,
            pos: 0,
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
        self.chars_iter.next().map(|c| {
            self.pos += 1;
            c
        })
    }

    pub fn ignore(&mut self) {
        self.start = self.pos;
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars_iter.peek().cloned()
    }

    pub fn accept(&mut self, valid: &str) -> bool {
        self.peek()
            .map_or(false, |c| if valid.contains(c) { self.next(); true } else { false })
    }

    pub fn accept_run(&mut self, valid: &str) {
        loop {
            match self.peek() {
                Some(c) => if valid.contains(c) { self.next(); } else { break; },
                None => break,
            }
        }
    }

    pub fn emit(&mut self, typ: T) {
        self.items.push(Item {
            typ: typ,
            val: &self.input[self.start .. self.pos]
        });
        self.start = self.pos;
    }

    pub fn emit_nonempty(&mut self, typ: T) {
        if self.pos > self.start {
            self.emit(typ);
        }
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
