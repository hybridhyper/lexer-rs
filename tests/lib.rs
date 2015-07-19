extern crate lexer;
use lexer::{Item, StateFn, Lexer};


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


#[test]
fn test_lexer() {
    let data = "foo,bar,baz";
    let items = lexer::lex(&data, lex_text);
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
