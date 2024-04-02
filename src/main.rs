mod compiler;
mod interpreter;
mod ir;
mod utils;
use compiler::Compiler;
use interpreter::Interpreter;
use ir::IntermRep;
use utils::{filter_chars, read_file};

fn main() {
    let file = "hello_world.bf";
    let mut data = read_file(file);
    data = filter_chars(&data);
    println!("\n{}\n\n", data);

    // Interpret from BrainFu*k
    let mut ir = IntermRep::new();
    ir.from_source_string(data);
    println!("{:?}", ir);
    Interpreter::new(ir.clone()).run();

    // Interpret from Text CortexForge
    let _ = ir.to_compiled_file("./compiled.cf");
    let _ = ir.from_compiled_file("./compiled.cf");
    println!("\n{:?}", ir);
    Interpreter::new(ir.clone()).run();

    // Interpret from Binary CortexForge
    let _ = ir.to_compiled_binary("./compiled.cfb");
    let _ = ir.from_compiled_binary("./compiled.cfb");
    println!("\n{:?}", ir);
    Interpreter::new(ir.clone()).run();

    // Try compiler
    let mut compiler = Compiler::new(ir);
    compiler.compile();
}
