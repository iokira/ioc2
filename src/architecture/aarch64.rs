use crate::token::Int;
use core::fmt;

pub enum Register {
    /// x0
    R0,
    /// x1
    R1,
    /// x8(rbp)
    R8,
    /// x9(rsp)
    R9,
    /// sp
    R13,
}

pub enum Operand {
    Num(Int),
    Register(Register),
    Address(Register),
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Register::R0 => "x0",
            Register::R1 => "x1",
            Register::R8 => "x8",
            Register::R9 => "x9",
            Register::R13 => "sp",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: String = match self {
            Operand::Register(r) => r.to_string(),
            Operand::Address(r) => format!("[{}]", r),
            Operand::Num(n) => format!("#{}", n),
        };
        write!(f, "{}", name)
    }
}

pub fn program_prologue() -> String {
    ".text\n".to_owned()
}

pub fn main_func() -> String {
    ".globl _main\n_main:\n".to_owned()
}

/// mov rbp(r8), sp(r13)
/// mov rsp(r9), sp(r13)
/// push rbp(r8)
/// mov rbp(r8), rsp(r9)
/// sub rsp(r9), $bytes
pub fn memory_allocate(bytes: usize) -> String {
    format!(
        "{}{}{}{}{}",
        mov(
            Operand::Register(Register::R8),
            Operand::Register(Register::R13)
        ),
        mov(
            Operand::Register(Register::R9),
            Operand::Register(Register::R13)
        ),
        push(Operand::Register(Register::R8)),
        mov(
            Operand::Register(Register::R8),
            Operand::Register(Register::R9)
        ),
        sub(Operand::Register(Register::R9), Operand::Num(bytes))
    )
}

fn ldr(rd: Operand, rn: Operand) -> String {
    format!("\tldr {}, {}\n", rd, rn)
}

fn str(rd: Operand, rn: Operand) -> String {
    format!("\tstr {}, {}\n", rd, rn)
}

pub fn stmt_epilogue() -> String {
    pop(Operand::Register(Register::R0))
}

/// mov rsp(r9), rbp(r8)
/// pop rbp(r8)
/// ret
pub fn program_epilogue() -> String {
    format!(
        "{}{}{}",
        mov(
            Operand::Register(Register::R9),
            Operand::Register(Register::R8)
        ),
        pop(Operand::Register(Register::R8)),
        ret()
    )
}

