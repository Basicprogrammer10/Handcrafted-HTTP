const console = document.querySelector("#console");

function writeConsole(text) {
  return new Promise((resolve, _reject) => {
    console.innerHTML = `${console.innerHTML.slice(
      0,
      console.innerHTML.length - 28
    )}${text.replace("\n", "<br>")}<span class="blink">█</span>`;
    resolve();
  });
}

// ⌠
// ⌡
