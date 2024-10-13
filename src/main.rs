#[derive(Debug)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }
}

macro_rules! expr {
    ($e:tt) => {
        expr_internal!($e)
    };
}

macro_rules! expr_internal {
    // 基本数字
    ($e:literal) => {
        Expr::Number($e)
    };
    // 括号中的表达式
    ( ( $($inner:tt)+ ) ) => {
        expr_internal!($($inner)+)
    };
    // 加法
    ($lhs:tt + $rhs:tt) => {
        Expr::Add(Box::new(expr_internal!($lhs)), Box::new(expr_internal!($rhs)))
    };
    // 乘法
    ($lhs:tt * $rhs:tt) => {
        Expr::Mul(Box::new(expr_internal!($lhs)), Box::new(expr_internal!($rhs)))
    };
}


fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Number(3)), 
        Box::new(Expr::Mul(
            Box::new(Expr::Number(5)), 
            Box::new(Expr::Number(2))
        )
    ));

    // let expression = expr!(1 + 2 * (3 + 4));
    // println!("表达式的 AST：{:?}", expression);
    // let result = expression.evaluate();
    // println!("表达式的结果：{}", result);
    // assert_eq!(result, 15);
    println!("ssss");
}
