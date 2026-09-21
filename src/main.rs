use std::env::args;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Inc(u32),
    Dec(u32),
    Left(u32),
    Right(u32),
    Out(u32),
    In(u32),
    Jz(usize),
    Jnz(usize),
}

impl Op {
    pub fn parse(source: &str) -> Vec<Self> {
        let mut ops: Vec<Op> = vec![];
        let mut stack = Stack::new();

        let mut _comments = 0;
        let mut operand = 0;
        let mut last_op = None;
        let mut i = 0;
        for c in source.chars() {
            let op = match c {
                '+' => Some(Op::Inc(0)),
                '-' => Some(Op::Dec(0)),
                '<' => Some(Op::Left(0)),
                '>' => Some(Op::Right(0)),
                '.' => Some(Op::Out(0)),
                ',' => Some(Op::In(0)),
                '[' => {
                    stack.push(i + 1);
                    Some(Op::Jz(stack.len()))
                }
                ']' => {
                    let el = stack.pop();
                    // Need to backpatch the [ instruction, so the destination addr is myself.
                    let _ = std::mem::replace(&mut ops[el], Op::Jz(i));
                    Some(Op::Jnz(el))
                }
                _ => {
                    /* This is a comment */
                    _comments += 1;
                    None
                }
            };
            if let Some(op) = op {
                if last_op.is_none()
                    || std::mem::discriminant(&last_op.clone().unwrap())
                        == std::mem::discriminant(&op)
                {
                    last_op = Some(op);
                } else {
                    ops.push(create_instruction(operand, &last_op));
                    operand = 0;
                    i += 1;
                    last_op = Some(op);
                }
                operand += 1;
            }
        }
        ops.push(create_instruction(operand, &last_op));
        ops
    }

    #[allow(dead_code)]
    pub fn dump(ops: &[Op]) {
        let mut level = 0;
        for (i, op) in ops.iter().enumerate() {
            println!("{i:04}{}- {:?}", " ".repeat(level), op);
            match op {
                Op::Jz(_) => level += 1,
                Op::Jnz(_) => level -= 1,
                _ => {}
            }
        }
    }
}

fn create_instruction(operand: u32, last_op: &Option<Op>) -> Op {
    match last_op.clone().unwrap() {
        Op::Inc(_) => Op::Inc(operand),
        Op::Dec(_) => Op::Dec(operand),
        Op::Left(_) => Op::Left(operand),
        Op::Right(_) => Op::Right(operand),
        Op::In(_) => Op::In(operand),
        Op::Out(_) => Op::Out(operand),
        _ => last_op.clone().unwrap(),
    }
}

struct Stack {
    elements: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    pub fn push(&mut self, el: usize) {
        self.elements.push(el);
    }
    pub fn pop(&mut self) -> usize {
        self.elements.pop().expect("Unbalanced stack")
    }
}

type Mem = Vec<u8>;
const MAX_MEM: usize = 1024;

struct Cpu {
    memory: Mem,
    ops: Vec<Op>,
    ip: usize,
    head: usize,
}

impl Cpu {
    pub fn new(ops: Vec<Op>) -> Self {
        Self {
            memory: vec![0; MAX_MEM],
            ops,
            ip: 0,
            head: 0,
        }
    }

    pub fn execute(&mut self) {
        let getch = getch::Getch::new();
        loop {
            // println!("Executing: {:?}", self.ops[self.ip]);
            match self.ops[self.ip] {
                Op::Dec(n) => self.memory[self.head] = self.memory[self.head].wrapping_sub(n as u8),
                Op::Inc(n) => self.memory[self.head] = self.memory[self.head].wrapping_add(n as u8),
                // This might break if the program wants to access the memory before addr 0
                Op::Left(n) => self.head -= n as usize,
                // This might break if the program wants to access the memory after addr MAX_MEM
                Op::Right(n) => self.head += n as usize,
                Op::Out(n) => {
                    for _ in 0..n {
                        print!("{}", self.memory[self.head] as char);
                    }
                }
                Op::In(n) => {
                    for _ in 0..n {
                        let s = getch.getch().unwrap();
                        self.memory[self.head] = s;
                    }
                }
                Op::Jz(addr) => {
                    if self.memory[self.head] == 0 {
                        // println!("Jump to {addr}, Memory: {:?}", self.memory);
                        self.ip = addr + 1;
                        continue; // Skip the ip++
                    } else {
                        // println!("Nop, Memory: {:?}", self.memory)
                    }
                }
                Op::Jnz(addr) => {
                    if self.memory[self.head] != 0 {
                        // println!("Jump to {addr}, Memory: {:?}", self.memory);
                        self.ip = addr + 1;
                        continue;
                    } else {
                        // println!("Nop, Memory: {:?}", self.memory)
                    }
                }
            };
            self.ip += 1;
            if self.ip >= self.ops.len() {
                break;
            }
        }

        println!();
        println!("Brainfuck program is over.")
    }
}

mod tests {
    #[test]
    fn test_10() {
        crate::exec_source(format!("{}.[-]{}.", "+".repeat(0x31), "+".repeat(0x30)).as_str());
    }

    #[test]
    pub fn hello() {
        crate::exec_source(
            "
        +++++ +++++             initialize counter (cell #0) to 10
        [                       use loop to set the next four cells to 70/100/30/10
            > +++++ ++              add  7 to cell #1
            > +++++ +++++           add 10 to cell #2
            > +++                   add  3 to cell #3
            > +                     add  1 to cell #4
            <<<< -                  decrement counter (cell #0)
        ]
        > ++ .                  print 'H'
        > + .                   print 'e'
        +++++ ++ .              print 'l'
        .                       print 'l'
        +++ .                   print 'o'
        > ++ .                  print ' '
        << +++++ +++++ +++++ .  print 'W'
        > .                     print 'o'
        +++ .                   print 'r'
        ----- - .               print 'l'
        ----- --- .             print 'd'
        > + .                   print '!'
        > .                     print '\n'",
        );
    }

    #[test]
    fn five() {
        crate::exec_source(
            "
        +++++           +++++
            +               +
            +     +         +     +++++
        +++++    +++    +++++
        +         +     +         +++++
        +               +
        +++++           +++++.
        ",
        );
    }

    #[test]
    fn quine() {
        crate::exec_source(
            "
        -->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+
        ++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++
        >++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>
        >>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>
        +++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]
        >>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+
        ++++++++++++++<-]>--.<<<]
        ",
        );
    }

    #[test]
    fn cat() {
        crate::exec_source(",[.,]");
    }
}

pub fn exec_source(source: &str) {
    let ops = Op::parse(source);
    if ops.is_empty() {
        eprintln!("no source code provided");
        return;
    }
    #[cfg(debug_assertions)]
    Op::dump(&ops);
    Cpu::new(ops).execute();
}

fn main() {
    if args().len() == 1 {
        exec_source(
            ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.",
        );
    } else {
        for arg in args().skip(1) {
            exec_source(arg.as_str());
        }
    }
}
