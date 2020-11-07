import {Cell, Action, Game} from "ursa-major";
import { memory } from "ursa-major/ursa_major_bg";

const CANVAS_SIZE = 750;
const GRID_COLS = 80;
const GRID_ROWS = 50;
const CELL_SIZE = CANVAS_SIZE/GRID_ROWS;
const TILE_SIZE = 39;

// Init canvas
const canvas = document.getElementById("ursa-canvas");
canvas.height = CELL_SIZE * GRID_ROWS;
canvas.width = CELL_SIZE * GRID_COLS;
const ctx = canvas.getContext('2d');
// API to the WASM
let game = null;

// Load tiles bitmap
let tiles_image = new Image();
tiles_image.src = 'assets/tiles.png';

function newMap() {
    var seed = Date.now();
    game = Game.new(GRID_COLS, GRID_ROWS, seed);
    requestAnimationFrame(renderLoop);
}

const renderLoop = () => {
    if (game != null) {
        game.tick();
    }
    drawCells();
    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * GRID_COLS + column;
};

const is_inner_wall = (tiles, col, row) => {
    for (let c = Math.max(col - 1, 0); c < Math.min(col + 2, GRID_COLS); c++) {
        for (let r = Math.max(row - 1, 0); r < Math.min(row + 2, GRID_ROWS); r++) {
            if ((c != col || r != row) && tiles[getIndex(r, c)] == Cell.Floor) {
                return false;
            }
        }
    }

    return true;
}

const draw_tile = (ctx, row, col, tile_type) => {
    var tile_x = 0;
    var tile_y = 0;
    if (tile_type == "floor") {
        tile_x = 3;
        tile_y = 2;
    } else if (tile_type == "wall") {
        tile_x = 0;
        tile_y = 3;
    } else if (tile_type == "player") {
        tile_x = 0;
        tile_y = 8;
    } else if (tile_type == "exit") {
        tile_x = 10;
        tile_y = 1;
    } else {
        tile_x = 18;
        tile_y = 0;
    }

    ctx.drawImage(
        tiles_image,
        tile_x * TILE_SIZE + 3,
        tile_y * TILE_SIZE + 3,
        TILE_SIZE - 3,
        TILE_SIZE - 3,
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE);

}

const drawCells = () => {
    const tilesPtr = game.tiles();
    const tiles = new Uint8Array(memory.buffer, tilesPtr, GRID_COLS * GRID_ROWS);

    // tiles
    for (let row = 0; row < GRID_ROWS; row++) {
        for (let col = 0; col < GRID_COLS; col++) {
            const idx = getIndex(row, col);
            if (tiles[idx] == Cell.Floor) {
                draw_tile(ctx, row, col, "floor");
            } else if (is_inner_wall(tiles, col, row)){
                draw_tile(ctx, row, col, "inner-wall");
            } else {
                draw_tile(ctx, row, col, "wall");
            }
        }
    }

    // Player position
    let player = game.player_pos();
    draw_tile(ctx, player.row(), player.col(), "player");

    // Exit position
    let exit = game.exit_pos();
    draw_tile(ctx, exit.row(), exit.col(), "exit");
};

window.addEventListener('keydown', onkeydown, true);

function onkeydown(event) { 
    if (event.code == "ArrowUp") {
        game.execute_action(Action.MoveUp)
        return event.preventDefault();
    } else if (event.code == "ArrowDown") {
        game.execute_action(Action.MoveDown)
        return event.preventDefault();
    } else if (event.code == "ArrowLeft") {
        game.execute_action(Action.MoveLeft)
        return event.preventDefault();
    } else if (event.code == "ArrowRight") {
        game.execute_action(Action.MoveRight)
        return event.preventDefault();
    }
    // console.log(event.code);
};

newMap();
