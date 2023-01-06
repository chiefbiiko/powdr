use lalrpop_util::*;

pub mod ast;

lalrpop_mod!(pil, "/parser/pil.rs");

pub fn parse(input: &str) -> Result<ast::PILFile, ParseError<usize, lexer::Token, &str>> {
    pil::PILFileParser::new().parse(input)
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use ast::*;

    #[test]
    fn empty() {
        assert!(pil::PILFileParser::new().parse("").is_ok());
    }

    #[test]
    fn simple_include() {
        let parsed = pil::PILFileParser::new().parse("include \"x\";").unwrap();
        assert_eq!(parsed, PILFile(vec![Statement::Include("x".to_string())]));
    }

    fn parse_file(name: &str) -> PILFile {
        let input = fs::read_to_string(name).unwrap();
        parse(&input).unwrap()
    }

    #[test]
    fn parse_example_files() {
        parse_file("test_files/global.pil");
    }
}
