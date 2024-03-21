use crate::ir::{IntermRep, Token};
use inkwell::context::Context;
use inkwell::module::Linkage;

struct Compiler {
    ir: IntermRep,
}

impl Compiler {
    pub fn new(ir: IntermRep) -> Compiler {
        Compiler { ir }
    }

    pub fn compile(&self) {
        let context = Context::create();
        let module = context.create_module("cortexforge");
        let builder = context.create_builder();

        // Declare main function, takes no arguments and returns i32
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        // let fn_main = module.add_function("main", fn_type, None);
        let fn_main = module.add_function("main", fn_type, Some(Linkage::External));

        // Set the instruction pointer to the first instruction of the main function
        let basic_block = context.append_basic_block(fn_main, "entry");
        builder.position_at_end(basic_block);

        // let i32_zero = i32_type.const_zero();
        // let i32_one = i32_type.const_int(1, false);
        // let i32_two = i32_type.const_int(2, false);

        // let add = builder.build_int_add(i32_one, i32_two, "add");
        // let sub = builder.build_int_sub(add, i32_one, "sub");
        // let mul = builder.build_int_mul(sub, i32_two, "mul");
        // let div = builder.build_int_unsigned_div(mul, i32_two, "div");

        // builder.build_return(Some(&div));
        // module.print_to_file("output.ll").unwrap();
    }
}
