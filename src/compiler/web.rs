#[cfg(target_arch = "wasm32")]
use futures::FutureExt;
use js_sys::{Function, JsOption, Promise};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::JsFuture;

use crate::compiler::{Runnable, RuntimeCompilerError, RuntimeCompilerTarget};
#[cfg(target_arch = "wasm32")]
use crate::{
    compiler::{Compiler, RuntimeCompilerTargetFuture, RuntimeYieldFuture},
    instruction::{self, Instruction},
    program::Program,
};

pub struct LEB128 {
    pub inner: Vec<u8>,
}

impl From<u32> for LEB128 {
    fn from(mut value: u32) -> Self {
        let mut inner = vec![];
        loop {
            let mut byte = value & 0x7f;
            value = value >> 7;
            if value != 0 {
                byte |= 0x80;
            }
            inner.push(byte as u8);
            if value == 0 {
                break;
            }
        }
        Self { inner }
    }
}

pub struct SLEB128 {
    pub inner: Vec<u8>,
}

impl From<i32> for SLEB128 {
    fn from(mut value: i32) -> Self {
        let mut inner = Vec::new();

        loop {
            let byte = (value & 0x7f) as u8;

            // arithmetic shift preserves sign
            value >>= 7;

            let sign_bit_set = (byte & 0x40) != 0;

            let done = (value == 0 && !sign_bit_set) || (value == -1 && sign_bit_set);

            if done {
                inner.push(byte);
                break;
            } else {
                inner.push(byte | 0x80);
            }
        }

        Self { inner }
    }
}

impl From<Option<f64>> for RuntimeCompilerError {
    fn from(value: Option<f64>) -> Self {
        if let Some(error) = value {
            match error {
                0.0 => RuntimeCompilerError::TypeError,
                1.0 => RuntimeCompilerError::CompileError,
                2.0 => RuntimeCompilerError::LinkError,
                3.0 => RuntimeCompilerError::RuntimeError,
                _ => RuntimeCompilerError::UnknownDefect,
            }
        } else {
            RuntimeCompilerError::UnknownDefect
        }
    }
}

pub struct WebAssembly(Function);

impl WebAssembly {
    pub fn new(function: Function) -> Self {
        Self(function)
    }
}

impl Runnable for WebAssembly {
    fn run(&self) -> () {
        let _ = self.0.call0(&JsValue::null());
    }
}

#[wasm_bindgen(module = "imports.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn extern_compile(
        get_chunk: &mut dyn FnMut() -> JsOption<JsValue>,
    ) -> Result<Promise<Function>, JsValue>;

    fn extern_yield() -> Promise<JsValue>;
}

pub type WebRuntimeCompilerTargetFuture = futures::future::Map<
    JsFuture<Function>,
    fn(Result<Function, JsValue>) -> Result<RuntimeCompilerTarget, RuntimeCompilerError>,
>;

pub type WebRuntimeYieldFuture =
    futures::future::Map<JsFuture<JsValue>, fn(Result<JsValue, JsValue>) -> ()>;

pub struct WebRuntimeCompiler;

