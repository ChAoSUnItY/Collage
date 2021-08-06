#[macro_use]
extern crate colour;

use collage::compilation::Compilation;

fn main() {
    loop {
        magenta!("> ");

        let mut input_source_code = String::new();
        std::io::stdin().read_line(&mut input_source_code).expect("Unexpected reading error.");

        input_source_code = input_source_code.trim().to_string();

        if input_source_code == ":exit" {
            break;
        } else {
            let compilation = Compilation::new(input_source_code);
            yellow_ln!("{:}", compilation.eval());
        }
    }
}