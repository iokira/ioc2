use crate::architecture::*;
use crate::tree::*;

pub type GenerateError = String;

pub fn generator(trees: Vec<Tree>, ident_count: usize) -> Result<String, GenerateError> {
    let mut asm = String::new();

    asm.push_str(&program_prologue());
    asm.push_str(&main_func());
    asm.push_str(&memory_allocate(ident_count * 8));

    for tree in trees {
        asm.push_str(&generate_assembly(tree, 0)?);
        asm.push_str(&stmt_epilogue());
    }

    asm.push_str(&program_epilogue());

    Ok(asm)
}

fn generate_val(offset: usize) -> String {
    gen_val(offset)
}

pub fn generate_assembly(tree: Tree, flow_count: usize) -> Result<String, GenerateError> {
    match tree {
        Tree::None => Ok(String::new()),
        Tree::Int(n) => Ok(push(Operand::Num(n))),
        Tree::Val { offset } => Ok(format!("{}{}", generate_val(offset), pop_val())),
        Tree::Return(t) => Ok(format!(
            "{}{}",
            generate_assembly(*t, flow_count)?,
            gen_ret()
        )),
        Tree::If(expr, stmt) => Ok(format!(
            "{}",
            &gen_if(
                &generate_assembly(*expr, flow_count + 1)?,
                &generate_assembly(*stmt, flow_count + 1)?,
                flow_count
            ),
        )),
        Tree::IfElse(expr, stmt, stmt_else) => Ok(format!(
            "{}",
            &gen_if_else(
                &generate_assembly(*expr, flow_count + 1)?,
                &generate_assembly(*stmt, flow_count + 1)?,
                &generate_assembly(*stmt_else, flow_count + 1)?,
                flow_count
            )
        )),
        Tree::While(expr, stmt) => Ok(format!(
            "{}",
            gen_while(
                &generate_assembly(*expr, flow_count + 1)?,
                &generate_assembly(*stmt, flow_count + 1)?,
                flow_count
            )
        )),
        Tree::For(init_expr, cond_expr, loop_expr, stmt) => Ok(format!(
            "{}",
            gen_for(
                &generate_assembly(*init_expr, flow_count + 1)?,
                &generate_assembly(*cond_expr, flow_count + 1)?,
                &generate_assembly(*loop_expr, flow_count + 1)?,
                &generate_assembly(*stmt, flow_count + 1)?,
                flow_count
            )
        )),
        Tree::Node(kind, lhs, rhs) => {
            let mut node_str = String::new();
            if let NodeKind::Assign = kind {
                let mut str = String::new();
                if let Tree::Val { offset } = *lhs {
                    str.push_str(&generate_val(offset));
                } else {
                    return Err(
                        "The left-hand side value of the assignment is not a variable".to_owned(),
                    );
                }
                str.push_str(&generate_assembly(*rhs, flow_count)?);
                str.push_str(&pop_lvar());
                return Ok(str);
            }

            node_str.push_str(&generate_assembly(*lhs, flow_count)?);
            node_str.push_str(&generate_assembly(*rhs, flow_count)?);

            node_str.push_str(&pop_arg());

            match kind {
                NodeKind::Equality => node_str.push_str(&eq_arg()),
                NodeKind::Nonequality => node_str.push_str(&neq_arg()),
                NodeKind::Less => node_str.push_str(&less_arg()),
                NodeKind::LessOrEqual => node_str.push_str(&less_or_eq_arg()),
                NodeKind::Add => node_str.push_str(&add_arg()),
                NodeKind::Sub => node_str.push_str(&sub_arg()),
                NodeKind::Mul => node_str.push_str(&mul_arg()),
                NodeKind::Div => node_str.push_str(&div_arg()),
                _ => {
                    return Err("unexpected node".to_owned());
                }
            }
            node_str.push_str(&push(Operand::Register(Register::R0)));
            Ok(node_str)
        }
    }
}
