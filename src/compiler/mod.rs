pub mod web;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use enum_dispatch::enum_dispatch;
#[cfg(target_arch = "wasm32")]
use futures::FutureExt;
#[cfg(not(target_arch = "wasm32"))]
use futures::future::{Ready, ready};
use js_sys::{Function, JsOption, Promise, futures::JsFuture};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::compiler::web::{LEB128, SLEB128};
use crate::program::Program;
use crate::{
    compiler::web::WebAssembly,
    instruction::{self, Instruction},
};

pub enum RuntimeCompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    UnknownDefect,
}

pub struct RuntimeCompiler;

pub trait Compiler {
    type RuntimeCompilerTargetInnerFuture: Unpin
        + Future<Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>>;
    fn compile<'a>(
        source: impl Iterator<Item = &'a instruction::InstructionSet>,
        program: &'a Program,
    ) -> Result<
        RuntimeCompilerTargetFuture<Self::RuntimeCompilerTargetInnerFuture>,
        RuntimeCompilerError,
    >;

    fn yield_now() -> impl Future<Output = ()>;
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

#[cfg(not(target_arch = "wasm32"))]
impl Compiler for RuntimeCompiler {
    type CompileFuture = Ready<Result<RuntimeCompilerTarget, RuntimeCompilerError>>;

    fn compile<'a>(
        _source: impl Iterator<Item = &'a instruction::InstructionSet>,
        _program: &'a Program,
    ) -> Result<Self::CompileFuture, RuntimeCompilerError> {
        Err(RuntimeCompilerError::UnknownDefect)
    }

    fn yield_now() -> impl Future<Output = ()> {
        ready(())
    }
}
