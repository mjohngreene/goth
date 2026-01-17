//! # Goth Eval - Interpreter for the Goth programming language

pub mod value;
pub mod error;
pub mod prim;
pub mod eval;

pub mod prelude {
    pub use crate::value::{Value, Tensor, TensorData, Closure, Env, PrimFn};
    pub use crate::error::{EvalError, EvalResult};
    pub use crate::eval::{Evaluator, eval, eval_trace};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use goth_ast::prelude::*;

    #[test] fn test_int_literal() { assert_eq!(eval(&Expr::int(42)).unwrap(), Value::Int(42)); }
    #[test] fn test_float_literal() { assert_eq!(eval(&Expr::float(3.14)).unwrap(), Value::float(3.14)); }
    #[test] fn test_bool_literals() { assert_eq!(eval(&Expr::bool(true)).unwrap(), Value::Bool(true)); assert_eq!(eval(&Expr::bool(false)).unwrap(), Value::Bool(false)); }
    #[test] fn test_addition() { assert_eq!(eval(&Expr::add(Expr::int(2), Expr::int(3))).unwrap(), Value::Int(5)); }
    #[test] fn test_subtraction() { assert_eq!(eval(&Expr::sub(Expr::int(10), Expr::int(4))).unwrap(), Value::Int(6)); }
    #[test] fn test_multiplication() { assert_eq!(eval(&Expr::mul(Expr::int(6), Expr::int(7))).unwrap(), Value::Int(42)); }
    #[test] fn test_division() { assert_eq!(eval(&Expr::div(Expr::int(20), Expr::int(4))).unwrap(), Value::Int(5)); }
    #[test] fn test_division_by_zero() { assert!(matches!(eval(&Expr::div(Expr::int(1), Expr::int(0))), Err(EvalError::DivisionByZero))); }
    #[test] fn test_nested_arithmetic() { assert_eq!(eval(&Expr::mul(Expr::add(Expr::int(2), Expr::int(3)), Expr::sub(Expr::int(4), Expr::int(1)))).unwrap(), Value::Int(15)); }
    #[test] fn test_float_arithmetic() { assert_eq!(eval(&Expr::add(Expr::float(1.5), Expr::float(2.5))).unwrap(), Value::float(4.0)); }
    #[test] fn test_mixed_arithmetic() { assert_eq!(eval(&Expr::add(Expr::int(1), Expr::float(2.5))).unwrap(), Value::float(3.5)); }
    #[test] fn test_equality() { assert_eq!(eval(&Expr::binop(BinOp::Eq, Expr::int(5), Expr::int(5))).unwrap(), Value::Bool(true)); assert_eq!(eval(&Expr::binop(BinOp::Eq, Expr::int(5), Expr::int(6))).unwrap(), Value::Bool(false)); }
    #[test] fn test_less_than() { assert_eq!(eval(&Expr::binop(BinOp::Lt, Expr::int(3), Expr::int(5))).unwrap(), Value::Bool(true)); }
    #[test] fn test_logical_and() { assert_eq!(eval(&Expr::binop(BinOp::And, Expr::bool(true), Expr::bool(true))).unwrap(), Value::Bool(true)); assert_eq!(eval(&Expr::binop(BinOp::And, Expr::bool(true), Expr::bool(false))).unwrap(), Value::Bool(false)); }
    #[test] fn test_logical_or() { assert_eq!(eval(&Expr::binop(BinOp::Or, Expr::bool(false), Expr::bool(true))).unwrap(), Value::Bool(true)); }
    #[test] fn test_logical_short_circuit() { assert_eq!(eval(&Expr::binop(BinOp::Or, Expr::bool(true), Expr::binop(BinOp::Eq, Expr::div(Expr::int(1), Expr::int(0)), Expr::int(0)))).unwrap(), Value::Bool(true)); }

