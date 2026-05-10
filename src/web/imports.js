export function extern_read() {
  const input = prompt("Input a character");
  return input.charCodeAt(0);
}

export function extern_write(byte) {
  console.log(String.fromCharCode(byte));
}