import init, { World } from "snake_game";

init().then(_ => {
    const world = World.new();
    const world_width = world.get_width();
    const CELL_SIZE = 20; // Oyun sahasındaki hücrelerin boyutu
    console.log("Oyun nesnesi oluşturuldu. Uzunluk " + world.get_width() + " birim.");

    // üzerinde çizim yapacağımız canvas elementine ulaşmalıyız
    const canvas = document.getElementById("game-canvas");

    // sahanın genişlik ve yüksekliğini Rust tarafındaki World nesnesinden gelen değere göre belirliyoruz
    canvas.height = world_width * CELL_SIZE;
    canvas.width = world_width * CELL_SIZE;

    // 2 boyutlu çizim yapmamızı sağlayacak context nesnesini alıyoruz
    const canvas_context=canvas.getContext("2d");

    // Oyun sahasını çizen fonksiyon
    function drawGameWorld(){
        canvas_context.beginPath();

        // hücre çeperi genişliğine göre X eksenindeki çizgiler oluşturulur
        // Yani dikey eksendeki çizgiler
        for (let x = 0; x < world_width + 1; x++){
            canvas_context.moveTo(CELL_SIZE * x, 0);
            canvas_context.lineTo(CELL_SIZE * x, world_width * CELL_SIZE);
        }

        // hücre çeğeri genişliğine göre Y eksenini dolduruyoruz
        // Yani yatay çizgiler çizdirilir
        for (let y = 0; y < world_width + 1; y++){
            canvas_context.moveTo(0, CELL_SIZE * y);
            canvas_context.lineTo(world_width * CELL_SIZE,CELL_SIZE * y);
        }

        canvas_context.stroke();
    }

    drawGameWorld();
})