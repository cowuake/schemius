#[macro_export]
macro_rules! integration_subtest_eval_to {
    ($({expression:$expression:literal, expected:$expected_result:literal};)*) => {
        let mut interpreter = schemius::Interpreter::default();

        $(
            match interpreter.eval_expression_and_format(String::from($expression)) {
                Ok(result) => assert_eq!(result, $expected_result),
                Err(err) => panic!("Error: {}", err),
            }
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
