use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub struct Item<'a, T> {
    typ: T,
    val: &'a str
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

    fn next(&mut self) -> Option<char> {
        self.chars_iter.next().map(|c| {
            self.pos += 1;
            c
        })
    }

    fn ignore(&mut self) {
        self.start = self.pos;
    }

    fn peek(&mut self) -> Option<char> {
        self.chars_iter.peek().cloned()
    }

    fn accept(&mut self, valid: &str) -> bool {
        self.peek()
            .map_or(false, |c| if valid.contains(c) { self.next(); true } else { false })
    }

    fn accept_run(&mut self, valid: &str) {
        loop {
            match self.peek() {
                Some(c) => if valid.contains(c) { self.next(); } else { break; },
                None => break,
            }
        }
    }

    fn emit(&mut self, typ: T) {
        self.items.push(Item {
            typ: typ,
            val: &self.input[self.start .. self.pos]
        });
        self.start = self.pos;
    }
}

pub fn lex<'a, T: PartialEq>(input: &'a str, start_state: fn(&mut Lexer<T>) -> Option<StateFn<T>>) -> Vec<Item<'a, T>> {
    let mut l = Lexer::new(input);
    l.run(start_state);

    l.items
}

pub struct StateFn<T: PartialEq>(fn(&mut Lexer<T>) -> Option<StateFn<T>>);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ItemType {
        Comma,
        Text,
        EOF
    }

    fn lex_text(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
        loop {
            match l.peek() {
                None => break,
                Some(',') => {
                    l.emit(ItemType::Text);
                    l.next();
                    l.emit(ItemType::Comma);
                },
                Some(_) => {},
            }
            l.next();
        }
        if l.pos > l.start {
            l.emit(ItemType::Text);
        }
        l.emit(ItemType::EOF);
        None
    }

    #[test]
    fn test_lexer() {
        let data = "foo,bar,baz";
        let items = lex(&data, lex_text);
        let expected_items = vec!(
            Item{typ: ItemType::Text, val: "foo"},
            Item{typ: ItemType::Comma, val: ","},
            Item{typ: ItemType::Text, val: "bar"},
            Item{typ: ItemType::Comma, val: ","},
            Item{typ: ItemType::Text, val: "baz"},
            Item{typ: ItemType::EOF, val: ""}
        );

        assert_eq!(items, expected_items);
    }
}
