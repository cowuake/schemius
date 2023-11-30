#[cfg(test)]
use crate::core::interpreter::Interpreter;

macro_rules! integration_subtest_eval_to {
    ($({expression:$expression:literal, expected:$expected_result:literal};)*) => {
        let mut interpreter = Interpreter::new();

        $(
            let res = interpreter.eval_expression_and_format(String::from($expression));
            let expected = String::from($expected_result);

            assert_eq!(res, expected);
        )*
    }
}

macro_rules! integration_subtest_is_err {
    ($(expression:$expression:literal;)*) => {
        let mut interpreter = Interpreter::new();

        $(
            let res = interpreter.eval_expression(String::from($expression));
            assert!(res.is_err())
        )*
    }
}

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
        { expression: "(begin (define a \"hello\") (define b \"world\") (define l (list a b)) (set! a \"farewell\") l)", expected: "(\"hello\" \"world\")" };
        { expression: "(begin (define a \"hello\") (define b \"world\") (define l (list a b)) (string-set! a 0 #\\W) l)", expected: "(\"Wello\" \"world\")" };
        { expression: "(begin (define a (cons 1 2)) (define l (list 1 a)) (set-car! a 0) l)", expected: "(1 (0 . 2))" };
        { expression: "(begin (define a (list 1 2)) (define b (list 3 4)) (define l (list a b)) (set! a '(0 1)) (set-car! b 2) l)", expected: "((1 2) (2 4))" };
    }
}

#[ignore] // Implement what needed in order to pass the test
#[test]
fn interpreter_r7rs_inter_variant() {
    integration_subtest_is_err! {
        // This returns an error in Cyclone Scheme and Gauche (R7RS), but not in Chez-Scheme nor in Chicken Scheme!
        expression: "(begin (define a (list 1 2)) (define b (list 3 4)) (define l (list a b)) (set! a '(0 1)) (set-car! b 2) (define c (cons a b)) (set-car! a 9))";
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
        { expression: "(define x \"unquoted\")", expected: "ok" };
        { expression: "`(1 2 ,x (+ 3 ,x))", expected: "(1 2 \"unquoted\" (+ 3 \"unquoted\"))" };
        { expression: "`(1 2 ,x `(1 2 ,x (+ 3 ,x)))", expected: "(1 2 \"unquoted\" (quasiquote (1 2 (unquote x) (+ 3 (unquote x)))))" };
    }
}

#[test]
fn interpreter_r7rs_quasiquotation() {
    integration_subtest_eval_to! {
        { expression: "`(list ,(+ 1 2) 4)", expected: "(list 3 4)" };
        { expression: "(let ((name 'a)) `(list ,name ',name))", expected: "(list a (quote a))" };
        { expression: "`((foo ,(- 10 3)) ,@(cdr '(c)) . ,(car '(cons)))", expected: "((foo 7) . cons)" };
        { expression: "`(10 5 ,(sqrt 4) ,@(map sqrt '(16 9)) 8)", expected: "#(10 5 2 4 3 8)" };
        { expression: "(let ((foo '(foo bar)) (@baz 'baz)) `(list ,@foo , @baz))", expected: "(list foo bar baz)" };
        { expression: "`(a `(b ,(+ 1 2) ,(foo ,(+ 1 3) d) e) f)", expected: "(a `(b ,(+ 1 2) ,(foo 4 d) e) f)" };
        { expression: "(let ((name1 'x) (name2 'y)) `(a `(b ,,name1 ,',name2 d) e))", expected: "(a `(b ,x ,'y d) e)" };
        { expression: "(quasiquote (list (unquote (+ 1 2)) 4))", expected: "(list 3 4)" };
        { expression: "'(quasiquote (list (unquote (+ 1 2)) 4))", expected: "`(list ,(+ 1 2) 4)" };
    }

    integration_subtest_is_err! {
        expression: "(begin (define x 10) `(1 2 ,@x))";
    }
}

#[test]
fn interpreter_r7rs_booleans() {
    integration_subtest_eval_to! {
        { expression: "#t", expected: "#t" };
        { expression: "#f", expected: "#f" };
        { expression: "'#f", expected: "#f" };
        { expression: "(not #t)", expected: "#f" };
        { expression: "(not 3)", expected: "#f" };
        { expression: "(not (list 3))", expected: "#f" };
        { expression: "(not #f)", expected: "#t" };
        { expression: "(not '())", expected: "#f" };
        { expression: "(not (list))", expected: "#f" };
        { expression: "(not 'nil)", expected: "#f" };
        { expression: "(boolean? #f)", expected: "#t" };
        { expression: "(boolean? 0)", expected: "#f" };
        { expression: "(boolean? '())", expected: "#f" };
    }
}

#[test]
fn interpreter_r7rs_conditionals() {
    integration_subtest_eval_to! {
        { expression: "(if (> 3 2) 'yes 'no)", expected: "yes" };
        { expression: "(if (> 2 3) 'yes 'no)", expected: "no" };
        { expression: "(if (> 3 2) (- 3 2) (+ 3 2))", expected: "1"};
        { expression: "(cond ((> 3 2) 'greater) ((< 3 2) 'less))", expected: "greater"};
        { expression: "(cond ((> 3 3) 'greater) ((< 3 3) 'less) else 'equal)", expected: "equal"};
    }
}

