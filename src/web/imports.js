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

export const output = new Output();
export function extern_write(byte) {
  output.write(byte);
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
        }
      }
    );
    const { run } = instance.exports;
    return () => run(); // TODO: just return run
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