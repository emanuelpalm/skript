/// `HALT() -> (program exits)`
pub const OP_HALT: u8 = 0x00;

/// `LOAD_I(A, Bx) -> R[A] := ((Bx as i32) << 14) >> 14`
pub const OP_LOAD_I: u8 = 0x01;

/// `ADD(A, B, C) -> R[A] := R[B] + R[C]`
pub const OP_ADD: u8 = 0x20;

/// `SUB(A, B, C) -> R[A] := R[B] - R[C]`
pub const OP_SUB: u8 = 0x24;

/// `MUL(A, B, C) -> R[A] := R[B] * R[C]`
pub const OP_MUL: u8 = 0x28;

/// `DIV(A, B, C) -> R[A] := R[B] / R[C]`
pub const OP_DIV: u8 = 0x2C;
