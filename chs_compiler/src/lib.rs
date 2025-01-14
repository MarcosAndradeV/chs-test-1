#![allow(unused)]

use fasm::{Fasm,FasmMacro, Segment};

mod fasm;

pub fn foo() -> Fasm {
    let mut f = Fasm::default();
    f.set_entry("main");
    f.push_macro(FasmMacro::equ_const("SYS_EXIT", "60"));
    f.push_macro(FasmMacro::equ_const("SYS_WRITE", "1"));
    f.push_macro(FasmMacro::equ_const("STDOUT_FILENO", "1"));
    f.push_macro(FasmMacro::equ_const("EXIT_OK", "0"));
    f.push_macro(FasmMacro::Struc {
        name: "string".to_string(),
        args: vec![
            "[data]".to_string()
        ],
        body: vec![
            "common".to_string(),
            ". db data".to_string(),
            ".len = $ - .".to_string(),
        ]
    });
    let seg = f.push_segment(Segment::new(true, false, false));
    seg.add_comment("code");
    seg.add_label("main");
    seg.add_insruction("mov rax, SYS_WRITE");
    seg.add_insruction("mov rdi, STDOUT_FILENO");
    seg.add_insruction("mov rsi, msg");
    seg.add_insruction("mov rdx, msg.len");
    seg.add_insruction("syscall");

    seg.add_insruction("mov rax, SYS_EXIT");
    seg.add_insruction("mov rdi, EXIT_OK");
    seg.add_insruction("syscall");

    let seg = f.push_segment(Segment::new(false, true, true));
    seg.add_comment("data");
    seg.add_data("msg string \"Hello, world!\", 10");
    return f;
}
