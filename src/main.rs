use std::env;
use std::io::prelude::Read;
use std::fs::OpenOptions;

struct Program {
    tape: Vec<u8>,
    code: Vec<u8>,
    codepos: usize,
    tapepos: usize
}

impl Program {
    fn new(code_: Vec<u8>) -> Self {
        Program {
            tape: Vec::new(),
            code: code_,
            codepos: 0,
            tapepos: 0
        }
    }

    fn run(&mut self, skip: bool) -> bool {
        while self.codepos < self.code.len() {
            if self.tapepos >= self.tape.len() {
                self.tape.push('\0' as u8); // increase tape size, because in B.F. tape is infinite!
            }

            if self.code[self.codepos] as char == '[' {
                self.codepos += 1;
                let oldpos = self.codepos;
                let tmp = self.tape[self.tapepos] == ('\0' as u8);
                while self.run(tmp) {
                    self.codepos = oldpos;
                }
            } else if self.code[self.codepos] as char == ']' {
                return self.tape[self.tapepos] != ('\0' as u8);
            } else if !skip {
                match self.code[self.codepos] as char {
                    '+' => self.tape[self.tapepos] = self.tape[self.tapepos].wrapping_add(1), // inc
                    // by default, rust panics in case of an overflow. by using this method, we avoid that.
                    '-' => self.tape[self.tapepos] = self.tape[self.tapepos].wrapping_sub(1), // dec
                    '>' => self.tapepos += 1,
                    '<' => self.tapepos -= 1,
                    '.' => print!("{}", self.tape[self.tapepos] as char),
                    ',' => {
                        let mut buf = [0u8];
                        std::io::stdin().read(&mut buf).unwrap();
                        self.tape[self.tapepos] = buf[0];
                    }
                    _   => {
                        // no-op
                    }
                }
            }


            self.codepos += 1;
        }
        false
    }
}

fn main() {
    let mut args = env::args();
    let mut program;
    let mut code = String::new();
    let (size, _) = args.size_hint();
    if size > 1 {
        args.next(); // pop name of the program.
        let mut file = match OpenOptions::new().read(true).open(args.next().expect("You should not see this message!")) {
            Ok(file) => file,
            Err(_)   => panic!("Could not open the file."),
        };
        file.read_to_string(&mut code).unwrap();;
    } else {
        println!("You did not provide a file name.");
        println!("Write your program here and then \"ctrl+d\":");
        std::io::stdin().read_to_string(&mut code).unwrap();
        println!("");
    }

    program = Program::new(code.into());
    program.run(false);
}
