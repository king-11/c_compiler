use crate::ast::model::*;

pub fn generate(root: &Program) -> String {
  generate_function(&root.func)
}

fn generate_function(func: &Function) -> String {
  let body = generate_statement(&func.body);

  format!(
  "\t.globl {name}
{name}:
{body}
  ", name = func.name, body = body)
}

fn generate_statement(st: &Statement) -> String {
  match st {
    Statement::Return(val) => {
      let exp = generate_expression(val);

      format!("\tmovl\t${exp}, %eax\n\tret")
    }
  }
}

fn generate_expression(exp: &Expression) -> String {
  match exp {
    Expression::Const(val) => val.to_string()
  }
}
