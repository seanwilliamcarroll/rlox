use crate::expr::{Expr, LiteralType};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) {
        let output = self.sprint(&expr);
        println!("{}", output);
    }

    pub fn sprint(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => {
                self.parenthesize(&operator.lexeme, vec![&left, &right])
            }
            Expr::Literal(literal_type) => match literal_type {
                LiteralType::Number(num) => num.to_string(),
                LiteralType::String(string) => string.to_string(),
                LiteralType::Boolean(boolean) => boolean.to_string(),
                LiteralType::Nil => String::from("nil"),
            },
            Expr::Grouping(grouped_expr) => {
                self.parenthesize(&String::from("group"), vec![&grouped_expr])
            }
            Expr::Unary(operator, unary_expr) => {
                self.parenthesize(&operator.lexeme, vec![&unary_expr])
            }
        }
    }

    pub fn parenthesize(&self, name: &String, exprs: Vec<&Expr>) -> String {
        let mut output: String = String::from("(");
        output.push_str(name);
        output.push(' ');
        output.push_str(
            &(exprs
                .iter()
                .map(|x| self.sprint(x))
                .collect::<Vec<String>>()
                .join(" ")),
        );
        output.push_str(&String::from(")"));
        output
    }
}
