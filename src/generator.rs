use crate::architecture::*;
use crate::tree::*;

pub type GenerateError = String;

pub fn generator(trees: Vec<Tree>, ident_count: usize) -> Result<String, GenerateError> {
    let mut asm = String::new();

    asm.push_str(&program_prologue());
    asm.push_str(&main_func());
    asm.push_str(&memory_allocate(ident_count * 8));

    for tree in trees {
        generate_assembly(&mut asm, tree)?;
        asm.push_str(&stmt_epilogue());
    }

    asm.push_str(&program_epilogue());

    Ok(asm)
}

fn generate_val(assembly: &mut String, offset: usize) {
    assembly.push_str(&gen_val(offset));
}

pub fn generate_assembly(assembly: &mut String, tree: Tree) -> Result<(), GenerateError> {
    if let Tree::Int(n) = tree {
        assembly.push_str(&push(Operand::Num(n)));
        return Ok(());
    }

    if let Tree::Val { offset } = tree {
        generate_val(assembly, offset);
        assembly.push_str(&pop_val());
        return Ok(());
    }

    if let Tree::Node(kind, lhs, rhs) = tree {
        if let NodeKind::Assign = kind {
            if let Tree::Val { offset } = *lhs {
                generate_val(assembly, offset);
            } else {
                return Err(
                    "The left-hand side value of the assignment is not a variable".to_owned(),
                );
            }
            generate_assembly(assembly, *rhs)?;
            assembly.push_str(&pop_lvar());
            return Ok(());
        }

        generate_assembly(assembly, *lhs)?;
        generate_assembly(assembly, *rhs)?;

        assembly.push_str(&pop_arg());

        match kind {
            NodeKind::Equality => assembly.push_str(&eq_arg()),
            NodeKind::Nonequality => assembly.push_str(&neq_arg()),
            NodeKind::Less => assembly.push_str(&less_arg()),
            NodeKind::LessOrEqual => assembly.push_str(&less_or_eq_arg()),
            NodeKind::Add => assembly.push_str(&add_arg()),
            NodeKind::Sub => assembly.push_str(&sub_arg()),
            NodeKind::Mul => assembly.push_str(&mul_arg()),
            NodeKind::Div => assembly.push_str(&div_arg()),
            _ => {
                return Err("unexpected node".to_owned());
            }
        }
        assembly.push_str(&push(Operand::Register(Register::R0)));
    }

    Ok(())
}
