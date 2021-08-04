use crate::parser::SyntaxNode;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}

pub fn print_syntax_tree<T: SyntaxNode<T> + PartialEq>(node: &Box<Option<T>>, mut indent: String, is_last: bool) {
    if let Some(syntax_node) = node.as_ref() {
        let marker = if is_last {
            "└──"
        } else {
            "├──"
        };

        print!("{}", indent);
        print!("{}", marker);
        print!("{:}", syntax_node.as_string());

        syntax_node.print();

        println!();

        indent.push_str(if is_last {
            "   "
        } else {
            "│  "
        });

        let last = syntax_node.children();

        for child in syntax_node.children() {
            print_syntax_tree(&child, indent.clone(), child == *last.last().unwrap());
        }
    }
}
