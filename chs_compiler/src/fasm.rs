
use core::fmt;

/// A subset of Fasm mapped into Rust data structures.
///
/// Example:
/// ```
/// pub fn foo() -> Fasm {
///     let mut f = Fasm::default();
///     f.set_entry("main");
///     f.push_macro(FasmMacro::equ_const("SYS_EXIT", "60"));
///     f.push_macro(FasmMacro::equ_const("SYS_WRITE", "1"));
///     f.push_macro(FasmMacro::equ_const("STDOUT_FILENO", "1"));
///     f.push_macro(FasmMacro::equ_const("EXIT_OK", "0"));
///     f.push_macro(FasmMacro::Struc {
///         name: "string".to_string(),
///         args: vec![
///             "[data]".to_string()
///         ],
///         body: vec![
///             "common".to_string(),
///             ". db data".to_string(),
///             ".len = $ - .".to_string(),
///         ]
///     });
///     let seg = f.push_segment(Segment::new(true, false, false));
///     seg.add_comment("code");
///     seg.add_label("main");
///     seg.add_insruction("mov rax, SYS_WRITE");
///     seg.add_insruction("mov rdi, STDOUT_FILENO");
///     seg.add_insruction("mov rsi, msg");
///     seg.add_insruction("mov rdx, [rsi+string.len]");
///     seg.add_insruction("syscall");
///
///     seg.add_insruction("mov rax, SYS_EXIT");
///     seg.add_insruction("mov rdi, EXIT_OK");
///     seg.add_insruction("syscall");
///
///     let seg = f.push_segment(Segment::new(false, true, true));
///     seg.add_comment("data");
///     seg.add_data("msg string \"Hello, world!\", 10");
///     return f;
/// }
/// ```
#[derive(Default)]
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