    #[test] fn test_identity_function() { assert_eq!(eval(&Expr::app(Expr::lam(Expr::idx(0)), Expr::int(42))).unwrap(), Value::Int(42)); }
    #[test] fn test_constant_function() { assert_eq!(eval(&Expr::app(Expr::app(Expr::lam(Expr::lam(Expr::idx(1))), Expr::int(1)), Expr::int(2))).unwrap(), Value::Int(1)); }
    #[test] fn test_add_function() { let add_fn = Expr::lam(Expr::lam(Expr::add(Expr::idx(1), Expr::idx(0)))); assert_eq!(eval(&Expr::app(Expr::app(add_fn, Expr::int(3)), Expr::int(4))).unwrap(), Value::Int(7)); }
    #[test] fn test_closure_capture() { let expr = Expr::let_(Pattern::var("x"), Expr::int(10), Expr::app(Expr::lam(Expr::add(Expr::idx(0), Expr::idx(1))), Expr::int(5))); assert_eq!(eval(&expr).unwrap(), Value::Int(15)); }

    #[test] fn test_simple_let() { let expr = Expr::let_(Pattern::var("x"), Expr::int(5), Expr::add(Expr::idx(0), Expr::int(3))); assert_eq!(eval(&expr).unwrap(), Value::Int(8)); }
    #[test] fn test_nested_let() { let expr = Expr::let_(Pattern::var("x"), Expr::int(5), Expr::let_(Pattern::var("y"), Expr::int(3), Expr::add(Expr::idx(1), Expr::idx(0)))); assert_eq!(eval(&expr).unwrap(), Value::Int(8)); }

    #[test] fn test_if_true() { assert_eq!(eval(&Expr::if_(Expr::bool(true), Expr::int(1), Expr::int(2))).unwrap(), Value::Int(1)); }
    #[test] fn test_if_false() { assert_eq!(eval(&Expr::if_(Expr::bool(false), Expr::int(1), Expr::int(2))).unwrap(), Value::Int(2)); }
    #[test] fn test_if_with_comparison() { assert_eq!(eval(&Expr::if_(Expr::binop(BinOp::Lt, Expr::int(3), Expr::int(5)), Expr::int(1), Expr::int(2))).unwrap(), Value::Int(1)); }

    #[test] fn test_match_literal() { let expr = Expr::match_(Expr::int(1), vec![MatchArm::new(Pattern::lit(1i64), Expr::int(10)), MatchArm::new(Pattern::wildcard(), Expr::int(20))]); assert_eq!(eval(&expr).unwrap(), Value::Int(10)); }
    #[test] fn test_match_wildcard() { let expr = Expr::match_(Expr::int(99), vec![MatchArm::new(Pattern::lit(1i64), Expr::int(10)), MatchArm::new(Pattern::wildcard(), Expr::int(20))]); assert_eq!(eval(&expr).unwrap(), Value::Int(20)); }
    #[test] fn test_match_tuple() { let expr = Expr::match_(Expr::tuple(vec![Expr::int(1), Expr::int(2)]), vec![MatchArm::new(Pattern::tuple(vec![Pattern::var("a"), Pattern::var("b")]), Expr::add(Expr::idx(1), Expr::idx(0)))]); assert_eq!(eval(&expr).unwrap(), Value::Int(3)); }
    #[test] fn test_match_variant() { let expr = Expr::match_(Expr::variant("Some", Some(Expr::int(5))), vec![MatchArm::new(Pattern::variant("None", None), Expr::int(0)), MatchArm::new(Pattern::variant("Some", Some(Pattern::var("x"))), Expr::idx(0))]); assert_eq!(eval(&expr).unwrap(), Value::Int(5)); }

