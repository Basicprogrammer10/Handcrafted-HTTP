const console = document.querySelector("#console");

document.querySelector("#check").addEventListener("click", () => {
  let toSend = document.querySelectorAll("#to > .drag");
  let out = "";

  toSend.forEach((i) => (out += i.innerHTML + ","));
  out = out.substr(0, out.length - 1);

  fetch(`/check/${document.location.toString().split('/')[4]}`, {
    method: "POST",
    body: out,
  })
    .then((res) => res.text())
    .then((res) => {
      if (res === "CORRECT") {
        document.querySelector("#console").style.filter = "blur(2px)"
        document.querySelector("#readme").style.filter = "blur(2px)"
        document.querySelector("#next").style.opacity = 1
      }
      window.console.log(res);
    });
});

document.querySelector('#nextButton').addEventListener('click', () => window.location = `/next/${document.location.toString().split('/')[4]}`);

function writeConsole(text) {
  return new Promise((resolve, _reject) => {
    console.innerHTML = `${console.innerHTML.slice(
      0,
      console.innerHTML.length - 27
    )}${text.replace("\n", "<br>")}<span class="blink"></span>`;
    resolve();
  });
}

dragula([document.querySelector("#from"), document.querySelector("#to")], {
  direction: "horizontal",
});
