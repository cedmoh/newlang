pub enum Instruction {
    Halt,        // Stop execution
    Push(i32),   // Push a value onto the stack
    Jump(usize), // Jump to a specific instruction index
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Debug, // Print the top value of the stack
    Call(usize),
    Ret,
    Add,            // Add the top two values on the stack
    Subtract,       // Subtract the top value from the second top value on the stack
    Multiply,       // Multiply the top two values on the stack
    Divide,         // Divide the second top value by the top value on the stack
    Dealloc(usize), // Deallocate a number of stack slots
    Ref(usize),     // Reference a value from the stack
}