    #[test] fn test_array_literal() { let result = eval(&Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3)])).unwrap(); match result { Value::Tensor(t) => { assert_eq!(t.shape, vec![3]); assert_eq!(t.get_flat(0), Some(Value::Int(1))); } _ => panic!("Expected tensor") } }
    #[test] fn test_array_sum() { assert_eq!(eval(&Expr::sum(Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3), Expr::int(4)]))).unwrap(), Value::Int(10)); }
    #[test] fn test_array_map() { let expr = Expr::map(Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3)]), Expr::lam(Expr::mul(Expr::idx(0), Expr::int(2)))); match eval(&expr).unwrap() { Value::Tensor(t) => { assert_eq!(t.get_flat(0), Some(Value::Int(2))); assert_eq!(t.get_flat(1), Some(Value::Int(4))); assert_eq!(t.get_flat(2), Some(Value::Int(6))); } _ => panic!("Expected tensor") } }
    #[test] fn test_array_filter() { let expr = Expr::filter(Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3), Expr::int(4), Expr::int(5)]), Expr::lam(Expr::binop(BinOp::Gt, Expr::idx(0), Expr::int(2)))); match eval(&expr).unwrap() { Value::Tensor(t) => { assert_eq!(t.len(), 3); assert_eq!(t.get_flat(0), Some(Value::Int(3))); } _ => panic!("Expected tensor") } }
    #[test] fn test_tensor_broadcasting() { let expr = Expr::add(Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3)]), Expr::int(10)); match eval(&expr).unwrap() { Value::Tensor(t) => { assert_eq!(t.get_flat(0), Some(Value::Int(11))); } _ => panic!("Expected tensor") } }

    #[test] fn test_tuple_construction() { match eval(&Expr::tuple(vec![Expr::int(1), Expr::bool(true), Expr::float(3.14)])).unwrap() { Value::Tuple(vs) => { assert_eq!(vs.len(), 3); assert_eq!(vs[0], Value::Int(1)); assert_eq!(vs[1], Value::Bool(true)); } _ => panic!("Expected tuple") } }
    #[test] fn test_tuple_field_access() { assert_eq!(eval(&Expr::field_idx(Expr::tuple(vec![Expr::int(1), Expr::int(2), Expr::int(3)]), 1)).unwrap(), Value::Int(2)); }
    #[test] fn test_unit() { assert_eq!(eval(&Expr::tuple(vec![])).unwrap(), Value::Unit); }

