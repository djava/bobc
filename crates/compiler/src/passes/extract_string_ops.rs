use crate::{
    constants::FN_STR_CONCAT,
    passes::ASTPass,
    syntax_trees::{ast::*, shared::*},
    utils::global,
};

#[derive(Debug)]
pub struct ExtractStringOps;

impl ASTPass for ExtractStringOps {
    fn run_pass(self, mut m: Program) -> Program {
        for f in m.functions.iter_mut() {
            for s in f.body.iter_mut() {
                extract_string_ops_for_statement(s, &f.types);
            }
        }
        m
    }
}

fn extract_string_ops_for_statement(s: &mut Statement, type_env: &TypeEnv) {
    match s {
        Statement::Assign(_, expr, _) | Statement::Expr(expr) => {
            extract_string_ops_for_expr(expr, type_env);
        }
        Statement::Conditional(expr, statements, statements1) => {
            extract_string_ops_for_expr(expr, type_env);
            for s in statements {
                extract_string_ops_for_statement(s, type_env);
            }
            for s in statements1 {
                extract_string_ops_for_statement(s, type_env);
            }
        }
        Statement::WhileLoop(expr, statements) => {
            extract_string_ops_for_expr(expr, type_env);
            for s in statements {
                extract_string_ops_for_statement(s, type_env);
            }
        }
        Statement::Return(expr) => {
            extract_string_ops_for_expr(expr, type_env);
        }
    }
}

fn extract_string_ops_for_expr(e: &mut Expr, type_env: &TypeEnv) {
    match e {
        Expr::BinaryOp(l, op, r) => {
            extract_string_ops_for_expr(l, type_env);
            extract_string_ops_for_expr(r, type_env);

            if l.get_type(type_env) == ValueType::string()
                && r.get_type(type_env) == ValueType::string()
                && *op == BinaryOperator::Add
            {
                *e = Expr::Call(
                    Box::new(Expr::GlobalSymbol(global!(FN_STR_CONCAT))),
                    vec![*l.clone(), *r.clone()],
                );
            }
        }
        Expr::StatementBlock(statements, expr) => {
            for s in statements {
                extract_string_ops_for_statement(s, type_env);
            }
            extract_string_ops_for_expr(expr, type_env);
        }
        Expr::UnaryOp(_, expr) => {
            extract_string_ops_for_expr(expr, type_env);
        }
        Expr::Call(expr, exprs) => {
            extract_string_ops_for_expr(expr, type_env);
            for e in exprs {
                extract_string_ops_for_expr(e, type_env);
            }
        }
        Expr::Ternary(expr, expr1, expr2) => {
            extract_string_ops_for_expr(expr, type_env);
            extract_string_ops_for_expr(expr1, type_env);
            extract_string_ops_for_expr(expr2, type_env);
        }
        Expr::Tuple(exprs) => {
            for e in exprs {
                extract_string_ops_for_expr(e, type_env);
            }
        }
        Expr::Array(exprs) => {
            for e in exprs {
                extract_string_ops_for_expr(e, type_env);
            }
        }
        Expr::Subscript(expr, expr1) => {
            extract_string_ops_for_expr(expr, type_env);
            extract_string_ops_for_expr(expr1, type_env);
        }
        Expr::Constant(_)
        | Expr::Id(_)
        | Expr::Allocate(_, _)
        | Expr::GlobalSymbol(_)
        | Expr::Closure(_, _)
        | Expr::Lambda(_) => {}
    }
}
