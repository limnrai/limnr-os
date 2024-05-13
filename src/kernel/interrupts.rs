// different types of interrupts
pub enum Interrupt {
    Timer,
    Keyboard,
    // TODO: more interrupt types
}


// Define a trait for handling interrupts
pub trait InterruptHandler {
    // Method to handle interrupts
    fn handle_interrupt(&self, interrupt: Interrupt);
}

// TODO: interrupt handling logic

