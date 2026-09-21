use std::env::args;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    INC(u32),
    DEC(u32),
    LEFT(u32),
    RIGHT(u32),
    OUT(u32),
    IN(u32),
    JZ(usize),
    JNZ(usize),
    Nop,
}

impl Op {
    pub fn parse(source: &str) -> Vec<Self> {
        let mut ops: Vec<Op> = vec![];
        let mut stack = Stack::new();

        let mut comments = 0;
        for (i, c) in source.chars().enumerate() {
            let op = match c {
                '+' => Op::INC(1),
                '-' => Op::DEC(1),
                '<' => Op::LEFT(1),
                '>' => Op::RIGHT(1),
                '.' => Op::OUT(1),
                ',' => Op::IN(1),
                '[' => {
                    stack.push(i - comments);
                    Op::JZ(stack.len())
                }
                ']' => {
                    let el = stack.pop();
                    // Need to backpatch the [ instruction, so the destination addr is myself.
                    let _ = std::mem::replace(&mut ops[el], Op::JZ(i - comments));
                    Op::JNZ(el)
                }
                _ => {
                    /* This is a comment */
                    comments += 1;
                    Op::Nop
                }
            };
            if op != Op::Nop {
                ops.push(op);
            }
        }
        ops
    }

    #[allow(dead_code)]
    pub fn dump(ops: &[Op]) {
        let mut level = 0;
        for (i, op) in ops.iter().enumerate() {
            println!("{i:04}{}- {:?}", " ".repeat(level), op);
            match op {
                Op::JZ(_) => level += 1,
                Op::JNZ(_) => level -= 1,
                _ => {}
            }
        }
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
const MAX_MEM:usize = 1024;

struct CPU {
    memory: Mem,
    ops: Vec<Op>,
    ip: usize,
    head: usize,
}

impl CPU {
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
                Op::DEC(n) => self.memory[self.head] = self.memory[self.head].wrapping_sub(n as u8),
                Op::INC(n) => self.memory[self.head] = self.memory[self.head].wrapping_add(n as u8),
		// This might break if the program wants to access the memory before addr 0
                Op::LEFT(n) => self.head -= n as usize,
		// This might break if the program wants to access the memory after addr MAX_MEM
                Op::RIGHT(n) => self.head += n as usize,
                Op::OUT(n) => {
                    for _ in 0..n {
                        print!("{}", self.memory[self.head] as char);
                    }
                }
                Op::IN(_n) => {
                    let s = getch.getch().unwrap();
                    self.memory[self.head] = s;
                }
                Op::JZ(addr) => {
                    if self.memory[self.head] == 0 {
                        // println!("Jump to {addr}, Memory: {:?}", self.memory);
                        self.ip = addr + 1;
                        continue; // Skip the ip++
                    } else {
                        // println!("Nop, Memory: {:?}", self.memory)
                    }
                }
                Op::JNZ(addr) => {
                    if self.memory[self.head] != 0 {
                        // println!("Jump to {addr}, Memory: {:?}", self.memory);
                        self.ip = addr + 1;
                        continue;
                    } else {
                        // println!("Nop, Memory: {:?}", self.memory)
                    }
                }
                _ => {
                    panic!("Op unknown")
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
    CPU::new(ops).execute();
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
