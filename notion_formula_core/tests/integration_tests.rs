#[cfg(test)]
mod test {
    use notion_formula_core::parser::BooleanOperator::*;
    use notion_formula_core::parser::ComparisonOperator::*;
    use notion_formula_core::parser::Expression::*;
    use notion_formula_core::*;
    use std::fs::File;

    #[test]
    fn test_input_string_to_ast() {
        let mut file = File::open("tests/test_formula.notion").unwrap();
        let input: Vec<char> = reader::read(&mut file).unwrap();
        let tokens = tokenizer::tokenizer(input).unwrap();
        let ast: parser::Expression = parser::formula_parser(tokens).unwrap();

        assert_eq!(
            Call(
                Box::new(Identifier("if".into())),
                vec![
                    BooleanOp(
                        Box::new(Comparison(
                            Box::new(Call(
                                Box::new(Identifier("prop".into())),
                                vec![Str("\"State\"".into())]
                            )),
                            Equals,
                            Box::new(Str("\"⚪\"".into()))
                        )),
                        Or,
                        Box::new(Comparison(
                            Box::new(Call(
                                Box::new(Identifier("prop".into())),
                                vec![Str("\"Estimated Completion Date\"".into())]
                            )),
                            Equals,
                            Box::new(Str("\"⏳ Waiting...\"".into()))
                        ))
                    ),
                    Str("\"🟨\"".into()),
                    Call(
                        Box::new(Identifier("if".into())),
                        vec![
                            Comparison(
                                Box::new(Call(
                                    Box::new(Identifier("prop".into())),
                                    vec![Str("\"State\"".into())]
                                )),
                                Equals,
                                Box::new(Str("\"🔵\"".into()))
                            ),
                            Str("\"🟩\"".into()),
                            Str("\"🟥\"".into())
                        ]
                    )
                ]
            ),
            ast
        )
    }
}
