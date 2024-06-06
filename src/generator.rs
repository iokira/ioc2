use crate::architecture::*;
use crate::tree::*;

pub type GenerateError = String;

pub fn generator(trees: Vec<Tree>, ident_count: usize) -> Result<String, GenerateError> {
    let mut asm = String::new();

    asm.push_str(&program_prologue());
    asm.push_str(&main_func());
    asm.push_str(&memory_allocate(ident_count * 8));

    let mut flow_count = 0;
    for tree in trees {
        let (str, count) = generate_assembly(tree, flow_count)?;
        flow_count = count;
        asm.push_str(&str);
        asm.push_str(&stmt_epilogue());
    }

    asm.push_str(&program_epilogue());

    Ok(asm)
}

fn generate_val(name: &str, offset: usize) -> String {
    gen_val(name, offset)
}

pub fn generate_assembly(tree: Tree, flow_count: usize) -> Result<(String, usize), GenerateError> {
    match tree {
        Tree::None => Ok((String::new(), flow_count)),
        Tree::Int(n) => Ok((push(Operand::Num(n)), flow_count)),
        Tree::Val { name, offset } => Ok((
            format!("{}{}", generate_val(&name, offset), pop_val()),
            flow_count,
        )),
        Tree::Return(t) => {
            let (asm, flow_count) = generate_assembly(*t, flow_count)?;
            Ok((format!("{}{}", asm, gen_ret()), flow_count))
        }
        Tree::If(expr, stmt) => {
            let (expr, flow_count) = generate_assembly(*expr, flow_count)?;
            let (stmt, flow_count) = generate_assembly(*stmt, flow_count)?;
            Ok((
                gen_if(&expr, &stmt, flow_count + 1).to_string(),
                flow_count + 1,
            ))
        }
        Tree::IfElse(expr, stmt, stmt_else) => {
            let (expr, flow_count) = generate_assembly(*expr, flow_count)?;
            let (stmt, flow_count) = generate_assembly(*stmt, flow_count)?;
            let (stmt_else, flow_count) = generate_assembly(*stmt_else, flow_count)?;
            Ok((
                gen_if_else(&expr, &stmt, &stmt_else, flow_count + 1).to_string(),
                flow_count + 1,
            ))
        }
        Tree::While(expr, stmt) => {
            let (expr, flow_count) = generate_assembly(*expr, flow_count)?;
            let (stmt, flow_count) = generate_assembly(*stmt, flow_count)?;
            Ok((
                gen_while(&expr, &stmt, flow_count + 1).to_string(),
                flow_count + 1,
            ))
        }
        Tree::For(init_expr, cond_expr, loop_expr, stmt) => {
            let (init_expr, flow_count) = generate_assembly(*init_expr, flow_count)?;
            let (cond_expr, flow_count) = generate_assembly(*cond_expr, flow_count)?;
            let (loop_expr, flow_count) = generate_assembly(*loop_expr, flow_count)?;
            let (stmt, flow_count) = generate_assembly(*stmt, flow_count)?;
            Ok((
                gen_for(&init_expr, &cond_expr, &loop_expr, &stmt, flow_count + 1).to_string(),
                flow_count + 1,
            ))
        }
        Tree::Block(trees) => {
            let mut asm = String::new();
            let mut count = flow_count;
            for tree in trees {
                let (str, n) = generate_assembly(tree, count)?;
                asm = format!("{}{}", asm, str);
                count = n;
            }
            Ok((asm, count))
        }
        Tree::Node(kind, lhs, rhs) => {
            let mut node_str = String::new();
            if let NodeKind::Assign = kind {
                let mut str = String::new();
                if let Tree::Val { name, offset } = *lhs {
                    str.push_str(&generate_val(&name, offset));
                } else {
                    return Err(
                        "The left-hand side value of the assignment is not a variable".to_owned(),
                    );
                }
                let (asm, flow_count) = generate_assembly(*rhs, flow_count)?;
                str.push_str(&asm);
                str.push_str(&pop_lvar());
                return Ok((str, flow_count));
            }

            let (asm, flow_count) = generate_assembly(*lhs, flow_count)?;
            node_str.push_str(&asm);
            let (asm, flow_count) = generate_assembly(*rhs, flow_count)?;
            node_str.push_str(&asm);

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
            Ok((node_str, flow_count))
        }
        Tree::Func { name } => Ok((gen_func(&name), flow_count)),
    }
}
