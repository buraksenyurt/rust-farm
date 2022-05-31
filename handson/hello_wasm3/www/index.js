import init, {print_temperature} from "hello_wasm3";

init().then(_ => {
    var info = print_temperature("istanbul");
    console.log(info);
})

// Alttaki kullanım yerine genellikle üstteki kullanım da tercih edilebilir.
//async function start(){
//    const wasm = await init();
//    var info = print_temperature("istanbul");
//    console.log(info);
//}
//
//start();