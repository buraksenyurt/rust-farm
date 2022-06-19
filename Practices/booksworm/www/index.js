import init,{Book,add_book} from "booksworm";

init().then(_ =>{
    const submitButton = document.getElementById("buttonAdd");

    submitButton.addEventListener("click",_=>{
        console.log("Ekle düğmesine basıldı");

        var title=document.getElementById("inputTitle").value;
        var authors=document.getElementById("inputAuthors").value;
        var isRead=document.getElementById("checkIsRead").checked;

        console.log(title);
        console.log(authors);
        console.log(isRead);

        var result=add_book(title,authors,isRead);
        console.log(result);
    });
});