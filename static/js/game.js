const config = {
  width: 12,
  height: 12,
};

const board = document.getElementById("game_board");
board.style.setProperty("--grid-columns", config.width);
board.style.setProperty("--grid-rows", config.height);

for (let y = 0; y < config.height; y++) {
  for (let x = 0; x < config.width; x++) {
    const cell = document.createElement("button");

    cell.className = "cell";

    cell.dataset.x = x;
    cell.dataset.y = y;

    board.appendChild(cell);
  }
}
