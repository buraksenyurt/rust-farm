import init, {print_temperature} from "hello_wasm3";

init().then(_ => {
    var info = print_temperature("istanbul");
    console.log(info);
})