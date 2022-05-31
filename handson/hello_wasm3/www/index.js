async function run() {

    // 64 Kb'lık (one page) bir bellek sayfası tanımlanır
    // bunu yine logObject içerisinde tanımlayıp WASM tarafında ele alınmasını sağlayabiliriz.
    const js_memory = new WebAssembly.Memory({initial:1});

    // Aşağıdaki nesne WASM tarafında kullanılabilir
    const jsObject = {
        js: {
            mem : js_memory
        },
        console: {
            log: (param)=> {
                console.log(param+" için alan hesaplanacak.");
            }
        }
    }

    const response = await fetch("calc.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer,jsObject);

    const calcFunc = wasm.instance.exports.calc;
    const result = calcFunc(2);
    console.log(result);

//    // WASM İçerisinde tanımlanmış olan bellek sayfasına erişim sağlanıyor
//    const memoryFromWasm = wasm.instance.exports.mem;
//    // WASM İçinden gelen bellek sayfasının ilk 32 byte'ını okumak için bir array tanımlanır
//    const message_buffer = new Uint8Array(memoryFromWasm.buffer,0,32);
//    // sonrasında TextDecoder'tan yararlanılarak byte dizisi decode edilir(okunur hale getirilir)
//    const message = new TextDecoder().decode(message_buffer);

    // JS tarafında tanımlı bellek bölgesini yukarıda tanımlamıştık
    const message_buffer = new Uint8Array(js_memory.buffer,0,32);
    const message = new TextDecoder().decode(message_buffer);
    console.log(message);
}

run();