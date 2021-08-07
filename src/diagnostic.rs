pub struct DiagnosticHolder {
    pub diagonistic_units: Vec<Unit>,
}

impl DiagnosticHolder {
    pub fn new() -> Self {
        Self {
            diagonistic_units: vec![],
        }
    }

    pub fn success(&self) -> bool {
        self.diagonistic_units.is_empty()
    }

    pub fn warning(&mut self, message: &str) {
        self.diagonistic_units
            .push(Unit::Warning(message.to_string()))
    }

    pub fn error(&mut self, message: &str) {
        self.diagonistic_units
            .push(Unit::Error(message.to_string()))
    }
}

pub enum Unit {
    Warning(String),
    Error(String),
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Warning(message) => format!("Warning: {}", message),
            Unit::Error(message) => format!("Error: {}", message)
        }
    }
}
