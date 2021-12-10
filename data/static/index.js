const console = document.querySelector("#console");

document.querySelector("#check").addEventListener("click", () => {
  let toSend = document.querySelectorAll("#to > .drag");
  let out = "";

  toSend.forEach((i) => (out += i.innerHTML + ","));
  out = out.substr(0, out.length - 1);

  if (out === "GET,/,HTTP/1.1") {
    document.querySelector("#console").style.filter = "blur(5px)";
    document.querySelector("#readme").style.filter = "blur(5px)";
    document.querySelector("#next").style.opacity = 1;
  }
});

document
  .querySelector("#nextButton")
  .addEventListener("click", () => (window.location = "/level/01"));

dragula([document.querySelector("#from"), document.querySelector("#to")], {
  direction: "horizontal",
});
