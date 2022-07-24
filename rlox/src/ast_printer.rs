use crate::expr::Expr;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) {
        let output = self.sprint(expr);
        println!("{}", output);
    }

    pub fn sprint(&self, expr: Expr) -> String {
        match expr {
            Expr::Binary(_left, _operator, _right) => unimplemented!(),
            Expr::Literal(_literal_type) => unimplemented!(),
            Expr::Grouping(_grouped_expr) => unimplemented!(),
            Expr::Unary(_operator, _unary_expr) => unimplemented!(),
        }
    }

    // pub fn parenthezie(&self, name: &String, exprs: Vec<Expr>) -> String {
    //     unimplemented!()
    // }
}
