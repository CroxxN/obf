use std::env;
use std::io::Read;
use std::fs::File;

#[derive(Debug, Clone)]
enum Token{
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart,
    LoopEnd
}

#[derive(Debug, Clone)]
enum Instruction{
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    Loop(Vec<Instruction>)
}

fn lexer(source: String) -> Vec<Token>{
    let mut lexed_token: Vec<Token> = Vec::new();
    for character in source.chars(){
        let token = match character {
            '>' => Some(Token::IncrementPointer),
            '<' => Some(Token::DecrementPointer),
            '+' => Some(Token::IncrementValue),
            '-' => Some(Token::DecrementValue),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None
        };

        match token {
            Some(token) => lexed_token.push(token),
            _ => {}
        }
    }
    lexed_token
}


fn parser(token: Vec<Token>)-> Vec<Instruction>{
    let mut parsed: Vec<Instruction> = Vec::new();
    let mut stack = 0;
    let mut start = 0;

    for (index, tok) in token.iter().enumerate(){
        if stack == 0{
            let parsed_token = match tok{
                Token::IncrementPointer => Some(Instruction::IncrementPointer),
                Token::DecrementPointer => Some(Instruction::DecrementPointer),
                Token::IncrementValue => Some(Instruction::IncrementValue),
                Token::DecrementValue => Some(Instruction::DecrementValue),
                Token::Output => Some(Instruction::Output),
                Token::Input => Some(Instruction::Input),
                Token::LoopStart => {
                    start = index;
                    stack += 1;
                    None
                },
                
                Token::LoopEnd => panic!("Missing loop start at Token {}", index),
            };

            match parsed_token {
                Some(parsed_token) => parsed.push(parsed_token),
                _ => ()
            }
               
        }
        else {
            match tok{
                Token::LoopStart => {
                    stack+=1;
                },
                Token::LoopEnd => {
                    stack-=1;

                    if stack == 0 {
                        parsed.push(Instruction::Loop(parser(token[start+1..index].to_vec())));
                    }
                },
                _ => ()
            }
        }

    }
    if stack!= 0 {
        panic!("Missing end loop at Token {}", start);
    }
    parsed
}


fn interpret(parsed: &Vec<Instruction>, finite_tape: &mut Vec<u8>, byte: &mut usize){
    for instruct in parsed {
        match instruct {
            Instruction::IncrementPointer => *byte +=1,
            Instruction::DecrementPointer => *byte -=1,
            Instruction::IncrementValue => finite_tape[*byte]+=1,
            Instruction::DecrementValue => finite_tape[*byte]-=1,
            Instruction::Output => print!("{}", finite_tape[*byte] as char),
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to parse input");
                finite_tape[*byte] = input[0];
            },
            Instruction::Loop(loop_instruct) => {
                while finite_tape[*byte] != 0 {
                    interpret(&loop_instruct, finite_tape, byte)
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let mut content = String::new();

     let file = &args[1];
    
    let mut opened_file = File::open(file).expect("Could not open file");
    opened_file.read_to_string(&mut content).expect("Could not read file");

    let lexed = lexer(content);
    let parsed = parser(lexed);

    let mut finite_tape: Vec<u8> = vec![0; 1024];
    let mut byte: usize = 512;

    interpret(&parsed, &mut finite_tape, &mut byte);
}
