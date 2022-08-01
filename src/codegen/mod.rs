use crate::{ast::model::*, lex::UnaryOperator};

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

      format!("{exp}\n\tret")
    }
  }
}

fn generate_expression(exp: &Expression) -> String {
  match exp {
    Expression::Const(val) => format!("\tmovl\t${}, %eax", val.to_string()),
    Expression::Unary { op, exp } => {
      let inner_exp = generate_expression(exp);
      let ext_exp = match op {
        UnaryOperator::Negation => format!("neg\t%eax"),
        UnaryOperator::BitwiseComplement => format!("not\t%eax"),
        UnaryOperator::LogicalNegation => {
          format!("cmpl\t$0, %eax\n\tmovl\t$0, %eax\n\tsete\t%al")
        }
      };
      format!("{}\n\t{}", inner_exp, ext_exp)
    }
  }
}
