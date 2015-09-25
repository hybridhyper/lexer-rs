extern crate lexer;
use lexer::{Item, StateFn, Lexer};


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
            l.emit_nonempty(ItemType::Text);
            l.next();
            return Some(StateFn(lex_comment));
        }
        match l.next() {
            None => break,
            Some(',') => {
                l.backup();
                l.emit_nonempty(ItemType::Text);
                l.next();
                l.emit(ItemType::Comma);
            },
            Some(_) => {},
        }
    }
    l.emit_nonempty(ItemType::Text);
    l.emit(ItemType::EOF);
    None
}


fn lex_comment(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    loop {
        match l.next() {
            None => break,
            Some('\n') => {
                l.emit_nonempty(ItemType::Comment);
                return Some(StateFn(lex_text));
            },
            Some(_) => {},
        }
    }
    l.emit_nonempty(ItemType::Comment);
    l.emit(ItemType::EOF);
    None
}


#[test]
fn test_lexer() {
    let data = "foo,bar,baz // some comment";
    let items = lexer::lex(data, lex_text);
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
