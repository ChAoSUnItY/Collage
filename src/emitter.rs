use crate::parser::CollageContext;

pub struct Emitter {
    ctx: Vec<CollageContext>,
    functions: Vec<String>,
}

impl Emitter {
    pub fn new(ctx: Vec<CollageContext>) -> Self {
        Self {
            ctx,
            functions: vec![],
        }
    }

    fn encode_vector(&self, vector: &mut Vec<u8>) -> Vec<u8> {
        vector.insert(0, vector.len() as u8);
        vector.to_vec()
    }

    pub fn emit(&mut self) -> Vec<u8> {
        let mut bytecode = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // Magic Number & Version

        let funtions: Vec<&CollageContext> = self
            .ctx
            .iter()
            .filter(|ctx| matches!(ctx, CollageContext::FunctionDeclaration(_, _, _)))
            .collect();

        self.emit_types(&mut bytecode, &funtions);

        bytecode
    }

    fn emit_section(&self, section_type: u8, len: u8, bytecode: &mut Vec<u8>) -> Vec<u8> {
        let mut section_bytecode = vec![section_type];
        bytecode.insert(0, len);
        section_bytecode.append(&mut self.encode_vector(bytecode));
        section_bytecode
    }

    fn emit_types(&self, bytecode: &mut Vec<u8>, functions: &Vec<&CollageContext>) {
        let mut function_types = self.emit_section(
            webassembly::SECTION_TYPE,
            functions.len() as u8,
            &mut functions
                .iter()
                .map(|f| self.emit_function_type(*f))
                .flatten()
                .collect(),
        );

        bytecode.append(&mut function_types);
    }

    fn emit_function_type(&self, function: &CollageContext) -> Vec<u8> {
        let mut function_type_vector = vec![webassembly::FUNC];

        if let CollageContext::FunctionDeclaration(_, argument_types, return_type) = function {
            let mut argument_types_vector = if argument_types.len() != 0 {
                self.encode_vector(
                    &mut argument_types
                        .iter()
                        .map(|s| self.emit_val_types(&s))
                        .collect(),
                )
            } else {
                vec![0x00]
            };
            let mut return_types_vector = if return_type != "()" {
                self.encode_vector(&mut vec![self.emit_val_types(&return_type)])
            } else {
                vec![0x00]
            };

            function_type_vector.append(&mut argument_types_vector);
            function_type_vector.append(&mut return_types_vector);
        }

        function_type_vector
    }

    fn emit_val_types(&self, type_name: &str) -> u8 {
        match type_name {
            "i32" => webassembly::I32,
            "i64" => webassembly::I64,
            "f32" => webassembly::F32,
            "f64" => webassembly::F64,
            _ => 0x00,
        }
    }
}
