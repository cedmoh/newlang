use crate::Instruction;

#[derive(Debug)]
pub struct VM {
    // Length of the stack
    stack_pointer: usize,

    stack: [i32; 1024],

    // Where I am
    instruction_pointer: usize,

    // Where I have been
    call_stack: [usize; 512],

    // length of the call stack
    call_stack_pointer: usize,
}

impl VM {
    fn push(&mut self, value: i32) {
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow: trying to push onto a full stack.");
        }

        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> i32 {
        if self.stack_pointer == 0 {
            panic!("Stack underflow: trying to pop from an empty stack.");
        }

        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

    fn step(&mut self) {
        self.instruction_pointer += 1;
    }

    fn goto(&mut self, to: usize) {
        self.instruction_pointer = to;
    }

    fn push_call(&mut self, line: usize) {
        if self.call_stack_pointer >= self.call_stack.len() {
            panic!("Call stack overflow: trying to push onto a full call stack.");
        }

        self.call_stack[self.call_stack_pointer] = line;
        self.call_stack_pointer += 1;
    }

    fn pop_call(&mut self) -> usize {
        if self.call_stack_pointer == 0 {
            panic!("Call stack underflow: trying to pop from an empty call stack.");
        }

        self.call_stack_pointer -= 1;
        self.call_stack[self.call_stack_pointer]
    }

    pub fn new() -> Self {
        VM {
            instruction_pointer: 0,
            stack_pointer: 0,
            stack: [0; 1024],
            call_stack_pointer: 0,
            call_stack: [0; 512],
        }
    }

    pub fn run(&mut self, instructions: &[Instruction]) {
        while self.instruction_pointer < instructions.len() {
            match instructions[self.instruction_pointer] {
                Instruction::Halt => break,
                Instruction::Push(value) => {
                    self.push(value);
                    self.step();
                }
                Instruction::Jump(to) => {
                    self.goto(to);
                }
                Instruction::Debug => {
                    println!("stck*  = {:#?}", self.stack_pointer);
                    println!("inst*  = {:#?}", self.instruction_pointer);
                    println!("stck[] = {:#?}", &self.stack[..self.stack_pointer]);

                    self.step();
                }
                Instruction::Call(line) => {
                    // Save next line on the call stack.
                    // It has to be the next line or we'll end up in an infinite loop.
                    self.push_call(self.instruction_pointer + 1);
                    self.goto(line);
                }
                Instruction::Ret => {
                    let whence = self.pop_call();
                    self.goto(whence);
                }
                Instruction::Add => {
                    let a = self.pop();
                    let b = self.pop();
                    let c = a + b;
                    self.push(c);
                    self.step();
                }
                Instruction::Subtract => {
                    let a = self.pop();
                    let b = self.pop();
                    let c = a - b;
                    self.push(c);
                    self.step();
                }
                Instruction::Multiply => {
                    let a = self.pop();
                    let b = self.pop();
                    let c = a * b;
                    self.push(c);
                    self.step();
                }
                Instruction::Divide => {
                    let a = self.pop();
                    let b = self.pop();
                    if b == 0 {
                        panic!("Division by zero error.");
                    }
                    let c = a / b;
                    self.push(c);
                    self.step();
                }
                Instruction::JumpIfTrue(line) => {
                    let a = self.pop();
                    if a == 1 {
                        self.goto(line);
                    } else {
                        self.step();
                    }
                }
                Instruction::JumpIfFalse(line) => {
                    let a = self.pop();
                    if a == 0 {
                        self.goto(line);
                    } else {
                        self.step();
                    }
                }
                Instruction::SetPin(_, _) => todo!(),
                Instruction::Sleep(_) => todo!(),
            }
        }

        println!("VM finished execution.");
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::*;

    #[test]
    fn two_plus_two() {
        let mut vm = VM::new();

        let instructions = vec![
            // start
            Push(2),
            Push(2),
            Add,
            Halt,
        ];

        vm.run(&instructions);

        assert_eq!(vm.pop(), 4);
    }

    #[test]
    fn go_to() {
        let mut vm = VM::new();

        let instructions = vec![
            // start
            Push(10),
            Push(20),
            Add,
            Jump(5), // Jump to the end
            Push(3), // This should be skipped
            Halt,
        ];

        vm.run(&instructions);

        assert_eq!(vm.pop(), 30);
    }

    #[test]
    fn call_and_return() {
        let mut vm = VM::new();

        let instructions = vec![
            // start
            Push(2),
            Push(2),
            Call(4),
            Halt,
            Add,
            Ret, // Return to the line after the call
        ];

        vm.run(&instructions);

        assert_eq!(vm.pop(), 4);
    }

    #[test]
    fn conditional_jump() {
        let mut vm = VM::new();

        let instructions = vec![
            // start
            Push(2),
            Push(2),
            Subtract,
            JumpIfFalse(7),
            Push(402),
            Push(403),
            Push(404),
            Push(200),
            Halt,
        ];

        vm.run(&instructions);

        assert_eq!(vm.stack_pointer, 1);
        assert_eq!(vm.pop(), 200);
    }
}
