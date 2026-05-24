use futures::future::{Ready, ready};

use crate::{
    compiler::{
        Compiler, RuntimeCompilerError, RuntimeCompilerTarget, RuntimeCompilerTargetFuture,
    },
    instruction,
    program::Program,
};

pub struct NativeRuntimeCompiler;

impl Compiler for NativeRuntimeCompiler {
    type RuntimeCompilerTargetInnerFuture =
        Ready<Result<RuntimeCompilerTarget, RuntimeCompilerError>>;

    fn compile<'a>(
        &self,
        _source: impl Iterator<Item = &'a instruction::InstructionSet>,
        _program: &'a Program,
    ) -> Result<
        RuntimeCompilerTargetFuture<Self::RuntimeCompilerTargetInnerFuture>,
        RuntimeCompilerError,
    > {
        Err(RuntimeCompilerError::UnknownDefect)
    }

    fn yield_now(&self) -> impl Future<Output = ()> {
        ready(())
    }
}
