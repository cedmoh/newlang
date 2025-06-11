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
        self.stack_pointer -= 1;

        if self.stack_pointer == 0 {
            panic!("Stack underflow: trying to pop from an empty stack.");
        }

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
        self.call_stack_pointer -= 1;

        if self.call_stack_pointer == 0 {
            panic!("Call stack underflow: trying to pop from an empty call stack.");
        }

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
                // TODO: Implement with push and pop
                Instruction::Add => {
                    let a = self.stack[self.stack_pointer - 1];
                    let b = self.stack[self.stack_pointer - 2];
                    let c = a + b;

                    self.stack[self.stack_pointer - 2] = c;
                    self.stack_pointer -= 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Subtract => {
                    let a = self.stack[self.stack_pointer - 1];
                    let b = self.stack[self.stack_pointer - 2];
                    let c = b - a;

                    self.stack[self.stack_pointer - 2] = c;
                    self.stack_pointer -= 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Multiply => {
                    let a = self.stack[self.stack_pointer - 1];
                    let b = self.stack[self.stack_pointer - 2];
                    let c = a * b;

                    self.stack[self.stack_pointer - 2] = c;
                    self.stack_pointer -= 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Divide => {
                    let a = self.stack[self.stack_pointer - 1];
                    let b = self.stack[self.stack_pointer - 2];
                    let c = if a == 0 { 0 } else { b / a };

                    self.stack[self.stack_pointer - 2] = c;
                    self.stack_pointer -= 1;
                    self.instruction_pointer += 1;
                }
                Instruction::JumpIfTrue(line) => {
                    let a = self.stack[self.stack_pointer - 1];
                    if a == 1 {
                        self.instruction_pointer = line;
                    } else {
                        self.instruction_pointer += 1;
                    }
                    self.stack_pointer -= 1;
                }
                Instruction::JumpIfFalse(line) => {
                    let a = self.stack[self.stack_pointer - 1];
                    if a == 0 {
                        self.instruction_pointer = line;
                    } else {
                        self.instruction_pointer += 1;
                    }
                    self.stack_pointer -= 1;
                }
                Instruction::Dealloc(size) => {
                    self.stack_pointer -= size;
                    self.instruction_pointer += 1;
                }
                Instruction::Ref(size) => {
                    self.stack[self.stack_pointer] = self.stack[self.stack_pointer - 1 - size];
                    self.stack_pointer += 1;
                    self.instruction_pointer += 1;
                }
            }
        }

        println!("VM finished execution.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_initialization() {
        use super::Instruction::*;

        let mut vm = VM::new();

        let instructions = vec![Push(2), Push(2), Add, Debug, Halt];

        vm.run(&instructions);
    }

    // Additional tests for the VM functionality would go here
}
