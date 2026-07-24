document
  .querySelector("#main_board")
  .addEventListener("pointerdown", onPointerDown);

function onPointerDown(event) {
  if (!(event.target instanceof Element)) {
    return;
  }

  const button = event.target.closest("[data-cell]");

  if (!(button instanceof HTMLButtonElement)) {
    return;
  }

  const x = Number(button.dataset.x);
  const y = Number(button.dataset.y);

  console.log(x, y);
}
