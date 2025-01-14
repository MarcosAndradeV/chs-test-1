
use core::fmt;

#[derive(Default)]
/// A subset of Fasm mapped into Rust data structures.
pub struct Fasm {
    entry: String,
    macros: Vec<FasmMacro>,
    segments: Vec<Segment>,
}

impl Fasm {
    /// Set entry label for the program
    ///
    /// `entry label`
    pub fn set_entry(&mut self, entry: impl ToString) {
        self.entry = entry.to_string();
    }
    /// Add a segment to the program
    pub fn push_segment(&mut self, segment: Segment) -> &mut Segment {
        self.segments.push(segment);
        self.segments.last_mut().unwrap()
    }
    /// Add a macro to the program
    pub fn push_macro(&mut self, fasm_macro: FasmMacro)  {
        self.macros.push(fasm_macro);
    }
}

impl fmt::Display for Fasm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "format ELF64 executable")?;

        for ele in &self.macros {
            writeln!(f, "{ele}")?;
        }

        if !self.entry.is_empty() {writeln!(f, "entry {}", self.entry)?;}

        writeln!(f, "")?;

        for ele in &self.segments {
            writeln!(f, "{ele}")?;
        }

        writeln!(f, "")?;
        Ok(())
    }
}

pub enum FasmMacro {
    Struc {
        name: String,
        args: Vec<String>,
        body: Vec<String>
    },
    EquConst(String, String)
}

impl FasmMacro {
    pub fn equ_const(name: impl ToString, val: impl ToString) -> Self {
        Self::EquConst(name.to_string(), val.to_string())
    }
}

impl fmt::Display for FasmMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struc { name, args, body } => {
                write!(f, "struc {name} ")?;
                for (i, ele) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ele)?;
                }
                writeln!(f, " {{")?;
                for ele in body {
                    writeln!(f, "{ele}",)?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
            Self::EquConst(name, val) => writeln!(f, "{name} equ {val}"),
        }
    }
}

/// Representation of Fasm segment
///
/// `segment executable readable writable`
pub struct Segment {
    executable: bool,
    readable: bool,
    writable: bool,
    instructions: Vec<String>,
    data: Vec<String>,
}

impl Segment {
    pub fn new(executable: bool, readable: bool, writable: bool) -> Self {
        Self {
            executable,
            readable,
            writable,
            instructions: vec![],
            data: vec![],
        }
    }

    pub fn add_label(&mut self, label: impl ToString) {
        self.instructions.push(format!("{}:", label.to_string()));
    }

    pub fn add_comment(&mut self, comment: impl ToString) {
        self.instructions.push(format!(";; {}", comment.to_string()));
    }

    pub fn add_insruction(&mut self, insruction: impl ToString) {
        self.instructions.push(format!("    {}", insruction.to_string()));
    }

    pub fn add_data(&mut self, data: impl ToString) {
        self.data.push(data.to_string());
    }

}

// pub fn add_function_prologe(&mut self, name: impl ToString, stack_alloc_bytes: usize) {
//     self.add_label(name);
//     self.add_insruction("push rbp");
//     self.add_insruction("mov rbp, rsp");
//     if stack_alloc_bytes > 0 { self.add_insruction(format!("sub rsp, {}", stack_alloc_bytes)); }
// }

// pub fn add_function_eploge(&mut self, name: impl ToString, stack_alloc_bytes: usize) {
//     self.add_comment(name);
//     if stack_alloc_bytes > 0 { self.add_insruction(format!("sub rsp, {}", stack_alloc_bytes)); }
//     self.add_insruction("pop rbp");
//     self.add_insruction("ret");
// }

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut perm = String::new();
        if self.executable {
            perm.push_str("executable ");
        }
        if self.readable {
            perm.push_str("readable ");
        }
        if self.writable {
            perm.push_str("writable ");
        }
        writeln!(f, "segment {perm}")?;

        for ele in &self.instructions {
            writeln!(f, "{ele}")?;
        }

        for ele in &self.data {
            writeln!(f, "{ele}")?;
        }

        Ok(())
    }
}
