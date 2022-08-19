use std::collections::HashMap;

use crate::{ast::model::*, lex::{UnaryOperator, BinaryOperator}, utility::{generate_clause, generate_end, SyntaxError}};

pub struct CodeGenerator {
  symbol_table: HashMap<String, i64>,
  stack_index: i64,
}

impl CodeGenerator {
  pub fn new() -> Self {
    Self { symbol_table: HashMap::new(), stack_index: 0 }
  }
  pub fn generate(&mut self, root: &Program) -> Result<String, SyntaxError> {
    self.generate_function(&root.func)
  }
  fn generate_function(&mut self, func: &Function) -> Result<String, SyntaxError> {
    let mut body = vec![];
    let mut return_flag = false;
    let function_prologue = "mov\t%rbp, %rsp\n\tpop\t%rbp\n\tret\n";
    for x in &func.body {
      let mut st = self.generate_statement(x)?;
      match x {
        Statement::Return(_) => {
          st = format!("{}\n\t{}", st, function_prologue);
          return_flag = true;
        },
        _ => {}
      }
      body.push(st);
    }

    let end = if return_flag {format!("")} else {format!("\n\tmov\t$0, %rax\n\t{}", function_prologue)};

    Ok(format!("\t.globl {name}
{name}:
  push  %rbp
  mov %rsp, %rbp
\t{body_text}{end}
", name = func.name, body_text = body.join("\n\t")))
  }
  fn generate_statement(&mut self, st: &Statement) -> Result<String, SyntaxError> {
    match st {
      Statement::Return(val) => {
        let exp = self.generate_expression(val)?;

        Ok(format!("{}", exp))
      },
      Statement::Declare { name, exp } => {
        if self.symbol_table.contains_key(name.as_ref()) {
          Err(SyntaxError::new_codegen_error(format!("re-declaration of variable {}", name).to_string()))
        }
        else {
          let mut assembly_exp = self.generate_expression(&Expression::Const(0))?;
          if let Some(exp_some) = exp {
            assembly_exp = self.generate_expression(exp_some)?;
          };
          self.stack_index -= 8;
          self.symbol_table.insert(name.to_string(), self.stack_index);
          Ok(format!("{}\n\tpush\t%rax", assembly_exp))
        }
      },
      Statement::Exp(val) => self.generate_expression(val)
    }
  }
  fn generate_expression(&self, exp: &Expression) -> Result<String, SyntaxError> {
    match exp {
      Expression::Const(val) => Ok(format!("mov\t${}, %rax", val.to_string())),
      Expression::Unary { op, exp } => {
        let inner_exp = self.generate_expression(exp)?;
        let ext_exp = match op {
          UnaryOperator::Negation => format!("neg\t%rax"),
          UnaryOperator::BitwiseComplement => format!("not\t%rax"),
          UnaryOperator::LogicalNegation => {
            format!("cmp\t$0, %rax\n\tmov\t$0, %rax\n\tsete\t%al")
          }
        };
        Ok(format!("{}\n\t{}", inner_exp, ext_exp))
      },
      Expression::Binary { exp1, op, exp2 } => {
        let exp1 = self.generate_expression(exp1)?;
        let exp2 = self.generate_expression(exp2)?;
        let inner_exp = format!("{}\n\tpush\t%rax\n{}\n\tpop\t%rcx", exp1, exp2);
        let ext_exp = match op {
          BinaryOperator::Addition => format!("add\t%rcx, %rax"),
          BinaryOperator::Multiplication => format!("imul\t%rcx, %rax"),
          BinaryOperator::Minus => format!("sub\t%rax, %rcx\n\tmov\t%rcx, %rax"),
          BinaryOperator::Division => format!("mov\t%rax, %rbx\n\tmov\t%rcx, %rax\n\tcqo\n\tidiv\t%rbx"),
          BinaryOperator::Equal => format!("cmp\t%rax, %rcx\n\tsete\t%al"),
          BinaryOperator::NotEqual => format!("cmp\t%rax, %rcx\n\tcmp\t$0, %rax\n\tsetne\t%al"),
          BinaryOperator::LessThan => format!("cmp\t%rax, %rcx\n\tsetl\t%al"),
          BinaryOperator::LessThanOrEqual => format!("cmp\t%rax, %rcx\n\tsetle\t%al"),
          BinaryOperator::GreaterThan => format!("cmp\t%rax, %rcx\n\tsetg\t%al"),
          BinaryOperator::GreaterThanOrEqual => format!("cmp\t%rax, %rcx\n\tsetge\t%al"),
          BinaryOperator::And => {
            return Ok(format!("{exp1}\n\tcmp\t$0, %rax\n\tjne\t{_clause2}\n\tjmp\t{_end}\n{_clause2}:\n{exp2}\n\tcmp\t$0, %rax\n\tcmp $0, %rax\n\tsetne\t%al\n{_end}:", _clause2 = generate_clause(), _end = generate_end()));
          },
          BinaryOperator::Or => {
            return Ok(format!("{exp1}\n\tcmp\t$0, %rax\n\tje\t{_clause2}\n\tmov\t$1, %rax\n\tjmp\t{_end}\n{_clause2}:\n{exp2}\n\tcmp\t$0, %rax\n\tcmp $0, %rax\n\tsetne\t%al\n{_end}:", _clause2 = generate_clause(), _end = generate_end()));
          },
        };
        Ok(format!("{}\n\t{}", inner_exp, ext_exp))
      },
      Expression::Assign { name, exp } => {
        match self.symbol_table.get(name.as_ref()) {
          None => Err(SyntaxError::new_codegen_error(format!("variable not declared {}", name).to_string())),
          Some(&offset) => {
            let assign_exp = self.generate_expression(exp)?;
            Ok(format!("{}\n\tmov\t%rax, {}(%rbp)", assign_exp, offset))
          }
        }
      },
      Expression::Var { name } => {
        match self.symbol_table.get(name.as_ref()) {
          None => Err(SyntaxError::new_codegen_error(format!("variable not declared {}", name).to_string())),
          Some(&offset) => {
            Ok(format!("\tmov\t{}(%rbp), %rax", offset))
          }
        }
      }
    }
  }
}
