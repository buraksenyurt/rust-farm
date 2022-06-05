import init, { World } from "snake_game";

init().then(_ => {
    const world = World.new();
    console.log("Oyun nesnesi oluşturuldu. Uzunluk " + world.get_width() + " birim.");

    // canvas elementini alıp 2 boyutlu context nesnesine ulaşıyoruz.
    const canvas = document.getElementById("game-canvas");
    const canvas_context=canvas.getContext("2d");
})