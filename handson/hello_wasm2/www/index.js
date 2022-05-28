async function run() {
    // Aşağıdaki nesne WASM tarafında kullanılabilir
    const logObject = {
        console: {
            log: (param)=> {
                console.log(param+" için alan hesaplanacak.");
            }
        }
    }

    const response = await fetch("calc.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer,logObject);

    const calcFunc = wasm.instance.exports.calc;
    const result = calcFunc(2);
    console.log(result);

    // WASM İçerisinde tanımlanmış olan bellek sayfasına erişim sağlanıyor
    const memoryFromWasm = wasm.instance.exports.mem;
    // WASM İçinden gelen bellek sayfasının ilk 32 byte'ını okumak için bir array tanımlanır
    const message_buffer = new Uint8Array(memoryFromWasm.buffer,0,32);
    // sonrasında TextDecoder'tan yararlanılarak byte dizisi decode edilir(okunur hale getirilir)
    const message = new TextDecoder().decode(message_buffer);
    console.log(message);
}

run();