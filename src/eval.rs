use crate::parser::Statement;
use rand::Rng;
use std::io::{Read, Write};
use std::num::Wrapping;

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Purple,
    Green,
    Yellow,
    Cyan,
    Black,
    White,
    Brown,
    Lime,
    Pink,
    Orange,
}

#[derive(Default, Debug)]
struct Memory {
    acc1: Wrapping<u8>,
    acc2: Wrapping<u8>,
    color: Option<Color>,
    stack: Vec<u8>,
}

pub(crate) fn eval(statements: &[Statement]) {
    inner_eval(statements, &mut Memory::default());
}

fn inner_eval(statements: &[Statement], memory: &mut Memory) {
    for statement in statements {
        match statement {
            Statement::Sus => match memory.color {
                Some(Color::Red) => memory.acc1 += Wrapping(1),
                Some(Color::Blue) => memory.stack.push(memory.acc1.0),
                Some(Color::Purple) => {
                    memory.stack.pop();
                }
                Some(Color::Green) => {
                    print!("{}", *memory.stack.last().unwrap() as char);
                    std::io::stdout().flush().unwrap();
                }
                Some(Color::Yellow) => {
                    let mut buf = [0];
                    match std::io::stdin().read_exact(&mut buf) {
                        Ok(_) => memory.stack.push(buf[0]),
                        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {}
                        Err(_) => todo!(),
                    };
                }
                Some(Color::Cyan) => {
                    for _ in 0..(rand::thread_rng().gen_range(0..=memory.acc1.0)) {
                        memory.stack.pop();
                    }
                }
                Some(Color::Black) => {
                    print!("{}", memory.stack.last().unwrap());
                    std::io::stdout().flush().unwrap();
                }
                Some(Color::White) => memory.acc1 -= Wrapping(1),
                Some(Color::Brown) => memory.acc1 = Wrapping(*memory.stack.last().unwrap()),
                Some(Color::Lime) => *memory.stack.last_mut().unwrap() *= 2,
                Some(Color::Pink) => memory.acc1 = Wrapping(0),
                Some(Color::Orange) => memory.acc1 += Wrapping(10),
                None => todo!(),
            },
            Statement::Vented => memory.acc2 += Wrapping(10),
            Statement::Sussy => memory.acc2 -= Wrapping(1),
            Statement::Electrical => memory.acc2 = Wrapping(0),
            Statement::Block(s) => {
                while *memory.stack.last().unwrap() != memory.acc2.0 {
                    inner_eval(s, memory);
                }
            }
            Statement::Red => memory.color = Some(Color::Red),
            Statement::Blue => memory.color = Some(Color::Blue),
            Statement::Purple => memory.color = Some(Color::Purple),
            Statement::Green => memory.color = Some(Color::Green),
            Statement::Yellow => memory.color = Some(Color::Yellow),
            Statement::Cyan => memory.color = Some(Color::Cyan),
            Statement::Black => memory.color = Some(Color::Black),
            Statement::White => memory.color = Some(Color::White),
            Statement::Brown => memory.color = Some(Color::Brown),
            Statement::Lime => memory.color = Some(Color::Lime),
            Statement::Pink => memory.color = Some(Color::Pink),
            Statement::Orange => memory.color = Some(Color::Orange),
        }
    }
}
