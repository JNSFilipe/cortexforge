mod interpreter;
mod ir;
mod utils;
use interpreter::Interpreter;
use ir::str_to_ir;
use utils::{filter_chars, read_file};

fn main() {
    let file = "hello_world.bf";
    let mut data = read_file(file);
    data = filter_chars(&data);
    println!("\n{}\n\n", data);

    let ir = str_to_ir(data.clone());

    println!("{:?}", ir);
    Interpreter::new(ir).run();
}
