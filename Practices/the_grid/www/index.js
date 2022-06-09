import init, {GameGrid,GridSize,get_random_color} from "the_grid";

init().then(_=>{
    const CELL_SIZE = 32;
    const grid = GameGrid.new();
    console.log(grid.size.rows + ' X ' + grid.size.columns);

    const max_width = grid.get_max_len();
    const canvas = document.getElementById("game-canvas");
    canvas.height = grid.size.rows * CELL_SIZE;
    canvas.width = grid.size.columns * CELL_SIZE;

    const line_color=get_random_color();

    const colorInfo = document.getElementById("color-info");
    colorInfo.textContent="Line Color Name = " + line_color.name + ". Code = " + line_color.code;

    const canvas_context=canvas.getContext("2d");
    canvas_context.strokeStyle = line_color.code;

    function drawGrid(){
        canvas_context.beginPath();

        for (let x = 0; x < max_width+ 1; x++){
            canvas_context.moveTo(CELL_SIZE * x, 0);
            canvas_context.lineTo(CELL_SIZE * x, max_width * CELL_SIZE);
        }

        for (let y = 0; y < max_width + 1; y++){
            canvas_context.moveTo(0, CELL_SIZE * y);
            canvas_context.lineTo(max_width * CELL_SIZE,CELL_SIZE * y);
        }

        canvas_context.stroke();
    }

    drawGrid();
})