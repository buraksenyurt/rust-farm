import init,{Book} from "booksworm";

init().then(_ =>{
    console.log("It's working...");

    const submitButton = document.getElementById("buttonAdd");

    submitButton.addEventListener("click",_=>{
        console.log("Ekle düğmesine basıldı");
    });
});