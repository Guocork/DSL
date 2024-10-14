#[derive(Debug)]
enum Expr { // 递归
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr { //  实现了AST 抽象语法树 左右子结点
    fn evaluate(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }
}

macro_rules! expr_internal {
    // 基本数字
    ($e:literal) => { // literal 匹配 字面量
        Expr::Number($e)
    };
    // 括号中的表达式
    ( ( $($inner:tt)+ ) ) => { // 处理括号 + 表示匹配一个或者多个 这里说白了就是去括号的 
        expr_internal!($($inner)+)
    };
    // 乘法
    ($lhs:tt * $rhs:tt) => {
        Expr::Mul(Box::new(expr_internal!($lhs)), Box::new(expr_internal!($rhs)))
    };
    // 加法
    ($lhs:tt + $rhs:tt) => {
        Expr::Add(Box::new(expr_internal!($lhs)), Box::new(expr_internal!($rhs)))
    };
    ($e:tt) => {  // tt 表示匹配任何的类型 
        expr_internal!($e)
    };
}

// 入口宏 
macro_rules! expr {
    ($e:tt) => {  // tt 表示匹配任何的类型 
        expr_internal!($e)
    };
}



fn main() {
    // let expr = Expr::Add(
    //     Box::new(Expr::Number(3)), 
    //     Box::new(Expr::Mul(
    //         Box::new(Expr::Number(5)), 
    //         Box::new(Expr::Number(2))
    //     )
    // ));

    // let expression = expr_internal!(1 + 2 * (3 + 4));
    let exp = expr_internal!(1 + 2);
    let exp = expr_internal!(1 + 2);
    let exp = expr_internal!((1 * 2));
    // let a = expr!(1 + 1); 
    // println!("表达式的 AST：{:?}", expression);
    // let result = expression.evaluate();
    // println!("表达式的结果：{}", result);
    // assert_eq!(result, 15);
    // println!("ssss");
}
