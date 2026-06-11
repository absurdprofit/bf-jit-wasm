pub mod native;
pub mod web;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use enum_dispatch::enum_dispatch;

use crate::{
    compiler::{
        native::NativeRuntimeCompilerTargetFuture,
        web::{WebRuntimeCompiler, WebRuntimeCompilerTargetFuture},
    },
    program::Program,
};
use crate::{
    compiler::{
        native::{NativeRuntimeCompiler, NativeRuntimeYieldFuture},
        web::{WebAssembly, WebRuntimeYieldFuture},
    },
    instruction::{self},
};

#[derive(Debug)]
pub enum RuntimeCompilerError {
    TypeError,
    CompileError,
    LinkError,
    RuntimeError,
    UnsupportedTarget,
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

pub enum RuntimeCompilerTargetFuture {
    WebRuntimeCompilerTargetFuture(WebRuntimeCompilerTargetFuture),
    NativeRuntimeCompilerTargetFuture(NativeRuntimeCompilerTargetFuture),
}

impl Future for RuntimeCompilerTargetFuture {
    type Output = Result<RuntimeCompilerTarget, RuntimeCompilerError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match &mut *self {
            Self::WebRuntimeCompilerTargetFuture(f) => Pin::new(f).poll(cx),
            Self::NativeRuntimeCompilerTargetFuture(f) => Pin::new(f).poll(cx),
        }
    }
}

pub enum RuntimeYieldFuture {
    WebRuntimeYieldFuture(WebRuntimeYieldFuture),
    NativeRuntimeYieldFuture(NativeRuntimeYieldFuture),
}

impl Future for RuntimeYieldFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match &mut *self {
            Self::WebRuntimeYieldFuture(f) => Pin::new(f).poll(cx),
            Self::NativeRuntimeYieldFuture(f) => Pin::new(f).poll(cx),
        }
    }
}

#[enum_dispatch]
pub trait Compiler {
    fn compile<'a>(
        &self,
        source: impl Iterator<Item = &'a instruction::InstructionSet>,
        program: &'a Program,
    ) -> Result<RuntimeCompilerTargetFuture, RuntimeCompilerError>;

    fn yield_now(&self) -> RuntimeYieldFuture;
}

#[enum_dispatch(Compiler)]
pub enum RuntimeCompiler {
    WebRuntimeCompiler(WebRuntimeCompiler),
    NativeRuntimeCompiler(NativeRuntimeCompiler),
}

impl RuntimeCompiler {
    pub fn new() -> Self {
        if cfg!(target_arch = "wasm32") {
            WebRuntimeCompiler.into()
        } else {
            NativeRuntimeCompiler.into()
        }
    }
}