/*

(module

  (import "env" "memory"
    (memory 1)
  )

  (import "imports.js" "extern_read"
    (func $extern_read
      (result i32)
    )
  )

  (import "imports.js" "extern_write"
    (func $extern_write
      (param i32)
    )
  )

  (import "imports.js" "runtime_error_tag"
    (tag $runtime_error_tag
        (param i32 i32 i32)
    )
  )

  (func (export "run")
    (local $cell_addr i32)
    (local $program_counter i32)
    (local $program_pointer i32)
    <emit>
))

*/
#[cfg(target_arch = "wasm32")]
const HEADER: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, // WASM_BINARY_MAGIC
    0x01, 0x00, 0x00, 0x00, // WASM_BINARY_VERSION
    // section "Type" (1)
    0x01, // section code
    0x15, // section size
    0x04, // num types
    // func type 0
    0x60, // func
    0x00, // num params
    0x01, // num results
    0x7f, // i32
    // func type 1
    0x60, // func
    0x02, // num params
    0x7f, // i32
    0x7f, // i32
    0x00, // num results
    // func type 2
    0x60, // func
    0x05, // num params
    0x7f, // i32
    0x7f, // i32
    0x7f, // i32
    0x7f, // i32
    0x7f, // i32
    0x00, // num results
    // func type 3
    0x60, // func
    0x00, // num params
    0x00, // num results
    // section "Import" (2)
    0x02, // section code
    0x62, // section size
    0x04, // num imports
    // import header 0
    0x03, // string length
    0x65, 0x6e, 0x76, // env  // import module name
    0x06, // string length
    0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, // memory  // import field name
    0x02, // import kind
    0x00, // limits: flags
    0x01, // limits: initial
    // import header 1
    0x0a, // string length
    0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x2e, 0x6a,
    0x73, // imports.js  // import module name
    0x0b, // string length
    0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f, 0x72, 0x65, 0x61,
    0x64, // extern_read  // import field name
    0x00, // import kind
    0x00, // import signature index
    // import header 2
    0x0a, // string length
    0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x2e, 0x6a,
    0x73, // imports.js  // import module name
    0x0c, // string length
    0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f, 0x77, 0x72, 0x69, 0x74,
    0x65, // extern_write  // import field name
    0x00, // import kind
    0x01, // import signature index
    // import header 3
    0x0a, // string length
    0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x2e, 0x6a,
    0x73, // imports.js  // import module name
    0x11, // string length
    0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x74,
    0x61, // runtime_error_tag
    0x67, // import field name
    0x04, // import kind
    0x00, // tag attribute
    0x02, // tag signature index
    // section "Function" (3)
    0x03, // section code
    0x02, // section size
    0x01, // num functions
    0x03, // function 0 signature index
    // section "Export" (7)
    0x07, // section code
    0x07, // section size
    0x01, // num exports
    0x03, // string length
    0x72, 0x75, 0x6e, // run  // export name
    0x00, // export kind
    0x02, // export func index
];

#[cfg(target_arch = "wasm32")]
const FOOTER: &[u8] = &[
    // section "name"
    0x00, // section code
    0x6f, // section size
    0x04, // string length
    0x6e, 0x61, 0x6d, 0x65, // name  // custom section name
    0x01, // name subsection type
    0x1c, // subsection size
    0x02, // num names
    0x00, // elem index
    0x0b, // string length
    0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f, 0x72, 0x65, 0x61,
    0x64, // extern_read  // elem name 0
    0x01, // elem index
    0x0c, // string length
    0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x5f, 0x77, 0x72, 0x69, 0x74,
    0x65, // extern_write  // elem name 1
    0x02, // local name type
    0x34, // subsection size
    0x03, // num functions
    0x00, // function index
    0x00, // num locals
    0x01, // function index
    0x00, // num locals
    0x02, // function index
    0x03, // num locals
    0x00, // local index
    0x09, // string length
    0x63, 0x65, 0x6c, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, // cell_addr  // local name 0
    0x01, // local index
    0x0f, // string length
    0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, // program_pointer  // local name 1
    0x02, // local index
    0x0f, // string length
    0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65,
    0x72, // program_pointer // local name 2
    0x0b, // name subsection type
    0x14, // subsection size
    0x01, // num names
    0x00, // elem index
    0x11, // string length
    0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x74,
    0x61, // runtime_error_tag // elem name 0
    0x67,
];

