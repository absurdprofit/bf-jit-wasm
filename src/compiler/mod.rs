pub mod native;
pub mod web;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use enum_dispatch::enum_dispatch;

use crate::{compiler::web::WebRuntimeCompiler, program::Program};
use crate::{
    compiler::{native::NativeRuntimeCompiler, web::WebAssembly},
    instruction::{self},
};

pub enum RuntimeCompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    UnknownDefect,
}

#[enum_dispatch]
pub trait Runnable {
    fn run(&self) -> ();
}

#[enum_dispatch(Runnable)]
pub enum RuntimeCompilerTarget {
    WebAssembly,
}

pub struct RuntimeCompilerTargetFuture<
    F: Unpin + Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>,
> {
    inner: F,
}

impl<F: Unpin + Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>>
    RuntimeCompilerTargetFuture<F>
{
    fn new(inner: F) -> Self {
        Self { inner }
    }
}

impl<F: Unpin + Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>> Future
    for RuntimeCompilerTargetFuture<F>
{
    type Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let pinned = Pin::new(&mut self.inner);
        pinned.poll(cx)
    }
}

pub trait Compiler {
    type RuntimeCompilerTargetInnerFuture: Unpin
        + Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>;
    fn compile<'a>(
        &self,
        source: impl Iterator<Item = &'a instruction::InstructionSet>,
        program: &'a Program,
    ) -> Result<
        RuntimeCompilerTargetFuture<Self::RuntimeCompilerTargetInnerFuture>,
        RuntimeCompilerError,
    >;

    fn yield_now(&self) -> impl Future<Output = ()>;
}

pub enum RuntimeCompiler {
    WebRuntimeCompiler(WebRuntimeCompiler),
    NativeRuntimeCompiler(NativeRuntimeCompiler),
}
