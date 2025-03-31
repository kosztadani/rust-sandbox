use std::num::Wrapping;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathRequest {
    pub operator: MathOperation,
    pub first_operand: i32,
    pub second_operand: i32,
}

impl MathRequest {
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut buffer = [0; 9];
        buffer[0] = self.operator.to_opcode();
        buffer[1..5].copy_from_slice(&self.first_operand.to_be_bytes());
        buffer[5..9].copy_from_slice(&self.second_operand.to_be_bytes());
        buffer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn to_opcode(&self) -> u8 {
        match self {
            MathOperation::ADD => 1,
            MathOperation::SUBTRACT => 2,
            MathOperation::MULTIPLY => 3,
            MathOperation::DIVIDE => 4,
        }
    }

    pub fn apply(&self, left: i32, right: i32) -> i32 {
        let wrapped_left = Wrapping(left);
        let wrapped_right = Wrapping(right);
        let wrapped_result = match self {
            MathOperation::ADD => wrapped_left + wrapped_right,
            MathOperation::SUBTRACT => wrapped_left - wrapped_right,
            MathOperation::MULTIPLY => wrapped_left * wrapped_right,
            MathOperation::DIVIDE => wrapped_left / wrapped_right,
        };
        wrapped_result.0
    }

    pub fn from_str(str: &str) -> Option<MathOperation> {
        match str {
            "+" => Some(MathOperation::ADD),
            "-" => Some(MathOperation::SUBTRACT),
            "*" => Some(MathOperation::MULTIPLY),
            "/" => Some(MathOperation::DIVIDE),
            _ => None,
        }
    }
}
