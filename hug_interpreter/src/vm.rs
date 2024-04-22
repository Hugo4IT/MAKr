use std::{fs::OpenOptions, io::Read};

use hug_ast::{Expression, HugTree, HugTreeEntry};
use hug_core::HUG_CORE_SCRIPT;
use hug_lexer::{parser::generate_pairs, tokenizer::Tokenizer};
use hug_lib::{
    ffi_helpers::PackedArgs,
    function::HugFunction,
    ident_table::IdentTable,
    module::HugModule,
    value::{HugExternalFunction, HugValue},
    variables::Variables,
};

const INVALID_MODULE_ERROR: &str = "This module does not have the required functions, add one with hug_module! or contact the module's developer.";

#[derive(Debug)]
pub struct HugVM {
    paused: bool,
    pub pointer: usize,
    tree: HugTree,
    idents: IdentTable,
    variables: Variables,
    // external_libraries: Vec<Library>,
    // external_functions: Vec<Symbol<'lib>>,
}

impl HugVM {
    pub fn new(file_path: &str) -> HugVM {
        let mut vm = HugVM {
            paused: false,
            pointer: 0,
            tree: HugTree::new(),
            idents: IdentTable::new(),
            variables: Variables::new(),
        };

        vm.load_script(HUG_CORE_SCRIPT);
        vm.load_file(file_path);
        vm
    }

    pub fn next(&mut self) {
        if !self.paused {
            self.pointer += 1;
        }
    }

    pub fn load_file(&mut self, file_path: &str) {
        #[cfg(debug_assertions)]
        println!("Loading file: {}", file_path);

        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .unwrap_or_else(|_| panic!("Could not open file {}!", file_path));

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Could not read file!");

        self.load_script(&buffer);
    }

    pub fn load_script(&mut self, program: &str) {
        #[cfg(debug_assertions)]
        println!("Loading script:\n{}", program);

        let mut tokenizer = Tokenizer::new(&mut self.idents, program);
        let tokens = tokenizer.tokenize();

        let pairs = generate_pairs(program, tokens);
        let t = HugTree::from_token_pairs(pairs);
        self.tree.merge_with(t);
    }

    pub fn evaluate(&mut self, expression: &Expression) -> HugValue {
        match expression {
            Expression::Literal(value) => value.to_owned(),
            Expression::Call { function, args } => {
                match self.variables.get(*function).cloned().unwrap() {
                    HugValue::Function(f) => match f {
                        HugFunction::Hug { address, arguments } => {
                            self.pointer = address;
                            println!("No return value supported yet");

                            HugValue::Int32(0)
                        }
                        HugFunction::External { function_pointer } => {
                            let args = args
                                .iter()
                                .map(|a| Some(self.evaluate(a)))
                                .collect::<Vec<_>>();

                            let return_value = unsafe { function_pointer(PackedArgs::pack(&args)) };

                            return_value.unpack()
                        }
                    },
                    _ => panic!("Not a function! {:?}", function),
                }
            }
            Expression::Variable(variable) => self.variables.get(*variable).cloned().unwrap(),
        }
    }

    pub fn run_instruction(&mut self, instruction: HugTreeEntry) {
        #[cfg(debug_assertions)]
        println!("Instruction: {:?}", instruction);

        match instruction {
            hug_ast::HugTreeEntry::ModuleDefinition { module } => todo!(),
            hug_ast::HugTreeEntry::ExternalModuleDefinition {
                module,
                ref location,
            } => {
                self.variables
                    .set(module, HugValue::Module(HugModule::external(location)));
            }
            hug_ast::HugTreeEntry::Import { path } => {
                if path.len() <= 1 {
                    panic!("Invalid import.");
                }

                match self.variables.get_mut(path[0]) {
                    Some(HugValue::Module(module)) => {
                        let ident = path.last().cloned().unwrap();
                        let variable = module.import(&self.idents, &path[1..]);

                        self.variables.set(ident, variable);
                    }
                    _ => panic!("Invalid import."),
                }
            }
            hug_ast::HugTreeEntry::VariableDefinition {
                variable,
                ref value,
            } => {
                let value = self.evaluate(value);
                self.variables.set(variable, value);
            }
            hug_ast::HugTreeEntry::Expression(ref expression) => {
                self.evaluate(expression);
            }
            _ => (),
        }
    }

    pub fn run(&mut self) {
        let on_load = self.tree.on_load.drain(..).collect::<Vec<_>>();

        for instruction in on_load {
            self.run_instruction(instruction);
        }

        while self.pointer < self.tree.entries.len() {
            let instruction = self.tree.entries.get(self.pointer).unwrap().clone();

            self.run_instruction(instruction);

            self.next();
        }
    }
}
