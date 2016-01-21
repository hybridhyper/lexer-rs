extern crate lexer;
use std::io;
use std::io::Write;
use lexer::{Item, StateFn, Lexer};

const NUMBER_SIGNS: &'static str = "+-";
const NUMBERS: &'static str = "0123456789";
const DECIMAL_SIGN: &'static str = ".";
const NEGATE_SIGN: char = '-';

#[derive(Debug, PartialEq)]
pub enum ItemType {
    Number,
    GroupStart,
    GroupEnd,
    OperatorPlus,
    OperatorMinus,
    OperatorMul,
    OperatorDiv,
    OperatorNegate,
}

fn ignore_whitespace(l: &mut Lexer<ItemType>) -> Option<()> {
    loop {
        match l.next() {
            None => return None,
            Some(ch) if ch.is_whitespace() => l.ignore(),
            Some(_) => {
                l.backup();
                break;
            }
        }
    }
    Some(())
}

fn lex_binary_op(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    loop {
        match l.next() {
            None => return None,
            Some(ch) if ch.is_whitespace() => l.ignore(),
            Some(')') => l.emit(ItemType::GroupEnd),
            Some(ch) => {
                match ch {
                    '+' => l.emit(ItemType::OperatorPlus),
                    '-' => l.emit(ItemType::OperatorMinus),
                    '*' => l.emit(ItemType::OperatorMul),
                    '/' => l.emit(ItemType::OperatorDiv),
                    _ => panic!("unexpected: {}, expected operator", ch)
                }
                break;
            },
        }
    }

    Some(StateFn(lex_group_or_number))
}

fn lex_group_or_number(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    loop {
        match l.next() {
            None => return None,
            Some(ch) if ch.is_whitespace() => l.ignore(),
            Some(ch) if ch == NEGATE_SIGN => {
                if l.peek().map_or(false, |ch| NUMBERS.contains(ch)) {
                    l.backup();
                    return Some(StateFn(lex_number));
                }
                l.emit(ItemType::OperatorNegate);
            },
            Some('(') => {
                l.emit(ItemType::GroupStart);
            },
            Some(')') => {
                l.emit(ItemType::GroupEnd);
                return Some(StateFn(lex_binary_op));
            },
            Some(_) => {
                l.backup();
                return Some(StateFn(lex_number));
            }
        }
    }
}

fn lex_number(l: &mut Lexer<ItemType>) -> Option<StateFn<ItemType>> {
    if let None = ignore_whitespace(l) {
        return None;
    }

    l.accept(NUMBER_SIGNS);

    if !l.accept_run(NUMBERS) {
        l.backup();
        panic!("unexpected: {:?}, expected number", l.next());
    }

    if l.accept(DECIMAL_SIGN) {
        l.accept_run(NUMBERS);
    }

    l.emit(ItemType::Number);

    Some(StateFn(lex_binary_op))
}

fn evaluate_group(items: &[Item<ItemType>]) -> f64 {
    let end_pos = {
        let mut inner_groups = 0usize;
        let mut pos = None;

        for (idx, item) in items.iter().enumerate() {
            inner_groups = match item.typ {
                ItemType::GroupStart => inner_groups + 1,
                ItemType::GroupEnd if inner_groups > 0 => inner_groups - 1,
                ItemType::GroupEnd if inner_groups == 0 => {
                    pos = Some(idx);
                    break;
                }
                _ => inner_groups,
            }
        }
        pos
    }.expect("unclosed group");

    evaluate_binary_op(&items[end_pos+1..], evaluate(&items[..end_pos]))
}

fn evaluate_binary_op(items: &[Item<ItemType>], op_a: f64) -> f64 {
    match items.len() {
        0 => op_a,
        n if n >=2 => {
            let op_b = evaluate(&items[1..]);
            apply_binary_op(&items[0], op_a, op_b)
        }
        _ => panic!("malformed operator")
    }
}

fn apply_binary_op(item: &Item<ItemType>, op_a: f64, op_b: f64) -> f64 {
    match item.typ {
        ItemType::OperatorPlus => op_a + op_b,
        ItemType::OperatorMinus => op_a - op_b,
        ItemType::OperatorMul => op_a * op_b,
        ItemType::OperatorDiv => op_a / op_b,
        _ => panic!("invalid operator item: {:?}", item),
    }
}

fn evaluate(items: &[Item<ItemType>]) -> f64 {
    if items.len() == 0 {
        return 0.0
    }
    let item = &items[0];

    match item.typ {
        ItemType::GroupStart => evaluate_group(&items[1..]),
        ItemType::OperatorNegate => -evaluate(&items[1..]),
        ItemType::Number => {
            let op_a = item.val.parse().unwrap();
            evaluate_binary_op(&items[1..], op_a)
        },
        _ => panic!("unable to evaluate: {:?}", item),
    }
}

fn main() {
    loop {
        println!("Input an expression or q to quit");
        print!(">>> ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let input = {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).ok().expect("read error");
            buffer.trim().to_owned()
        };

        if input == "q" {
            return
        }

        let items = lexer::lex(&input, lex_group_or_number);

        if items.len() == 0 {
            continue;
        }

        println!("{}", evaluate(&items));
    }
}

#[test]
fn test_parser() {
    let data = "-( 123 + 2 * 3 ) * 4 + -5 * -(5+6)";
    let expected_items = vec!(
        Item { typ: ItemType::OperatorNegate, val: "-" },
        Item { typ: ItemType::GroupStart, val: "(" },
        Item { typ: ItemType::Number, val: "123" },
        Item { typ: ItemType::OperatorPlus, val: "+" },
        Item { typ: ItemType::Number, val: "2" },
        Item { typ: ItemType::OperatorMul, val: "*" },
        Item { typ: ItemType::Number, val: "3" },
        Item { typ: ItemType::GroupEnd, val: ")" },
        Item { typ: ItemType::OperatorMul, val: "*" },
        Item { typ: ItemType::Number, val: "4" },
        Item { typ: ItemType::OperatorPlus, val: "+" },
        Item { typ: ItemType::Number, val: "-5" },
        Item { typ: ItemType::OperatorMul, val: "*" },
        Item { typ: ItemType::OperatorNegate, val: "-" },
        Item { typ: ItemType::GroupStart, val: "(" },
        Item { typ: ItemType::Number, val: "5" },
        Item { typ: ItemType::OperatorPlus, val: "+" },
        Item { typ: ItemType::Number, val: "6" },
        Item { typ: ItemType::GroupEnd, val: ")" },
    );
    let items = lexer::lex(data, lex_group_or_number);
    println!("data: {:?}", data);
    println!("items: {:?}", items);
    assert_eq!(expected_items, items);
}
