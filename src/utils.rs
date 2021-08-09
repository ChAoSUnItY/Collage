use std::any::{Any, TypeId};
use std::fmt::Display;

use crate::parser::SyntaxNode;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn to_string<T: Display + 'static>(any: &dyn Any) -> Result<String, &'static str> {
    if any.type_id() == TypeId::of::<String>() {
        return Ok(any.downcast_ref::<String>().unwrap().clone());
    }

    if any.type_id() == TypeId::of::<T>() {
        if let Some(val) = any.downcast_ref::<T>() {
            return Ok(val.to_string());
        }
    }

    Err("")
}

pub fn print_syntax_tree<T: SyntaxNode<T> + PartialEq>(
    node: &Box<Option<T>>,
    mut indent: String,
    is_last: bool,
) {
    if let Some(syntax_node) = node.as_ref() {
        let marker = if is_last { "└──" } else { "├──" };

        print!("{}", indent);
        print!("{}", marker);
        print!("{:}", syntax_node.as_string());

        syntax_node.print();

        println!();

        indent.push_str(if is_last { "   " } else { "│  " });

        let last = syntax_node.children();

        for child in syntax_node.children() {
            print_syntax_tree(&child, indent.clone(), child == *last.last().unwrap());
        }
    }
}

pub fn get_syntax_tree<T: SyntaxNode<T> + PartialEq>(
    builder: &mut String,
    node: &Box<Option<T>>,
    mut indent: String,
    is_last: bool,
) -> String {
    if let Some(syntax_node) = node.as_ref() {
        let marker = if is_last { "└──" } else { "├──" };

        builder.push_str(&*indent);
        builder.push_str(marker);
        builder.push_str(&*format!("{:}", syntax_node.as_string()));
        builder.push('\n');

        indent.push_str(if is_last { "   " } else { "│  " });

        let last = syntax_node.children();

        for child in syntax_node.children() {
            get_syntax_tree(
                builder,
                &child,
                indent.clone(),
                child == *last.last().unwrap(),
            );
        }
    }

    builder.clone()
}
