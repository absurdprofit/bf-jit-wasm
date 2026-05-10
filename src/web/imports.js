export function extern_read() {
  const input = prompt("Input a character");
  return input.charCodeAt(0);
}

export function extern_write(byte) {
  console.log(String.fromCharCode(byte));
}

let memory = new WebAssembly.Memory({
  initial: 0,
  maximum: 0
});
export function extern_compile() {
  console.log(memory);
}

export function setMemory(_memory) {
  memory = _memory;
}