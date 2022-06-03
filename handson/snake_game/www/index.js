import init, { World } from "snake_game";

init().then(_ => {
    const world = World.new();
    console.log("Oyun nesnesi oluşturuldu. Uzunluk " + world.width + " birim.");
})