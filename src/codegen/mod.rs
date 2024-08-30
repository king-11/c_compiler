use std::collections::HashMap;

use crate::{
    ast::model::*,
    lex::{BinaryOperator, UnaryOperator},
    utility::SyntaxError,
};

static FUNCTION_PROLOGUE_START: &str = "push\t%rbp\n\tmov\t%rsp, %rbp";
static FUNCTION_PROLOGUE_END: &str = "mov\t%rbp, %rsp\n\tpop\t%rbp\n\tret\n";

pub struct CodeGenerator {
    symbol_table: HashMap<String, i64>,
    stack_index: i64,
    clause_count: u64,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            stack_index: 0,
            clause_count: 0,
        }
    }

    pub fn generate(&mut self, root: &Program) -> Result<String, SyntaxError> {
        self.generate_function(&root.func)
    }

    fn generate_clause(&mut self) -> String {
        self.clause_count += 1;
        format!("_clause{}", self.clause_count)
    }

    fn generate_end(&self) -> String {
        format!("_end{}", self.clause_count)
    }

    fn generate_function(&mut self, func: &Function) -> Result<String, SyntaxError> {
        let mut body = vec![];
        let mut return_flag = false;
        for x in &func.body {
            let mut st = self.generate_statement(x)?;
            st = st.replace("\n", "\n\t");

            match x {
                Statement::Return(_) => {
                    st = format!("{}\n\t{}", st, FUNCTION_PROLOGUE_END);
                    return_flag = true;
                }
                _ => {}
            }
            body.push(st);
        }

        let end = if return_flag {
            format!("")
        } else {
            format!("mov\t$0, %rax\n\t{}", FUNCTION_PROLOGUE_END)
        };

        Ok(format!(
            "
\t.globl {name}
{name}:
\t{start}
\t{body_text}
\t{end}
",
            start = FUNCTION_PROLOGUE_START,
            name = func.name,
            body_text = body.join("\n\t")
        ))
    }

    fn generate_statement(&mut self, st: &Statement) -> Result<String, SyntaxError> {
        match st {
            Statement::Return(val) => {
                let exp = self.generate_expression(val)?;

                Ok(format!("{}", exp))
            }
            Statement::Declare { name, exp } => {
                if self.symbol_table.contains_key(name.as_ref()) {
                    Err(SyntaxError::new_codegen_error(
                        format!("re-declaration of variable {}", name).to_string(),
                    ))
                } else {
                    let mut assembly_exp = self.generate_expression(&Expression::Const(0))?;
                    if let Some(exp_some) = exp {
                        assembly_exp = self.generate_expression(exp_some)?;
                    };
                    self.stack_index -= 8;
                    self.symbol_table.insert(name.to_string(), self.stack_index);
                    Ok(format!("{}\npush\t%rax", assembly_exp))
                }
            }
            Statement::Exp(val) => self.generate_expression(val),
        }
    }

    fn generate_expression(&mut self, exp: &Expression) -> Result<String, SyntaxError> {
        match exp {
            Expression::Const(val) => Ok(format!("mov\t${}, %rax", val.to_string())),
            Expression::Unary { op, exp } => {
                let inner_exp = self.generate_expression(exp)?;
                let ext_exp = match op {
                    UnaryOperator::Negation => format!("neg\t%rax"),
                    UnaryOperator::BitwiseComplement => format!("not\t%rax"),
                    UnaryOperator::LogicalNegation => {
                        format!("cmp\t$0, %rax\nmov\t$0, %rax\nsete\t%al")
                    }
                };
                Ok(format!("{}\n{}", inner_exp, ext_exp))
            }
            Expression::Binary { exp1, op, exp2 } => {
                let exp1 = self.generate_expression(exp1)?;
                let exp2 = self.generate_expression(exp2)?;
                let inner_exp = format!("{}\npush\t%rax\n{}\npop\t%rcx", exp1, exp2);
                let ext_exp = match op {
                    BinaryOperator::Addition => format!("add\t%rcx, %rax"),
                    BinaryOperator::Multiplication => format!("imul\t%rcx, %rax"),
                    BinaryOperator::Minus => format!("sub\t%rax, %rcx\nmov\t%rcx, %rax"),
                    BinaryOperator::Division => {
                        format!("mov\t%rax, %rbx\nmov\t%rcx, %rax\ncqo\nidiv\t%rbx")
                    }
                    BinaryOperator::Equal => format!("cmp\t%rax, %rcx\nsete\t%al"),
                    BinaryOperator::NotEqual => {
                        format!("cmp\t%rax, %rcx\ncmp\t$0, %rax\nsetne\t%al")
                    }
                    BinaryOperator::LessThan => format!("cmp\t%rax, %rcx\nsetl\t%al"),
                    BinaryOperator::LessThanOrEqual => format!("cmp\t%rax, %rcx\nsetle\t%al"),
                    BinaryOperator::GreaterThan => format!("cmp\t%rax, %rcx\nsetg\t%al"),
                    BinaryOperator::GreaterThanOrEqual => format!("cmp\t%rax, %rcx\nsetge\t%al"),
                    BinaryOperator::And => {
                        return Ok(format!("{}\ncmp\t$0, %rax\njne\t{_clause2}\njmp\t{_end}\n{_clause2}:\n{}\ncmp\t$0, %rax\ncmp $0, %rax\nsetne\t%al\n{_end}:", exp1, exp2, _clause2 = self.generate_clause(), _end = self.generate_end()));
                    }
                    BinaryOperator::Or => {
                        return Ok(format!("{}\ncmp\t$0, %rax\nje\t{_clause2}\nmov\t$1, %rax\njmp\t{_end}\n{_clause2}:\n{}\ncmp\t$0, %rax\ncmp $0, %rax\nsetne\t%al\n{_end}:", exp1, exp2, _clause2 = self.generate_clause(), _end = self.generate_end()));
                    }
                };
                Ok(format!("{}\n{}", inner_exp, ext_exp))
            }
            Expression::Assign { name, exp } => match self.symbol_table.get(name.as_ref()) {
                None => Err(SyntaxError::new_codegen_error(
                    format!("variable not declared {}", name).to_string(),
                )),
                Some(&offset) => {
                    let assign_exp = self.generate_expression(exp)?;
                    Ok(format!("{}\nmov\t%rax, {}(%rbp)", assign_exp, offset))
                }
            },
            Expression::Var { name } => match self.symbol_table.get(name.as_ref()) {
                None => Err(SyntaxError::new_codegen_error(
                    format!("variable not declared {}", name).to_string(),
                )),
                Some(&offset) => Ok(format!("mov\t{}(%rbp), %rax", offset)),
            },
        }
    }
}
