#[derive(Debug)]
struct VM {
    // Length of the stack
    stack_pointer: usize,

    // Length of the call stack
    call_stack_pointer: usize,

    // Where I am in the code
    instruction_pointer: usize,

    stack: [i32; 1024],

    call_stack: [usize; 1024],
}

// stack
//
//
//   <--
// 3
// 2
// 1

// call stack
//
// foo  <--
// root

// instructions
//
// push 1
// push 2
// call foo
// ...
// ...
// push 5
// push 2
// foo:
// add
// ret

impl VM {
    fn new() -> Self {
        VM {
            stack_pointer: 0,
            instruction_pointer: 0,
            stack: [0; 1024],
            call_stack_pointer: 0,
            call_stack: [0; 1024],
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
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
                Instruction::Call(line) => {
                    self.call_stack[self.call_stack_pointer] = self.instruction_pointer;
                    self.call_stack_pointer += 1;
                    self.instruction_pointer = line;
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
            }
        }
    }
}

enum Instruction {
    Halt,              // Stop execution
    Push(i32),         // Push a value onto the stack
    Jump(usize),       // Jump to a specific instruction index
    JumpIfTrue(usize), // Jump if the top two values on the stack are equal
    Debug,             // Print the top value of the stack
    Call(usize),
    Ret,
    Add,      // Add the top two values on the stack
    Subtract, // Subtract the top value from the second top value on the stack
    Multiply, // Multiply the top two values on the stack
    Divide,   // Divide the second top value by the top value on the stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_initialization() {
        let mut vm = VM::new();

        let instructions = vec![
            Instruction::Push(2),
            Instruction::Push(2),
            Instruction::Add,
            Instruction::Debug,
            Instruction::Halt,
        ];

        vm.run(&instructions);

        assert_eq!(vm.stack_pointer, 1);
    }

    // Additional tests for the VM functionality would go here
}

fn main() {}
