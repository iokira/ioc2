use crate::token::Int;
use core::fmt;

pub enum Register {
    /// rax
    R0,
    /// rdi
    R1,
    /// rbp
    R5,
    /// rsp
    R6,
}

pub enum Operand {
    Num(Int),
    Register(Register),
    Address(Register),
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Register::R0 => "rax",
            Register::R1 => "rdi",
            Register::R5 => "rbp",
            Register::R6 => "rsp",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: String = match self {
            Operand::Register(r) => r.to_string(),
            Operand::Address(r) => format!("[{}]", r),
            Operand::Num(n) => n.to_string(),
        };
        write!(f, "{}", name)
    }
}

pub fn program_prologue() -> String {
    ".intel_syntax noprefix\n".to_owned()
}

pub fn main_func() -> String {
    ".globl main\nmain:\n".to_owned()
}

/// push rbp
/// mov rbp, rsp
/// sub rsp, #bytes
pub fn memory_allocate(bytes: usize) -> String {
    format!(
        "{}{}{}",
        push(Operand::Register(Register::R5)),
        mov(
            Operand::Register(Register::R5),
            Operand::Register(Register::R6)
        ),
        sub(Operand::Register(Register::R6), Operand::Num(bytes))
    )
}

pub fn stmt_epilogue() -> String {
    pop(Operand::Register(Register::R0))
}

/// mov rsp, rbp
/// pop rbp
/// ret
pub fn program_epilogue() -> String {
    format!(
        "{}{}{}",
        mov(
            Operand::Register(Register::R6),
            Operand::Register(Register::R5)
        ),
        pop(Operand::Register(Register::R5)),
        ret()
    )
}

/// mov rax, rbp
/// sub rax, offset
/// push rax
pub fn gen_val(offset: usize) -> String {
    format!(
        "{}{}{}",
        mov(
            Operand::Register(Register::R0),
            Operand::Register(Register::R5)
        ),
        sub(Operand::Register(Register::R0), Operand::Num(offset)),
        push(Operand::Register(Register::R0))
    )
}

/// pop r0
/// mov r0, [r0]
/// push r0
pub fn pop_val() -> String {
    format!(
        "{}{}{}",
        pop(Operand::Register(Register::R0)),
        mov(
            Operand::Register(Register::R0),
            Operand::Address(Register::R0)
        ),
        push(Operand::Register(Register::R0))
    )
}

/// pop r1
/// pop r0
/// mov [r0], r1
/// push r1
pub fn pop_lvar() -> String {
    format!(
        "{}{}{}{}",
        pop(Operand::Register(Register::R1)),
        pop(Operand::Register(Register::R0)),
        mov(
            Operand::Address(Register::R0),
            Operand::Register(Register::R1)
        ),
        push(Operand::Register(Register::R1))
    )
}

/// pop r1
/// pop r0
pub fn pop_arg() -> String {
    format!(
        "{}{}",
        pop(Operand::Register(Register::R1)),
        pop(Operand::Register(Register::R0))
    )
}

/// push rd
pub fn push(rd: Operand) -> String {
    format!("\tpush {}\n", rd)
}

/// pop rd
fn pop(rd: Operand) -> String {
    format!("\tpop {}\n", rd)
}

pub fn add_arg() -> String {
    add(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn add(rd: Operand, rn: Operand) -> String {
    format!("\tadd {}, {}\n", rd, rn) // rd <- rd + rn
}

pub fn sub_arg() -> String {
    sub(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn sub(rd: Operand, rn: Operand) -> String {
    format!("\tsub {}, {}\n", rd, rn) // rd <- rd - rn
}

pub fn mul_arg() -> String {
    mul(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn mul(rd: Operand, rn: Operand) -> String {
    format!("\timul {}, {}\n", rd, rn) // rd <- rd x rn
}

pub fn div_arg() -> String {
    div(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn div(_: Operand, rn: Operand) -> String {
    format!("\tcqo\n\tidiv {}\n", rn)
}

fn mov(rd: Operand, src2: Operand) -> String {
    format!("\tmov {}, {}\n", rd, src2)
}

pub fn eq_arg() -> String {
    eq(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn eq(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tsete al\n\tmovzb {}, al\n", rd, rn, rd)
}

pub fn neq_arg() -> String {
    neq(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn neq(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tsetne al\n\tmovzb {}, al\n", rd, rn, rd)
}

pub fn less_arg() -> String {
    less(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn less(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tsetl al\n\tmovzb {}, al\n", rd, rn, rd)
}

pub fn less_or_eq_arg() -> String {
    less_or_eq(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn less_or_eq(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tsetle al\n\tmovzb {}, al\n", rd, rn, rd)
}

pub fn gen_ret() -> String {
    format!(
        "{}{}{}{}",
        pop(Operand::Register(Register::R0)),
        mov(
            Operand::Register(Register::R6),
            Operand::Register(Register::R5)
        ),
        pop(Operand::Register(Register::R5)),
        ret()
    )
}

fn ret() -> String {
    "\tret\n".to_owned()
}
