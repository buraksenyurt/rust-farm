async function run() {

  const response = await fetch("calc.wasm");
  const buffer = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer);

  const calcFunc = wasm.instance.exports.sum;
  const result = calcFunc(2);
  console.log(result);
}

run();