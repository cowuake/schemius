mod common;

#[test]
fn interpreter_define() {
    integration_subtest_eval_to! {
        { expression: "(begin (define x 0) (define x 1) x)", expected: "1" };
    }
}

#[test]
fn interpreter_set() {
    integration_subtest_eval_to! {
        { expression: "(begin (define x 0) (set! x 1) x)", expected: "1" };
    }
}

#[test]
fn interpreter_define_set() {
    integration_subtest_eval_to! {
        { expression: "(begin (define x 7) (define f1 (lambda () (define x 10) x)) (define f2 (lambda () (set! x 11) x))))", expected: "ok" };
        { expression: "(begin (f1) x)", expected: "7" };
        { expression: "(begin (f2) x)", expected: "11" };
        { expression: "(begin (lambda () (define x 12)) x)", expected: "11" };
        { expression: "(begin (lambda () (set! x 64)) x)", expected: "11" };
    }
}

#[test]
fn interpreter_lambda_malformed_args() {
    integration_subtest_is_err! {
        expression: "(define f (lambda (3) (* x 2)))";
    }
}

#[test]
fn interpreter_define_malformed_args() {
    integration_subtest_is_err! {
        expression: "(define (f 3) (* x 2))";
    }
}

#[test]
fn interpreter_sum() {
    integration_subtest_eval_to! {
        { expression: "(+)", expected: "0" };
        { expression: "(+ 1)", expected: "1" };
        { expression: "(+ 1 2 3)", expected: "6" };
    }
}

#[test]
fn interpreter_diff() {
    integration_subtest_is_err! {
        expression: "(-)";
    }

    integration_subtest_eval_to! {
        { expression: "(- 1)", expected: "-1" };
        { expression: "(- 1 -2 3)", expected: "0" };
    }
}

#[test]
fn interpreter_prod() {
    integration_subtest_eval_to! {
        { expression: "(*)", expected: "1" };
        { expression: "(* 7)", expected: "7" };
        { expression: "(* 2 2 3)", expected: "12" };
    }
}

#[test]
fn interpreter_quot() {
    integration_subtest_is_err! {
        expression: "(/)";
    }

    integration_subtest_eval_to! {
        { expression: "(/ 2)", expected: "1/2" };
        { expression: "(/ -1 -2 -3)", expected: "-1/6" };
    }
}

