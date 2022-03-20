use inkwell::{context::Context, module::Module, builder::Builder, values::BasicValueEnum};

use crate::parser::{Operation, Proc};

#[derive(Debug)]
pub struct Compiler<'a, 'ctx> {
    pub builder: &'a Builder<'ctx>,
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub stack: Vec<BasicValueEnum<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(builder: &'a Builder<'ctx>, context: &'ctx Context, module: &'a Module<'ctx>) -> Compiler<'a, 'ctx> {
        Compiler {
            builder,
            context,
            module,
            stack: Vec::new(),
        }
    }

    pub fn compile_proc(&mut self, proc: &Proc) {
        let function = self.module.add_function(
            &proc.name,
            self.context.i64_type().fn_type(&[], false),
            None
        );

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        proc.ops.iter().for_each(|op| {
            match *op {
                Operation::Integer(i) => {
                    self.stack.push(BasicValueEnum::IntValue(
                        self.context.i64_type().const_int(
                            i, false
                        )
                    ))
                },
    
                // Arithmetic
                Operation::Add => {
                    let y = self.stack.pop().unwrap().into_int_value();
                    let x = self.stack.pop().unwrap().into_int_value();
    
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_add(x, y, "tmpadd")));
                },
                Operation::Sub => {
                    let y = self.stack.pop().unwrap().into_int_value();
                    let x = self.stack.pop().unwrap().into_int_value();
    
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_sub(x, y, "tmpsub")));
                },
                Operation::Mul => {
                    let y = self.stack.pop().unwrap().into_int_value();
                    let x = self.stack.pop().unwrap().into_int_value();
    
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_mul(x, y, "tmpmul")));
                },
                Operation::DivMod => {
                    let y = self.stack.pop().unwrap().into_int_value();
                    let x = self.stack.pop().unwrap().into_int_value();
    
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_unsigned_div(x, y, "tmpdiv")));
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_unsigned_rem(x, y, "tmpmod")));
                },
                Operation::IDivMod => {
                    let y = self.stack.pop().unwrap().into_int_value();
                    let x = self.stack.pop().unwrap().into_int_value();
    
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_signed_div(x, y, "tmpidiv")));
                    self.stack.push(BasicValueEnum::IntValue(self.builder.build_int_signed_rem(x, y, "tmpimod")));
                },

                // Intrinsics
                Operation::Drop => {
                    self.stack.pop();
                },
                Operation::Print => {
                    let x = self.stack.pop().unwrap().into_int_value();
                    unimplemented!();
                }
            }
        });

        self.builder.build_return(Some(&self.stack.pop().unwrap().into_int_value()));

        assert!(function.verify(true));
    }
}
