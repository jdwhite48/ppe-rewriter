#![allow(dead_code)]

use egg::*;

define_language! {
    enum PPELanguage {
        Num(i32),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Symbol(Symbol),
    }
}

fn make_rules() -> Vec<Rewrite<PPELanguage, ()>> {
    vec![
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        rewrite!("mul-0"; "(* ?a 0)" => "0"),
        rewrite!("mul-1"; "(* ?a 1)" => "?a"),
    ]
}

fn simplify(s: &str) -> String {
    // Parse the expression
    let expr: RecExpr<PPELanguage> = s.parse().unwrap();
    // Simplify expression using a Runner, creating an e-graph
    // and running the given rules over given expression
    let runner = Runner::default().with_expr(&expr).run(&make_rules());
    // Runner knows which e-class the expression is in
    let root = runner.roots[0];

    // Use extractor to pick best element in the root e-class
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tests() {
        assert_eq!(simplify("(* 0 42)"), "0");
        assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
        assert_eq!(simplify("(+ x (* y 0))"), "x");
    }
}