#[test]
fn interpreter_inter_variant() {
    integration_subtest_eval_to! {
        { expression: r#"(begin (define a "hello") (define b "world") (define l (list a b)) (set! a "farewell") l)"#, expected: r#"("hello" "world")"# };
        { expression: r#"(begin (define a "hello") (define b "world") (define l (list a b)) (string-set! a 0 #\W) l)"#, expected: r#"("Wello" "world")"# };
        { expression: "(begin (define a (cons 1 2)) (define l (list 1 a)) (set-car! a 0) l)", expected: "(1 (0 . 2))" };
        { expression: "(begin (define a (list 1 2)) (define b (list 3 4)) (define l (list a b)) (set! a '(0 1)) (set-car! b 2) l)", expected: "((1 2) (2 4))" };
    }
}

#[test]
fn interpreter_factorial_basic() {
    integration_subtest_eval_to! {
        { expression: "(begin (define (fact n) (if (= n 0) 1 (* n (fact (- n 1))))) (fact 5))", expected: "120" };
    }
}

#[test]
fn interpreter_factorial_bigint() {
    integration_subtest_eval_to! {
        {
            expression: "(begin (define (fact n) (if (= n 0) 1 (* n (fact (- n 1))))) (fact 50))",
            expected: "30414093201713378043612608166064768844377641568960512000000000000"
        };
    }
}

#[test]
fn interpreter_apply() {
    integration_subtest_eval_to! {
        { expression: "(begin (define (f x) (* x 2)) (apply f '(4)))", expected: "8" };
        { expression: "(begin (define (f) (* 3 2)) (apply f '()))", expected: "6" };
        { expression: "(begin (define (f x y) (+ x y)) (apply f '(3 4)))", expected: "7" };
    }
}

#[test]
fn interpreter_quoting() {
    integration_subtest_eval_to! {
        { expression: "(begin (define x 5) 'x)", expected: "x" };
        { expression: "'(1 2 3)", expected: "(1 2 3)" };
        { expression: "'hello", expected: "hello" };
    }
}

#[test]
fn interpreter_binding() {
    integration_subtest_eval_to! {
        { expression: "(let ((x 2) (y 3)) (* x y))", expected: "6" };
        { expression: "(let ((x 2) (y 3)) (let ((x 7) (z (+ x y))) (* z x)))", expected: "35" };
        { expression: "(let ((x 2) (y 3)) (let* ((x 7) (z (+ x y))) (* z x)))", expected: "70" };
    }
}

#[test]
fn interpreter_flattening_unflattening() {
    integration_subtest_eval_to! {
        {
            expression: "(unflatten (flatten `(a `(b ,(+ 1 2) ,(foo ,(+ 1 3) d) e) f)))",
            expected: "(quasiquote (a (quasiquote (b (unquote (+ 1 2)) (unquote (foo (unquote (+ 1 3)) d)) e)) f))"
        };
    }
}

#[ignore]
#[test]
fn interpreter_quasiquotation() {
    integration_subtest_eval_to! {
        { expression: "(define x '(1 2 3))", expected: "ok" };
        { expression: "`(,x 3 ,pi ,(list 1 2 3 4 5))", expected: "((1 2 3) 3 3.141592653589793 (1 2 3 4 5))" };
        { expression: "`(,x ,x)", expected: "((1 2 3) (1 2 3))" };
        { expression: "`(,@x ,@x)", expected: "(1 2 3 1 2 3)" };
        { expression: "`(,x ,@x)", expected: "((1 2 3) 1 2 3)" };
        { expression: "`(1 2 ,(list 1 2 3))", expected: "(1 2 (1 2 3))" };
        { expression: "`(1 2 ,@(list 1 2 3))", expected: "(1 2 1 2 3)" };
        { expression: "`(,@x ,x)", expected: "(1 2 3 (1 2 3))" };
        { expression: "`(,@x ,x ,@x ,x ,@x)", expected: "(1 2 3 (1 2 3) 1 2 3 (1 2 3) 1 2 3)" };
        { expression: r#"(define x "unquoted")"#, expected: "ok" };
        { expression: r#"`(1 2 ,x (+ 3 ,x))"#, expected: r#"(1 2 "unquoted" (+ 3 "unquoted"))"# };
        { expression: "`(1 2 ,x `(1 2 ,x (+ 3 ,x)))", expected: r#"(1 2 "unquoted" (quasiquote (1 2 (unquote x) (+ 3 (unquote x)))))"# };
    }
}

#[test]
fn interpreter_recursion_deep() {
    integration_subtest_eval_to! {
        {
            expression: r#"(begin (define (count-to n) (if (= n 0) "Done!" (count-to (- n 1)))) (count-to 100000))"#,
            expected: r#""Done!""#
        };
    }
}

#[test]
fn interpreter_number_comparison() {
    integration_subtest_eval_to! {
        { expression: "(> 5 4 3 2 1)", expected: "#t"};
        { expression: "(>= 5 4 4 4.0 3 2 1 -4)", expected: "#t"};
        { expression: "(< 1 2 2 3)", expected: "#f"};
        { expression: "(<= 0.0 0.1 0.2 1 2 2.0 4/2 3)", expected: "#t"};
        { expression: "(= 2 2 2.0 2/1 4/2 6/3)", expected: "#t"};
    }
}

#[test]
fn interpreter_sexpr_type() {
    integration_subtest_eval_to! {
        { expression: "(boolean? #f)", expected: "#t" };
        { expression: r#"(string? "hello")"#, expected: "#t" };
        { expression: "(number? 1/2)", expected: "#t" };
        { expression: "(number? .11)", expected: "#t" };
        { expression: "(number? 100000000000000000000000)", expected: "#t" };
        { expression: "(exact? 100000000000000000000000)", expected: "#t" };
        { expression: "(exact? 10)", expected: "#t" };
        { expression: "(exact? 10.0)", expected: "#f" };
        { expression: "(exact? 1/2)", expected: "#t" };
        { expression: "(procedure? +)", expected: "#t" };
        { expression: "(procedure? apply)", expected: "#t" };
        { expression: "(procedure? eval)", expected: "#t" };
        { expression: "(list? '(1 2 3))", expected: "#t" };
        { expression: "(list? '(1 . 2))", expected: "#f" };
        { expression: "(pair? '(1 2 3))", expected: "#t" };
        { expression: "(pair? '(1 . 2))", expected: "#t" };
        { expression: "(symbol? 'hello)", expected: "#t" };
        { expression: r#"(symbol? "hello")"#, expected: "#f" };
    }
}

#[test]
fn interpreter_sexpr_null() {
    integration_subtest_eval_to! {
        { expression: "(null? '())", expected: "#t" };
        { expression: "(null? '(1 2 3))", expected: "#f" };
        { expression: "(null? 1)", expected: "#f" };
        { expression: "(null? #f)", expected: "#f" };
        { expression: "(null? 'hello)", expected: "#f" };
        { expression: "(null? 0)", expected: "#f" };
        { expression: "(null? 0.0)", expected: "#f" };
        { expression: "(null? 1/2)", expected: "#f" };
        { expression: "(null? +nan.0)", expected: "#f" };
        { expression: "(null? -inf.0)", expected: "#f" };
        { expression: "(null? +inf.0)", expected: "#f" };
    }
}

#[test]
fn interpreter_strings() {
    integration_subtest_eval_to! {
        { expression: r#"(string #\h #\e #\l #\l #\o)"#, expected: r#""hello""# };
        { expression: "(string-append \"hello, \" \"world\")", expected: r#""hello, world""# };
        { expression: r#"(string-downcase "HELLO")"#, expected: r#""hello""# };
        { expression: r#"(string-upcase "hello")"#, expected: r#""HELLO""# };
        { expression: r#"(string-upcase (string-downcase "HELLO"))"#, expected: r#""HELLO""# };
        { expression: "(make-string 7)", expected: r#""       ""# };
        { expression: "(make-string 3 #\\W)", expected: r#""WWW""# };
        { expression: r#"(string-length "hello")"#, expected: "5" };
        { expression: r#"(string-ref "hello" 1)"#, expected: r#"#\e"# };
        { expression: r#"(string-set! "hallo" 1 #\e)"#, expected: r#""hello""# };
    }

    integration_subtest_is_err! {
        expression: r#"(string-ref "hello" 5)"#;
        expression: "(string-set! \"hello\" 5 #\\e)";
    }
}
