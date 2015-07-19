extern crate lexer;
use lexer::{Item, StateFn, Lexer};


const DATA: &'static str = "foo,bar,baz // some comment";


#[derive(Debug, PartialEq)]
pub enum ItemType {
    Comma,
    Text,
    Comment,
    EOF
}


fn lex_text(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    loop {
        if l.remaining_input().starts_with("//") {
            l.finish_started(ItemType::Text);
            l.next();
            return Some(StateFn(lex_comment));
        }
        match l.peek() {
            None => break,
            Some(',') => {
                l.finish_started(ItemType::Text);
                l.next();
                l.emit(ItemType::Comma);
            },
            Some(_) => {},
        }
        l.next();
    }
    l.finish_started(ItemType::Text);
    l.emit(ItemType::EOF);
    None
}


fn lex_comment(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    loop {
        match l.peek() {
            None => break,
            Some('\n') => {
                l.finish_started(ItemType::Comment);
                l.next();
                return Some(StateFn(lex_text));
            },
            Some(_) => {},
        }
        l.next();
    }
    l.finish_started(ItemType::Comment);
    l.emit(ItemType::EOF);
    None
}


#[test]
fn test_lexer() {
    let items = lexer::lex(DATA, lex_text);
    let expected_items = vec!(
        Item{typ: ItemType::Text, val: "foo"},
        Item{typ: ItemType::Comma, val: ","},
        Item{typ: ItemType::Text, val: "bar"},
        Item{typ: ItemType::Comma, val: ","},
        Item{typ: ItemType::Text, val: "baz "},
        Item{typ: ItemType::Comment, val: "// some comment"},
        Item{typ: ItemType::EOF, val: ""}
    );

    assert_eq!(items, expected_items);
}
