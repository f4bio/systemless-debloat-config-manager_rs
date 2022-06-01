import "./style.css";
import "@fortawesome/fontawesome-free";
// import { SignatureTemplate } from "systemless-debloat-config-manager";
import wasmLogo from "./assets/webassembly-icon.svg";
import rustLogo from "./assets/rust-lang-icon.png";
import bootstrapLogo from "./assets/bootstrap-icon.svg";

const doInit = () => {
  document.querySelector("#rustLangIcon").src = rustLogo;
  document.querySelector("#webassemblyIcon").src = wasmLogo;
  document.querySelector("#bootstrapIcon").src = bootstrapLogo;

  // const signatureCode = document.querySelector("#signatureCode");
  // const signatureWysiwyg = document.querySelector("#signatureWysiwyg");
  // const signatureTemplate = SignatureTemplate.new();

  // eslint-disable-next-line no-unused-vars
  const doUpdate = () => {
    // const name = document.getElementById("name").value;
    // const position = document.getElementById("position").value;
    // const phone = document.getElementById("phone").value;
    // const email = document.getElementById("email").value;
    // const website = document.getElementById("website").value;

    // const result = signatureTemplate.interpolate(
    //   name,
    //   position,
    //   phone,
    //   email,
    //   website
    // );

    // signatureCode.querySelector("code").textContent = result;
    // signatureWysiwyg.innerHTML = result;
  };

  // eslint-disable-next-line no-unused-vars
  const setClipboard = (value) => {
    navigator.clipboard.writeText(value).then(
      () => {
        /* clipboard successfully set */
        alert("clipboard successfully set:", value);
      },
      () => {
        /* clipboard write failed */
        alert("clipboard write failed :", value);
      }
    );
  };

  // document.querySelector("#copy2clipboard").addEventListener("click", () => {
  //   setClipboard(signatureCode.querySelector("code").textContent);
  // });

  // document.querySelector("#code").addEventListener("click", () => {
  //   signatureWysiwyg.classList.add("hidden");
  //   signatureCode.classList.remove("hidden");
  // });

  // document.querySelector("#wysiwyg").addEventListener("click", () => {
  //   signatureWysiwyg.classList.remove("hidden");
  //   signatureCode.classList.add("hidden");
  // });

  // document.querySelectorAll(".default-text-input").forEach((inputElement) => {
  //   inputElement.addEventListener("input", () => {
  //     doUpdate();
  //   });
  // });

  // signatureCode.classList.remove("hidden");
  doUpdate();
};

if (document.readyState !== "loading") {
  console.log("document is already ready, just execute code here");
  doInit();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    console.log("document was not ready, place code here");
    doInit();
  });
}
