import init, { World, Direction } from "snake_game";

init().then(_ => {
    const CELL_SIZE = 20; // Oyun sahasındaki hücrelerin boyutu
    const WORLD_WIDTH = 8; // Sütun sayısı

    // Yılanın başlayacağı indeks değerini hesaplarken o anki zaman bilgisinin
    // sahadaki hücre sayısına bölümünden kalan değeri hesaba katıyoruz
    const SNAKE_START_INDEX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

    // Oyun nesnesi js tarafından gelen parametrelere göre oluşturulur.
    const world = World.new(WORLD_WIDTH,SNAKE_START_INDEX);
    const world_width = world.get_width();

    console.log("Oyun nesnesi oluşturuldu. Uzunluk " + world.get_width() + " birim.");

    // üzerinde çizim yapacağımız canvas elementine ulaşmalıyız
    const canvas = document.getElementById("game-canvas");

    // sahanın genişlik ve yüksekliğini Rust tarafındaki World nesnesinden gelen değere göre belirliyoruz
    canvas.height = world_width * CELL_SIZE;
    canvas.width = world_width * CELL_SIZE;

    document.addEventListener("keydown", (event)=>{
        switch(event.code){
            case "ArrowLeft":
                console.log("Sol tuşa basıldı");
                world.change_direction(Direction.Left);
                break;
            case "ArrowRight":
                console.log("Sağ tuşa basıldı");
                world.change_direction(Direction.Right);
                break;
            case "ArrowDown":
                console.log("Aşağı tuşa basıldı");
                world.change_direction(Direction.Down);
                break;
            case "ArrowUp":
                console.log("Yukarı tuşa basıldı");
                world.change_direction(Direction.Up);
                break;
        }
    })

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

    // Yılanı sahaya çizmek için kullanılan fonksiyon
    function drawSnake(){
        // Rust kütüphanesinden yararlanarak yılanın başlangıç indeksini alalım
        const index = world.snake_head();
        // başlangıç konumuna göre kolon ve sütun konumlarını bulmamız lazım
        // oyun sahasının genişliği 8 birim olduğundan mod alarak kolonu bulabiliriz
        // yani 8e bölümden kalan hangi kolon olduğunu söyler
        const column = index % world_width;
        // oyun sahasına bölümdeki tam kısım satrı ifade eder
        const row = Math.floor(index / world_width);

        // çizime başlanır
        canvas_context.beginPath();
        // yılan nesnesinin sahip olduğu hücreler birer dörtgendir
        // İlk iki değerle x,y başlangıç koordinatları verilir
        // sondaki iki değer ise genişlik ve yükseklik için kullanılır
        canvas_context.fillRect(
            column * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );
        canvas_context.stroke();
    }

    function render(){
        drawGameWorld();
        drawSnake();
    }

    function tick(){
//        // her 100 saniyede bir çalışacak olan oyun döngüsü
//        setInterval(()=>{
//            // canvas alanı temizlenir
//            canvas_context.clearRect(0,0,canvas.width,canvas.height);
//            // yılan için güncel hücre pozisyonu alınır
//            world.update_position();
//            // oyun alanı ve yılan yeniden çizilir
//            render();
//        },100);

        const fps = 3;
        setTimeout(()=>{
            canvas_context.clearRect(0,0,canvas.width,canvas.height);
            world.update_position();
            render();
            requestAnimationFrame(tick);
        },1000 / fps);
    }

    render();
    tick();
})