    #[test] fn test_factorial() {
        let mut e = Evaluator::new();
        let factorial_body = Expr::match_(Expr::idx(0), vec![MatchArm::new(Pattern::lit(0i64), Expr::int(1)), MatchArm::new(Pattern::var("n"), Expr::mul(Expr::idx(0), Expr::app(Expr::name("factorial"), Expr::sub(Expr::idx(0), Expr::int(1)))))]);
        // Create closure with access to globals so it can call itself recursively
        let env = Env::with_globals(e.globals());
        e.define("factorial", Value::closure(1, factorial_body, env));
        assert_eq!(e.eval(&Expr::app(Expr::name("factorial"), Expr::int(5))).unwrap(), Value::Int(120));
    }

    #[test] fn test_fibonacci() {
        let mut e = Evaluator::new();
        let fib_body = Expr::match_(Expr::idx(0), vec![MatchArm::new(Pattern::lit(0i64), Expr::int(0)), MatchArm::new(Pattern::lit(1i64), Expr::int(1)), MatchArm::new(Pattern::var("n"), Expr::add(Expr::app(Expr::name("fib"), Expr::sub(Expr::idx(0), Expr::int(1))), Expr::app(Expr::name("fib"), Expr::sub(Expr::idx(0), Expr::int(2)))))]);
        // Create closure with access to globals so it can call itself recursively
        let env = Env::with_globals(e.globals());
        e.define("fib", Value::closure(1, fib_body, env));
        assert_eq!(e.eval(&Expr::app(Expr::name("fib"), Expr::int(10))).unwrap(), Value::Int(55));
    }

    #[test] fn test_sqrt_primitive() { let mut e = Evaluator::new(); assert_eq!(e.eval(&Expr::app(Expr::name("sqrt"), Expr::float(16.0))).unwrap(), Value::float(4.0)); }
    #[test] fn test_abs_primitive() { let mut e = Evaluator::new(); assert_eq!(e.eval(&Expr::app(Expr::name("abs"), Expr::int(-5))).unwrap(), Value::Int(5)); }
    #[test] fn test_partial_application() { let mut e = Evaluator::new(); let add5 = Expr::app(Expr::name("add"), Expr::int(5)); assert!(e.eval(&add5).unwrap().is_callable()); assert_eq!(e.eval(&Expr::app(add5, Expr::int(3))).unwrap(), Value::Int(8)); }
    #[test] fn test_function_composition() { let add1 = Expr::lam(Expr::add(Expr::idx(0), Expr::int(1))); let mul2 = Expr::lam(Expr::mul(Expr::idx(0), Expr::int(2))); let composed = Expr::binop(BinOp::Compose, add1, mul2); assert_eq!(eval(&Expr::app(composed, Expr::int(3))).unwrap(), Value::Int(7)); }
    #[test] fn test_map_filter_sum() { let expr = Expr::sum(Expr::map(Expr::filter(Expr::array(vec![Expr::int(1), Expr::int(2), Expr::int(3), Expr::int(4), Expr::int(5)]), Expr::lam(Expr::binop(BinOp::Eq, Expr::binop(BinOp::Mod, Expr::idx(0), Expr::int(2)), Expr::int(0)))), Expr::lam(Expr::mul(Expr::idx(0), Expr::idx(0))))); assert_eq!(eval(&expr).unwrap(), Value::Int(20)); }
    #[test] fn test_nested_lambdas() { let expr = Expr::app(Expr::app(Expr::app(Expr::lam(Expr::lam(Expr::lam(Expr::add(Expr::add(Expr::idx(2), Expr::idx(1)), Expr::idx(0))))), Expr::int(1)), Expr::int(2)), Expr::int(3)); assert_eq!(eval(&expr).unwrap(), Value::Int(6)); }
    #[test] fn test_type_error() { assert!(eval(&Expr::add(Expr::int(1), Expr::bool(true))).is_err()); }
    #[test] fn test_unbound_variable() { assert!(matches!(eval(&Expr::idx(999)), Err(EvalError::UnboundIndex(999)))); }
    #[test] fn test_undefined_name() { assert!(matches!(eval(&Expr::name("nonexistent")), Err(EvalError::UndefinedName(_)))); }
    #[test] fn test_de_bruijn_simple() { assert_eq!(eval(&Expr::app(Expr::lam(Expr::idx(0)), Expr::int(5))).unwrap(), Value::Int(5)); }
    #[test] fn test_de_bruijn_nested() { assert_eq!(eval(&Expr::app(Expr::app(Expr::lam(Expr::lam(Expr::add(Expr::idx(0), Expr::idx(1)))), Expr::int(3)), Expr::int(4))).unwrap(), Value::Int(7)); }
    #[test] fn test_de_bruijn_capture_in_closure() { let expr = Expr::let_(Pattern::var("x"), Expr::int(5), Expr::let_(Pattern::var("f"), Expr::lam(Expr::add(Expr::idx(1), Expr::idx(0))), Expr::app(Expr::idx(0), Expr::int(3)))); assert_eq!(eval(&expr).unwrap(), Value::Int(8)); }
    #[test] fn test_dot_product() { let mut e = Evaluator::new(); let a = Expr::array(vec![Expr::float(1.0), Expr::float(2.0), Expr::float(3.0)]); let b = Expr::array(vec![Expr::float(4.0), Expr::float(5.0), Expr::float(6.0)]); let expr = Expr::app(Expr::app(Expr::name("dot"), a), b); assert_eq!(e.eval(&expr).unwrap(), Value::float(32.0)); }
}
