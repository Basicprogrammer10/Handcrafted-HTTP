const console = document.querySelector("#console");
const correctEmoji = ['✨', '🎉', '✨', '🚀', '🎈', '👏'];

document.querySelector("#check").addEventListener("click", () => {
  let toSend = document.querySelectorAll("#to > .drag");
  let out = "";

  toSend.forEach((i) => (out += i.innerHTML + ","));
  out = out.substr(0, out.length - 1);

  fetch(`/check/${document.location.toString().split("/")[4]}`, {
    method: "POST",
    body: out,
  })
    .then((res) => res.text())
    .then((res) => {
      if (res === "CORRECT") {
        let e = correctEmoji[Math.floor(Math.random() * correctEmoji.length)];
        document.querySelector("#console").style.filter = "blur(5px)";
        document.querySelector("#readme").style.filter = "blur(5px)";
        document.querySelector("#next").style.opacity = 1;
        document.querySelector("#next").style.zIndex = 10;
        document.querySelector(
          "#next > h1:nth-child(1)"
        ).innerHTML = `${e} Correct ${e}`;
      }

      if (res === "WRONG") {
        document.querySelector("#console").style.filter = "blur(5px)";
        document.querySelector("#readme").style.filter = "blur(5px)";
        document.querySelector("#wrong").style.opacity = 1;
        document.querySelector("#wrong").style.zIndex = 10;
      }
    });
});

document
  .querySelector("#nextButton")
  .addEventListener(
    "click",
    () =>
      (window.location = `/next/${document.location.toString().split("/")[4]}`)
  );

document.querySelector("#wrongButton").addEventListener("click", () => {
  document.querySelector("#console").style.filter = "blur(0)";
  document.querySelector("#readme").style.filter = "blur(0)";
  document.querySelector("#wrong").style.opacity = 0;
  document.querySelector("#wrong").style.zIndex = 0;
});

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
