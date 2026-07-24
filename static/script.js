const board = document
  .querySelector("#main_board");

  board.addEventListener("pointerdown", onPointerDown);

function onPointerDown(event) {
  const button = getCell(event);
  
  if (!button) {
    return;
  }

  button.dataset.visible = "false";

  const x = Number(button.dataset.x);
  const y = Number(button.dataset.y);
  
  console.log(x, y);
}

function getCell(event) {
  if (!(event.target instanceof Element)) {
    return null;
  }
  
  const button = event.target.closest("[data-cell]");
  
  if (!(button instanceof HTMLButtonElement)) {
    return null;
  }

  return button;
}