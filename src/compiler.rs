use crate::ir::{IntermRep, Token};
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;

pub struct Compiler {
    ir: IntermRep,
}

impl Compiler {
    pub fn new(ir: IntermRep) -> Compiler {
        Compiler { ir }
    }

    pub fn compile(&mut self) {
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

        // Return zero
        let i32_zero = i32_type.const_int(0, false);
        builder.build_return(Some(&i32_zero));

        self.produce_executable(module);
    }

    fn produce_executable(&self, module: inkwell::module::Module) {
        Target::initialize_all(&InitializationConfig::default());
        // use the host machine as the compilation target
        let target_triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        // make a target from the triple
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("{:?}", e))
            .unwrap();

        // make a machine from the target
        let target_machine = target
            .create_target_machine(
                &target_triple,
                &cpu,
                &features,
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| "Unable to create target machine!".to_string())
            .unwrap();

        // use the machine to convert our module to machine code and write the result to a file
        let output_filename = "out.a";
        target_machine
            .write_to_file(&module, FileType::Object, output_filename.as_ref())
            .map_err(|e| format!("{:?}", e))
            .unwrap();

        // TODO: call gcc to link the object file as explained in https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/
    }
}
