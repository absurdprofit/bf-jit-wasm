use futures::future::{Ready, ready};

use crate::{
    compiler::{
        Compiler, RuntimeCompilerError, RuntimeCompilerTarget, RuntimeCompilerTargetFuture,
        RuntimeYieldFuture,
    },
    instruction,
    program::Program,
};

pub type NativeRuntimeCompilerTargetFuture =
    Ready<Result<RuntimeCompilerTarget, RuntimeCompilerError>>;

pub type NativeRuntimeYieldFuture = Ready<()>;
pub struct NativeRuntimeCompiler;

impl Compiler for NativeRuntimeCompiler {
    fn compile<'a>(
        &self,
        _source: impl Iterator<Item = &'a instruction::InstructionSet>,
        _program: &'a Program,
    ) -> Result<RuntimeCompilerTargetFuture, RuntimeCompilerError> {
        Ok(
            RuntimeCompilerTargetFuture::NativeRuntimeCompilerTargetFuture(ready(Err(
                RuntimeCompilerError::UnsupportedTarget,
            ))),
        )
    }

    fn yield_now(&self) -> RuntimeYieldFuture {
        RuntimeYieldFuture::NativeRuntimeYieldFuture(ready(()))
    }
}
