#[derive(Debug)]
pub enum Instruction {
    // Stop execution
    Halt,
    // Jumpt to a specific instruction index while keeping the current address in the call stack
    Call(usize),
    // Go back to the call site by popping off of the call stack
    Ret,
    // Push a value onto the stack
    Push(i32),
    // Add the top two values on the stack
    Add,
    // Subtract the top value from the second top value on the stack
    Subtract,
    // Multiply the top two values on the stack
    Multiply,
    // Divide the second top value by the top value on the stack
    Divide,
    // Print the top value of the stack
    Debug,
    // Jump to a specific instruction index
    Jump(usize),
    // Jump to a specific instruction index if the top of the stack is 1
    JumpIfTrue(usize),
    // Jump to a specific instruction index if the top of the stack is 0
    JumpIfFalse(usize),

    // Set a pin to High (1) or Low (0)
    SetPin(u8, i32),

    // Sleep for the given amount of time
    Sleep(u32),
}
