use crate::Instruction;

#[derive(Debug)]
pub struct VM {
    // Length of the stack
    stack_pointer: usize,

    // Length of the call stack
    call_stack_pointer: usize,

    // Where I am in the code
    instruction_pointer: usize,

    stack: [i32; 1024],

    call_stack: [usize; 1024],
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack_pointer: 0,
            instruction_pointer: 0,
            stack: [0; 1024],
            //
            call_stack_pointer: 0,
            call_stack: [0; 1024],
        }
    }

    pub fn run(&mut self, instructions: &[Instruction]) {
        // This is where the main loop of the VM would go
        // It would read instructions from a bytecode file and execute them
        loop {
            if self.instruction_pointer >= instructions.len() {
                println!("Reached end of instructions.");
                return;
            }

            let current_instruction = self.instruction_pointer;

            match instructions[current_instruction] {
                Instruction::Halt => return,
                Instruction::Push(value) => {
                    self.stack[self.stack_pointer] = value;
                    self.stack_pointer += 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Jump(to) => {
                    self.instruction_pointer = to;
                }
                Instruction::Debug => {
                    println!("stck_ptr  = {:#?}", self.stack_pointer);
                    println!("cstck_ptr = {:#?}", self.call_stack_pointer);
                    println!("inst_ptr  = {:#?}", self.instruction_pointer);
                    println!("stck      = {:?}", &self.stack[..self.stack_pointer]);
                    println!(
                        "cstck     = {:?}",
                        &self.call_stack[..self.call_stack_pointer]
                    );
                    self.instruction_pointer += 1;
                }
                Instruction::Call(line_on_stack) => {
                    self.call_stack[self.call_stack_pointer] = self.instruction_pointer + 1;
                    self.call_stack_pointer += 1;
                    self.instruction_pointer = line_on_stack;
                }
                Instruction::Ret => {
                    self.call_stack_pointer -= 1;
                    self.instruction_pointer = self.call_stack[self.call_stack_pointer];
                }
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
