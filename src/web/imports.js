export function extern_read() {
  const input = prompt("Input a character");
  return input.charCodeAt(0);
}

export function extern_write(byte) {
  console.log(String.fromCharCode(byte));
}

function createReadableFromIterable(iterable) {
  const iterator = Symbol.iterator in iterable ? iterable[Symbol.iterator]() : iterable[Symbol.asyncIterator]();
  return new ReadableStream({
    async pull(controller) {
      const { value, done } = await iterator.next();
      if (done) {
        controller.close();
      }
      else {
        controller.enqueue(value);
      }
    },
  });
}


let memory = new WebAssembly.Memory({
  initial: 0,
  maximum: 0
});
export async function extern_compile(getByte) {
  function* sourceGenerator() {
    let next = getByte();
    while (next !== undefined) {
      yield next;
      next = getByte();
    }
  }

  try {
    const { instance } = await WebAssembly.instantiateStreaming(
      new Response(createReadableFromIterable(sourceGenerator)),
      {
        env: {
          memory
        }
      }
    );
    return instance;
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