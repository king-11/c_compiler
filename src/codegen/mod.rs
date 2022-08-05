use crate::{ast::model::*, lex::{UnaryOperator, BinaryOperator}, utility::{generate_clause, generate_end}};

pub fn generate(root: &Program) -> String {
  generate_function(&root.func)
}

fn generate_function(func: &Function) -> String {
  let body = generate_statement(&func.body);

  format!(
  "\t.globl {name}
{name}:
{body}", name = func.name, body = body)
}

fn generate_statement(st: &Statement) -> String {
  match st {
    Statement::Return(val) => {
      let exp = generate_expression(val);

      format!("{exp}\n\tret\n")
    }
  }
}

fn generate_expression(exp: &Expression) -> String {
  match exp {
    Expression::Const(val) => format!("\tmov\t${}, %rax", val.to_string()),
    Expression::Unary { op, exp } => {
      let inner_exp = generate_expression(exp);
      let ext_exp = match op {
        UnaryOperator::Negation => format!("neg\t%rax"),
        UnaryOperator::BitwiseComplement => format!("not\t%rax"),
        UnaryOperator::LogicalNegation => {
          format!("cmp\t$0, %rax\n\tmov\t$0, %rax\n\tsete\t%al")
        }
      };
      format!("{}\n\t{}", inner_exp, ext_exp)
    },
    Expression::Binary { exp1, op, exp2 } => {
      let exp1 = generate_expression(exp1);
      let exp2 = generate_expression(exp2);
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
          return format!("{exp1}\n\tcmp\t$0, %rax\n\tjne\t{_clause2}\n\tjmp\t{_end}\n{_clause2}:\n{exp2}\n\tcmp\t$0, %rax\n\tcmp $0, %rax\n\tsetne\t%al\n{_end}:", _clause2 = generate_clause(), _end = generate_end());
        },
        BinaryOperator::Or => {
          return format!("{exp1}\n\tcmp\t$0, %rax\n\tje\t{_clause2}\n\tmov\t$1, %rax\n\tjmp\t{_end}\n{_clause2}:\n{exp2}\n\tcmp\t$0, %rax\n\tcmp $0, %rax\n\tsetne\t%al\n{_end}:", _clause2 = generate_clause(), _end = generate_end());
        },
      };
      format!("{}\n\t{}", inner_exp, ext_exp)
    }
  }
}
