// Rust tarafındaki servis ile iletişim kuracak fonksiyonaliteyi içeren kısım
type WebMethod = "GET" | "POST" | "DELETE" | "PATCH";
const API_BASE_PATH = '/api';

// HTTP Get,Post,Put ve Delete talepleri için modülden dışarıya açılan fonksiyonlar.
// Her biri restCall fonksiyonunu çağırmakta
export async function get(path:string,data?:any){
    return restCall("GET",path,data);
}

export async function post(path:string,data?:any){
    return restCall("POST",path,data);
}

export async function patch(path:string,data?:any){
    return restCall("PATCH",path,data);
}

export async function del(path:string,data?:any){
    return restCall("DELETE",path,data);
}

async function restCall(httpMethod:WebMethod,path:string,data?:any){
    // Tipik olarak bir HTTP talebi gönderiyoruz
    // Bunun için fetch fonksiyonu kullanılmakta.
    // Fonksiyona gerekli header bilgileri dışında body kısmı da aktarılıyor(JSON formatında)

    const url = `${API_BASE_PATH}/${path}`;

    const response = await fetch(url,{
        method: httpMethod,
        mode: 'same-origin',
        cache:'no-cache',
        headers: {
            'Content-Type':'application/json',
            'X-Auth-Token':'10101' // Normalde bu bilgi bir user identity sistem üstünden gelmeli tabii.
        },
        body: JSON.stringify(data)
    });

    // Çağrıdan gelen veriyi geriye dönüyoruz
    let result= await response.json();
    return result.data;

}
