mod common;

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
        // { expression: "(cond ((assv 'b '((a 1) (b 2))) => cadr) (else #f))", expected: "2" };
        // { expression: "(case (* 2 3) ((2 3 5 7) 'prime) ((1 4 6 8 9) 'composite))", expected: "composite" };
        // { expression: "(case (car '(c d)) ((a) 'a) ((b) 'b))", expected: "unspecified" };
        // { expression: "(case (car '(c d)) ((a e i o u) 'vowel) ((w y) 'semivowel) (else => (lambda (x) x)))", expected: "c" };
        { expression: "(and (= 2 2) (> 2 1))", expected: "#t" };
        { expression: "(and (= 2 2) (< 2 1))", expected: "#f" };
        { expression: "(and 1 2 'c '(f g))", expected: "(f g)" };
        { expression: "(and)", expected: "#t" };
        { expression: "(or (= 2 2) (> 2 1))", expected: "#t" };
        { expression: "(or (= 2 2) (< 2 1))", expected: "#t" };
        { expression: "(or #f #f #f)", expected: "#f" };
        // { expression: "(or (memq 'b '(a b c)) (/ 3 0))", expected: "(b c)" };
        // { expression: "(when (= 1 1.0) (display \"1\") (display \"2\"))", expected: "unspecified" /* (and prints 12) */ };
        // { expression: "(unless (= 1 1.0) (display \"1\") (display \"2\"))", expected: "unspecified" /* (and prints nothing) */ };
    }
}

#[test]
fn interpreter_r7rs_cons() {
    integration_subtest_eval_to! {
        { expression: "(cons 'a '())", expected: "(a)" };
        { expression: "(cons '(a) '(b c d))", expected: "((a) b c d)" };
        { expression: r#"(cons "a" '(b c))"#, expected: r#"("a" b c)"# };
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
        { expression: r#"(eqv? "" "")"#, expected: "" };
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
        { expression: r#"(eqv? "a" "a")"#, expected: "" };
        { expression: "(eqv? '(b) (cdr '(a b)))", expected: "" };
        { expression: "(let ((x '(a))) (eqv? x x))", expected: "#t" };
        { expression: "(eq? 'a 'a)", expected: "#t" };
        { expression: "(eq? '(a) '(a))", expected: "" };
        { expression: "(eq? (list 'a) (list 'a))", expected: "#f" };
        { expression: r#"(eq? "a" "a")"#, expected: "" };
        { expression: "(eq? '() '())", expected: "#t" };
        { expression: "(eq? 2 2)", expected: "" };
        { expression: r#"(eq? #\A #\A)"#, expected: "" };
        { expression: "(eq? car car)", expected: "#t" };
        { expression: "(let ((n (+ 2 3))) (eq? n n))", expected: "" };
        { expression: "(let ((x '(a))) (eq? x x))", expected: "#t" };
        { expression: "(let ((x '#())) (eq? x x))", expected: "#t" };
        { expression: "(let ((p (lambda (x) x))) (eq? p p))", expected: "#t" };
        { expression: "(equal? 'a 'a)", expected: "#t" };
        { expression: "(equal? '() '())", expected: "#t" };
        { expression: "(equal? '(a (b) c) '(a (b) c))", expected: "#t" };
        { expression: r#"(equal? "abc" "abc")"#, expected: "#t" };
        { expression: "(equal? 2 2)", expected: "#t" };
        { expression: "(equal? (make-vector 5 'a) (make-vector 5 'a))", expected: "#t" };
        { expression: "(equal? '#1=(a b . #1#) '#2=(a b a b . #2#))", expected: "#t" };
        { expression: "(equal? (lambda (x) x) (lambda (y) y))", expected: "" };
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

#[ignore]
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
fn interpreter_r7rs_string() {
    integration_subtest_eval_to! {
        { expression: r#"(define (f) (make-string 3 #\*))"#, expected: "ok" };
        { expression: r#"(define (g) "***")"#, expected: "ok" };
        // { expression: r#"(string-set! (f) 0 #\?)"#, expected: "unspecified" };
    }

    integration_subtest_is_err! {
        expression: r#"(string-set! (g) 0 #\?)"#;
        expression: r#"(string-set! (symbol->string 'immutable) 0 #\?)"#;
    }
}

#[test]
fn interpreter_r7rs_numbers_is() {
    integration_subtest_eval_to! {
        // { expression: "(complex? 3+4i)", expected: "#t" };
        // { expression: "(complex? 3)", expected: "#t" };
        { expression: "(real? 3)", expected: "#t" };
        // { expression: "(real? -2.5+0i)", expected: "#t" };
        // { expression: "(real? -2.5+0.0i) ", expected: "#f" };
        { expression: "(real? #e1e10)", expected: "#t" };
        { expression: "(real? +inf.0)", expected: "#t" };
        { expression: "(real? +nan.0)", expected: "#t" };
        { expression: "(rational? -inf.0)", expected: "#f" };
        { expression: "(rational? 3.5)", expected: "#t" };
        { expression: "(rational? 6/10)", expected: "#t" };
        { expression: "(rational? 6/3)", expected: "#t" };
        // { expression: "(integer? 3+0i)", expected: "#t" };
        { expression: "(integer? 3.0)", expected: "#t" };
        { expression: "(integer? 8/4)", expected: "#t" };
        { expression: "(exact? 3.0)", expected: "#f" };
        { expression: "(exact? #e3.0)", expected: "#t" };
        { expression: "(inexact? 3.)", expected: "#t" };
        { expression: "(exact-integer? 32)", expected: "#t" };
        { expression: "(exact-integer? 32.0)", expected: "#f" };
        { expression: "(exact-integer? 32/5)", expected: "#f" };
        { expression: "(finite? 3)", expected: "#t" };
        { expression: "(finite? +inf.0)", expected: "#f" };
        // { expression: "(finite? 3.0+inf.0i)", expected: "#f" };
        { expression: "(infinite? 3)", expected: "#f" };
        { expression: "(infinite? +inf.0)", expected: "#t" };
        { expression: "(infinite? +nan.0)", expected: "#f" };
        // { expression: "(infinite? 3.0+inf.0i)", expected: "#t" };
        { expression: "(nan? +nan.0)", expected: "#t" };
        { expression: "(nan? 32)", expected: "#f" };
        // { expression: "(nan? +nan.0+5.0i)", expected: "#t" };
        // { expression: "(nan? 1+2i) ", expected: "#f" };
    }
}