#[ignore] // TODO: Implement what is needed in order to pass the test
#[test]
fn interpreter_r7rs_equivalence_predicates() {
    integration_subtest_eval_to! {
        { expression: "(eqv? 'a 'a)", expected: "#t" };
        { expression: "(eqv? 'a 'b)", expected: "#f" };
        { expression: "(eqv? 2 2)", expected: "#t" };
        { expression: "(eqv? '() '())", expected: "#t" };
        { expression: "(eqv? 100000000 100000000)", expected: "#t" };
        { expression: "(eqv? 0.0 +nan.0)", expected: "#f" };
        { expression: "(eqv? (cons 1 2) (cons 1 2))", expected: "#f" };
        { expression: "(eqv? (lambda () 1) (lambda () 2))", expected: "#f" };
        { expression: "(let ((p (lambda (x) x))) (eqv? p p))", expected: "#t" };
        { expression: "(eqv? #f 'nil)", expected: "#f" };
        { expression: "(eqv? \"\" \"\")", expected: "" };
        { expression: "(eqv? '#() '#())", expected: "" };
        { expression: "(eqv? (lambda (x) x) (lambda (x) x))", expected: "" };
        { expression: "(eqv? (lambda (x) x) (lambda (y) y)", expected: "" };
        { expression: "(eqv? 1.0e0 1.0f0)", expected: "" };
        { expression: "(eqv? +nan.0 +nan.0", expected: "" };
        { expression: "(define gen-counter (lambda () (let ((n 0)) (lambda () (set! n (+ n 1)) n))))", expected: "ok" };
        { expression: "(let ((g gen-counter))) (eqv? g g))", expected: "#t" };
        { expression: "(eqv? (gen-counter) (gen-counter))", expected: "#f" };
        { expression: "(define gen-loser (lambda () (let ((n 0)) (lambda () (set! n (+ n 1)) 27))))", expected: "ok" };
        { expression: "(let ((g (gen-loser))) (eqv? g g)", expected: "#t" };
        { expression: "(eqv? (gen-loser) (gen-loser)", expected: "" };
        { expression: "(letrec ((f (lambda () (if (eqv? f g) 'both 'f))) (g (lambda () (if eqv? f g) 'both 'f)))) (eqv? f g)) ", expected: "" };
        { expression: "(letrec ((f (lambda () (if (eqv? f g) 'f 'both))) (g (lambda () (if eqv? f g) 'g 'both)))) (eqv? f g)) ", expected: "#f" };
        { expression: "(eqv? '(a) '(a))", expected: "" };
        { expression: "(eqv? \"a\" \"a\")", expected: "" };
        { expression: "(eqv? '(b) (cdr '(a b)))", expected: "" };
        { expression: "(let ((x '(a))) (eqv? x x))", expected: "#t" };
        { expression: "(eq? 'a 'a)", expected: "#t" };
        { expression: "(eq? '(a) '(a))", expected: "" };
        { expression: "(eq? (list 'a) (list 'a))", expected: "#f" };
        { expression: "(eq? \"a\" \"a\")", expected: "" };
        { expression: "(eq? '() '())", expected: "#t" };
        { expression: "(eq? 2 2)", expected: "" };
        { expression: "(eq? #\\A #\\A)", expected: "" };
        { expression: "(eq? car car)", expected: "#t" };
        { expression: "(let ((n (+ 2 3))) (eq? n n))", expected: "" };
        { expression: "(let ((x '(a))) (eq? x x))", expected: "#t" };
        { expression: "(let ((x '#())) (eq? x x))", expected: "#t" };
        { expression: "(let ((p (lambda (x) x))) (eq? p p))", expected: "#t" };
        { expression: "(equal? 'a 'a)", expected: "#t" };
        { expression: "(equal? '() '())", expected: "#t" };
        { expression: "(equal? '(a (b) c) '(a (b) c))", expected: "#t" };
        { expression: "(equal? \"abc\" \"abc\")", expected: "#t" };
        { expression: "(equal? 2 2)", expected: "#t" };
        { expression: "(equal? (make-vector 5 'a) (make-vector 5 'a))", expected: "#t" };
        { expression: "(equal? '#1=(a b . #1#) '#2=(a b a b . #2#))", expected: "#t" };
        { expression: "(equal? (lambda (x) x) (lambda (y) y))", expected: "" };
    }
}

#[test]
fn interpreter_r7rs_cons() {
    integration_subtest_eval_to! {
        { expression: "(cons 'a '())", expected: "(a)" };
        { expression: "(cons '(a) '(b c d))", expected: "((a) b c d)" };
        { expression: "(cons \"a\" '(b c))", expected: "(\"a\" b c)" };
        { expression: "(cons 'a 3)", expected: "(a . 3)" };
        { expression: "(cons '(a b) 'c)", expected: "((a b) . c)" };
    }
}

#[test]
fn interpreter_r7rs_control_features() {
    integration_subtest_eval_to! {
        { expression: "(procedure? car)", expected: "#t" };
        { expression: "(procedure? 'car)", expected: "#f" };
        { expression: "(procedure? (lambda (x) (* x x)))", expected: "#t" };
        { expression: "(procedure? '(lambda (x) (* x x)))", expected: "#f" };
    }
}

#[test]
fn interpreter_recursion_deep() {
    integration_subtest_eval_to! {
        {
            expression: "(begin (define (count-to n) (if (= n 0) \"Done!\" (count-to (- n 1)))) (count-to 100000))",
            expected: "\"Done!\""
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
fn interpreter_sepr_type() {
    integration_subtest_eval_to! {
        { expression: "(boolean? #f)", expected: "#t" };
        { expression: "(string? \"hello\")", expected: "#t" };
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
    }
}
