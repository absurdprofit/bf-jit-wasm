class Output {
  #decoder = new TextDecoderStream();
  #output = new TransformStream();
  #writer = this.#output.writable.getWriter();
  readable = this.#decoder.readable;

  constructor() {
    this.#output.readable.pipeTo(this.#decoder.writable);
  }

  write(byte) {
    this.#writer.write(Uint8Array.of(byte));
  }
}

export function extern_yield() {
  return new Promise(resolve => setTimeout(resolve, 0));
}

export function extern_read() {
  const input = prompt("Input a character");
  return input.charCodeAt(0);
}

const runtime_error_tag = new WebAssembly.Tag({ parameters: ["i32", "i32", "i32", "i32", "i32"] });

export const output = new Output();
export function extern_write(byte, count = 1) {
  for (let i = 0; i < count; i++) {
    output.write(byte);
  }
}

function createReadableFromIterable(iterable) {
  const iterator = iterable[Symbol.iterator]();
  return new ReadableStream({
    start(controller) {
      for (let byte of iterator) {
        controller.enqueue(byte);
      }
      controller.close();
    },
  });
}


let memory = new WebAssembly.Memory({
  initial: 0,
  maximum: 0
});
export async function extern_compile(getChunk) {
  function* sourceGenerator() {
    let next = getChunk();
    while (next !== undefined) {
      yield next;
      next = getChunk();
    }
  }

  try {
    const headers = {
      "Content-Type": "application/wasm"
    };
    const { instance } = await WebAssembly.instantiateStreaming(
      new Response(createReadableFromIterable(sourceGenerator()), { headers }),
      {
        env: {
          memory
        },
        "imports.js": {
          extern_read,
          extern_write,
          runtime_error_tag,
        }
      }
    );
    return () => {
      try {
        instance.exports.run();
      } catch (e) {
        handleRuntimeError(e, "<compiled>");
      }
    };
  } catch (e) {
    switch (e.name) {
      case "TypeError":
        throw 0;
      case "CompileError":
        throw 1;
      case "LinkError":
        throw 2;
      case "RuntimeError":
        throw 3;
      default:
        throw -1;
    }
  }
}

export function setMemory(_memory) {
  memory = _memory;
}

export function handleRuntimeError(tag) {
  const path_pointer = tag.getArg(runtime_error_tag, 3);
  const path_length = tag.getArg(runtime_error_tag, 4);
  const path = new TextDecoder().decode(new Uint8Array(memory.buffer, path_pointer, path_length));
  switch (tag.getArg(runtime_error_tag, 0)) {
    case 0: // Underflow
      console.error(`Runtime error: Underflow at ${path}:${tag.getArg(runtime_error_tag, 1)}:${tag.getArg(runtime_error_tag, 2)}`);
      break;
    case 1: // Overflow
      console.error(`Runtime error: Overflow at ${path}:${tag.getArg(runtime_error_tag, 1)}:${tag.getArg(runtime_error_tag, 2)}`);
      break;
    default:
      console.error("Unknown runtime error");
  }
}