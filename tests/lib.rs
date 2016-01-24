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
                l.backup();
                l.emit_nonempty(ItemType::Comment);
                l.next();
                l.ignore();

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
    let data = "foo,bar,baz // some comment\nfoo,bar";
    let items = lexer::lex(data, lex_text);
    let expected_items = vec!(
        Item{typ: ItemType::Text, val: "foo", col: 1, lineno: 1},
        Item{typ: ItemType::Comma, val: ",", col: 4, lineno: 1},
        Item{typ: ItemType::Text, val: "bar", col: 5, lineno: 1},
        Item{typ: ItemType::Comma, val: ",", col: 8, lineno: 1},
        Item{typ: ItemType::Text, val: "baz ", col: 9, lineno: 1},
        Item{typ: ItemType::Comment, val: "// some comment", col: 13, lineno: 1},
        Item{typ: ItemType::Text, val: "foo", col: 1, lineno: 2},
        Item{typ: ItemType::Comma, val: ",", col: 4, lineno: 2},
        Item{typ: ItemType::Text, val: "bar", col: 5, lineno: 2},
        Item{typ: ItemType::EOF, val: "", col: 8, lineno: 2}
    );

    for (item, expected) in items.iter().zip(expected_items.iter()) {
        println!("ITEM: {:?}", item);
        assert_eq!(item, expected);
    }

    assert_eq!(items, expected_items);
}