#[cfg(target_arch = "wasm32")]
impl Compiler for WebRuntimeCompiler {
    // source is not a full WASM binary, it is simply the concatenation of emit_wasm results from instructions.
    // compilation could fail, let's handle failures by matching the error ID.
    // in the case of compilation failure we can simply do nothing and let the interpreter run to completion.
    fn compile<'a>(
        &self,
        source: impl Iterator<Item = &'a instruction::InstructionSet>,
        program: &'a Program,
    ) -> Result<RuntimeCompilerTargetFuture, RuntimeCompilerError> {
        use crate::compiler::RuntimeCompilerTargetFuture;

        let mut source = Vec::from_iter(
            source
                .enumerate()
                .map(|(index, instruction)| {
                    let mut source = instruction.emit(program);
                    // don't add block if instruction is LeftJump
                    let mut result: Vec<u8> = match instruction {
                        instruction::InstructionSet::LeftJump(_) => vec![],
                        _ => vec![
                            0x02, // block
                            0x40, // void
                        ],
                    };
                    // if instruction is RightJump, index comparison is deferred to the end of the loop
                    match instruction {
                        instruction::InstructionSet::RightJump(_) => {}
                        _ => {
                            result.push(0x41); // i32.const
                            result.append(&mut SLEB128::from(index as i32).inner);
                            result.push(0x20); // local.get
                            result.push(0x01); // local index 1 load program pointer into stack
                            result.push(0x6b); // i32.sub sub program pointer from index
                            result.push(0x41); // i32.const
                            result.push(0x00); // 0
                            result.push(0x48); // i32.lt_s
                            result.push(0x0d); // br_if
                            match instruction {
                                instruction::InstructionSet::LeftJump(_) => {
                                    result.push(0x01); // break depth
                                }
                                _ => {
                                    result.push(0x00); // break depth
                                }
                            }

                            source.push(0x0b); // end block
                        }
                    }
                    result.append(&mut source);

                    result
                })
                .flatten(),
        );
        // function body 0
        let mut local_decl = vec![
            0x01, // local decl count
            0x03, // local type count
            0x7f, // i32
        ];

        let program = program as *const Program;
        let mut set_program_pointer = vec![0x41]; // i32.const
        set_program_pointer.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        set_program_pointer.push(0x28); // i32.load load program pointer into stack
        set_program_pointer.push(0x02); // alignment
        set_program_pointer.push(0x04); // load offset
        set_program_pointer.push(0x21); // local.set
        set_program_pointer.push(0x01); // local index 1 store program pointer in local variable 1
        let mut func_body =
            LEB128::from((local_decl.len() + set_program_pointer.len() + source.len() + 1) as u32)
                .inner; // func body size = local decl size + set_program_pointer size + source size + end opcode size
        func_body.append(&mut local_decl);
        func_body.append(&mut set_program_pointer);

        func_body.append(&mut source);
        func_body.append(&mut vec![0x0b]); // end
        // section "Code" (10)
        let mut num_functions = Vec::from(&[0x01]);
        let mut code_section = vec![0x0a];
        code_section
            .append(&mut LEB128::from((func_body.len() + num_functions.len()) as u32).inner); // code section size = function body size + num functions size
        code_section.append(&mut num_functions);
        code_section.append(&mut func_body);
        let mut source = std::iter::once(HEADER.to_vec())
            .chain(std::iter::once(
                code_section, // section code
            ))
            .chain(std::iter::once(FOOTER.to_vec()));
        let mut get_chunk =
            || JsOption::from_option(source.next().map(|value| JsValue::from(value)));

        match extern_compile(&mut get_chunk) {
            Ok(promise) => {
                fn to_result(
                    result: Result<Function, JsValue>,
                ) -> Result<RuntimeCompilerTarget, RuntimeCompilerError> {
                    result
                        .map(|function| WebAssembly::new(function).into())
                        .map_err(|js_value| js_value.as_f64().into())
                }
                let promise = promise.into_future().map(
                    to_result
                        as fn(
                            Result<Function, JsValue>,
                        )
                            -> Result<RuntimeCompilerTarget, RuntimeCompilerError>,
                );
                Ok(RuntimeCompilerTargetFuture::WebRuntimeCompilerTargetFuture(
                    promise,
                ))
            }
            Err(js_value) => Err(js_value.as_f64().into()),
        }
    }

    fn yield_now(&self) -> RuntimeYieldFuture {
        fn to_unit(_result: Result<JsValue, JsValue>) -> () {}
        RuntimeYieldFuture::WebRuntimeYieldFuture(
            JsFuture::from(extern_yield()).map(to_unit as fn(Result<JsValue, JsValue>) -> ()),
        )
    }
}
