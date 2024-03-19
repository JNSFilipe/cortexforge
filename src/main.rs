mod interpreter;
mod ir;
mod utils;
use interpreter::Interpreter;
use ir::IntermRep;
use utils::{filter_chars, read_file};

fn main() {
    let file = "hello_world.bf";
    let mut data = read_file(file);
    data = filter_chars(&data);
    println!("\n{}\n\n", data);

    let mut ir = IntermRep::new(data.clone());
    ir.from_source_string(data);

    println!("{:?}", ir);
    Interpreter::new(ir).run();
}
