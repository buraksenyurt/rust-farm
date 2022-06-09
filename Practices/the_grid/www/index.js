import init, {GameGrid,GridSize} from "the_grid";

init().then(_=>{
    const CELL_SIZE = 32;
    const ROW_SIZE = 12;
    const COLUMN_SIZE = 8;
    const grid = GameGrid.new(ROW_SIZE,COLUMN_SIZE);

    console.log(grid.size.rows + ' X ' + grid.size.columns);
})