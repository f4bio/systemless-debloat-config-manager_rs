import "./style.scss";
import hljs from "highlight.js/lib/core";
import { SignatureTemplate } from "esig";
import wasmLogo from "./assets/webassembly-icon.svg";
import rustLogo from "./assets/rust-lang-icon.png";

document.addEventListener("DOMContentLoaded", async () => {
  hljs.registerLanguage("html", require("highlight.js/lib/languages/xml"));

  document.querySelector("#rustLangIcon").src = rustLogo;
  document.querySelector("#webassemblyIcon").src = wasmLogo;

  const signatureCode = document.querySelector("#signatureCode");
  const signatureWysiwyg = document.querySelector("#signatureWysiwyg");
  const signatureTemplate = SignatureTemplate.new();

  // eslint-disable-next-line no-unused-vars
  const doUpdate = () => {
    const name = document.getElementById("name").value;
    const position = document.getElementById("position").value;
    const phone = document.getElementById("phone").value;
    const email = document.getElementById("email").value;
    const website = document.getElementById("website").value;

    const result = signatureTemplate.interpolate(
      name,
      position,
      phone,
      email,
      website
    );

    signatureCode.querySelector("code").textContent = result;
    signatureWysiwyg.innerHTML = result;
    hljs["highlightElement"](signatureCode.querySelector("code"));
  };

  // eslint-disable-next-line no-unused-vars
  const setClipboard = (value) => {
    const tempInput = document.createElement("input");
    tempInput.style.position = "absolute";
    tempInput.style.left = "-1000px";
    tempInput.style.top = "-1000px";
    tempInput.value = value;
    document.body.appendChild(tempInput);
    tempInput.select();
    document.execCommand("copy");
    document.body.removeChild(tempInput);
  };

  document.querySelector("#copy2clipboard").addEventListener("click", () => {
    setClipboard(signatureCode.querySelector("code").textContent);
  });

  document.querySelector("#code").addEventListener("click", () => {
    signatureWysiwyg.classList.add("hidden");
    signatureCode.classList.remove("hidden");
  });

  document.querySelector("#wysiwyg").addEventListener("click", () => {
    signatureWysiwyg.classList.remove("hidden");
    signatureCode.classList.add("hidden");
  });

  document.querySelectorAll(".default-text-input").forEach((inputElement) => {
    inputElement.addEventListener("input", () => {
      doUpdate();
    });
  });

  signatureCode.classList.remove("hidden");
  doUpdate();

});
