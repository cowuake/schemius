#[macro_export]
macro_rules! integration_subtest_eval_to {
    ($({expression:$expression:literal, expected:$expected_result:literal};)*) => {
        let mut interpreter = schemius::Interpreter::default();

        $(
            let res = interpreter.eval_expression_and_format(String::from($expression));
            let expected = $expected_result;

            assert_eq!(res, expected);
        )*
    }
}

#[macro_export]
macro_rules! integration_subtest_is_err {
    ($(expression:$expression:literal;)*) => {
        let mut interpreter = schemius::Interpreter::default();

        $(
            let res = interpreter.eval_expression(String::from($expression));
            assert!(res.is_err());
        )*
    }
}
