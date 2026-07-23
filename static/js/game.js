async function loadGame() {
  const response = await fetch("/api/state");

  const state = await response.json();

  render(state);
}

function render(state) {
  const board = document.querySelector("#game_board");

  board.style.gridTemplateColumns = `repeat(${state.size}, 1fr)`;

  board.style.gridTemplateRows = `repeat(${state.size}, 1fr)`;

  board.innerHTML = "";

  for (const cell of state.cells) {
    const div = document.createElement("div");

    div.className = "cell";

    div.style.background = cell.color;

    div.onclick = () => {
      div.style.background = div.style.background === "white" ? "red" : "white";
    };
    board.appendChild(div);
  }
}

loadGame();
