pub enum MathOperation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

impl MathOperation {
    pub fn from_opcode(opcode: u8) -> Option<MathOperation> {
        match opcode {
            1 => Some(MathOperation::ADD),
            2 => Some(MathOperation::SUBTRACT),
            3 => Some(MathOperation::MULTIPLY),
            4 => Some(MathOperation::DIVIDE),
            _ => None,
        }
    }

    pub fn apply(self: &MathOperation, left: i32, right: i32) -> i32 {
        match self {
            MathOperation::ADD => left + right,
            MathOperation::SUBTRACT => left - right,
            MathOperation::MULTIPLY => left * right,
            MathOperation::DIVIDE => left / right,
        }
    }
}
