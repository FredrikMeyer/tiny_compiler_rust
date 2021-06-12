use std::{
    io::{stdin, BufRead, BufWriter, Write},
    println,
};

mod tokenizer;

fn read_from_stdin<R>(mut reader: R) -> String
where
    R: BufRead,
{
    let mut s = String::new();
    for line in reader.lines() {
        match line {
            Ok(okstring) => s.push_str(&okstring),
            Err(_) => panic!(),
        }
    }
    s
}

fn main() {
    let stdio = stdin();
    let input = stdio.lock();

    let input = read_from_stdin(input);

    let lexer = tokenizer::Lexer::new(&input);

    println!("Hello, world!");
}

#[test]
fn test_read_input() {
    let input = b"Hey!";

    let res = read_from_stdin(&input[..]);

    assert_eq!(res, String::from_utf8(input.to_vec()).expect("Not UTF-8"))
}
