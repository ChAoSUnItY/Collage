#[macro_use]
extern crate colour;

use collage::compilation::Compilation;

fn main() {
    loop {
        magenta!("> ");

        let mut input_source_code = String::new();
        std::io::stdin()
            .read_line(&mut input_source_code)
            .expect("Unexpected reading error.");

        input_source_code = input_source_code.trim().to_string();

        match &*input_source_code {
            ":exit" => break,
            ":cls" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            _ => {
                let mut compilation = Compilation::new(input_source_code);
                let expression = compilation.lex_parse();
                let bound_expression = compilation.bind_tree(expression);

                if compilation.holder.success() {
                    yellow_ln!("{:}", compilation.eval_expression(bound_expression));
                } else {
                    for i in compilation.holder.diagonistic_units {
                        red_ln!("{:}", i.to_string());
                    }
                }
            }
        }
    }
}
