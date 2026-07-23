const WIDTH = 12;
const HEIGHT = 12;

const board = document.getElementById("game_board");

for (let y = 0; y < HEIGHT; y++) {
    for (let x = 0; x < WIDTH; x++) {
        const cell = document.createElement("button");

        cell.className = "cell";

        cell.dataset.x = x;
        cell.dataset.y = y;

        board.appendChild(cell);
    }
}