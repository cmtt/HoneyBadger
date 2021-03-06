extern crate badger;

pub use badger::*;
pub use badger::grammar::*;
pub use badger::parser::parse;
pub use badger::grammar::OperatorType::*;

fn output_program(input_program: &str) -> String {
    let mut ast = parser::parse(input_program.to_string());
    transformer::transform(&mut ast, transformer::Settings::target_es5());
    codegen::generate_code(ast, true)
}

macro_rules! assert_compile {
    ($string:expr, $expect:expr) => {
        println!("{:?}", output_program($string));
        assert_eq!(output_program($string), $expect.to_string());
    }
}

#[test]
fn convert_const_to_var_in_global_scope() {
    assert_compile!("const pi = 314;\n", "var pi=314;");
}

#[test]
fn convert_let_to_var_in_global_scope() {
    assert_compile!("let pi = 314;\n", "var pi=314;");
}

#[test]
fn dont_touch_var_in_global_scope() {
    assert_compile!("var pi = 314;\n", "var pi=314;");
}

#[test]
fn convert_let_to_var_in_block() {
    let program = "if(true) {
      let pi = 3.14;
    }\n";

    let expected = "if(!0){var pi=3.14;}";

    assert_compile!(program, expected);
}