/// mov r0, rbp(r8)
/// sub rax, offset
/// push rax
pub fn gen_val(offset: usize) -> String {
    format!(
        "{}{}{}",
        mov(
            Operand::Register(Register::R0),
            Operand::Register(Register::R8)
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
        ldr(
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
        str(
            Operand::Register(Register::R1),
            Operand::Address(Register::R0)
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

/// sub r9, #8
/// mov r0, rd
/// str r0, r9
pub fn push(rd: Operand) -> String {
    format!(
        "{}{}{}",
        sub(Operand::Register(Register::R9), Operand::Num(8)),
        mov(Operand::Register(Register::R1), rd),
        str(
            Operand::Register(Register::R1),
            Operand::Address(Register::R9)
        )
    )
}

/// ldr rd, r9
/// add r9, #8
pub fn pop(rd: Operand) -> String {
    format!(
        "{}{}",
        ldr(rd, Operand::Address(Register::R9)),
        add(Operand::Register(Register::R9), Operand::Num(8))
    )
}

pub fn add_arg() -> String {
    add(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn add(rd: Operand, rn: Operand) -> String {
    format!("\tadd {}, {}, {}\n", rd, rd, rn) // rd <- rn + src2
}

pub fn sub_arg() -> String {
    sub(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn sub(rd: Operand, rn: Operand) -> String {
    format!("\tsub {}, {}, {}\n", rd, rd, rn) // rd <- rn - src2
}

pub fn mul_arg() -> String {
    mul(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn mul(rd: Operand, rn: Operand) -> String {
    format!("\tmul {}, {}, {}\n", rd, rd, rn) // rd <- rn x rm
}

pub fn div_arg() -> String {
    div(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn div(rd: Operand, rn: Operand) -> String {
    format!("\tudiv {}, {}, {}\n", rd, rd, rn)
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
    format!("\tcmp {}, {}\n\tcset {}, EQ\n", rd, rn, rd)
}

pub fn neq_arg() -> String {
    neq(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn neq(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tcset {}, NE\n", rd, rn, rd)
}

pub fn less_arg() -> String {
    less(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn less(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tcset {}, LT\n", rd, rn, rd)
}

pub fn less_or_eq_arg() -> String {
    less_or_eq(
        Operand::Register(Register::R0),
        Operand::Register(Register::R1),
    )
}

fn less_or_eq(rd: Operand, rn: Operand) -> String {
    format!("\tcmp {}, {}\n\tcset {}, LS\n", rd, rn, rd)
}

pub fn gen_ret() -> String {
    format!(
        "{}{}{}{}",
        pop(Operand::Register(Register::R0)),
        mov(
            Operand::Register(Register::R9),
            Operand::Register(Register::R8)
        ),
        pop(Operand::Register(Register::R8)),
        ret()
    )
}

fn ret() -> String {
    "\tret\n".to_owned()
}

fn lbegin(n: usize) -> String {
    format!("Lbegin{:0width$}:\n", n, width = 3)
}

fn lend(n: usize) -> String {
    format!("Lend{:0width$}:\n", n, width = 3)
}

fn lelse(n: usize) -> String {
    format!("Lelse{:0width$}:\n", n, width = 3)
}

fn b_lbegin(n: usize) -> String {
    format!("\tb Lbegin{:0width$}\n", n, width = 3)
}

fn b_lend(n: usize) -> String {
    format!("\tb Lend{:0width$}\n", n, width = 3)
}

fn b_eq_lend(n: usize) -> String {
    format!("\tb.EQ Lend{:0width$}\n", n, width = 3)
}

fn b_eq_lelse(n: usize) -> String {
    format!("\tb.EQ Lelse{:0width$}\n", n, width = 3)
}

pub fn gen_if(expr: &str, stmt: &str, n: usize) -> String {
    format!(
        "{}{}\tcmp {}, {}\n{}{}{}",
        expr,
        pop(Operand::Register(Register::R0)),
        Operand::Register(Register::R0),
        Operand::Num(0),
        b_eq_lend(n),
        stmt,
        lend(n)
    )
}

pub fn gen_if_else(expr: &str, stmt: &str, stmt_else: &str, n: usize) -> String {
    format!(
        "{}{}\tcmp {}, {}\n{}{}{}{}{}{}",
        expr,
        pop(Operand::Register(Register::R0)),
        Operand::Register(Register::R0),
        Operand::Num(0),
        b_eq_lelse(n),
        stmt,
        b_lend(n),
        lelse(n),
        stmt_else,
        lend(n),
    )
}

pub fn gen_while(expr: &str, stmt: &str, n: usize) -> String {
    format!(
        "{}{}{}\tcmp {}, {}\n{}{}{}{}",
        lbegin(n),
        expr,
        pop(Operand::Register(Register::R0)),
        Operand::Register(Register::R0),
        Operand::Num(0),
        b_eq_lend(n),
        stmt,
        b_lbegin(n),
        lend(n),
    )
}

pub fn gen_for(init_expr: &str, cond_expr: &str, loop_expr: &str, stmt: &str, n: usize) -> String {
    format!(
        "{}{}{}{}\tcmp {}, {}\n{}{}{}{}{}",
        init_expr,
        lbegin(n),
        cond_expr,
        pop(Operand::Register(Register::R0)),
        Operand::Register(Register::R0),
        Operand::Num(0),
        b_eq_lend(n),
        stmt,
        loop_expr,
        b_lbegin(n),
        lend(n)
    )
}
