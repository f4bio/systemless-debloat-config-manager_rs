import "bootstrap";
import "@fortawesome/fontawesome-free";
import "@fontsource/fira-mono";

import "./assets/placeholder-loading.svg";
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
  // const doUpdate = () => {
  // const rust = import("../pkg");
  // rust.then((m) => m.run()).catch(console.error);

  // const selectFile = document.getElementById("selectFile");
  // const startButton = document.getElementById("startButton");
  const rust = import("../pkg");
  const selectFile = document.getElementById("selectFile");

  selectFile.addEventListener("change", (event) => {
    const selectedFileList = event.target["files"];
    // console.log("selectedFileList:", selectedFileList);

    const reader = new FileReader();
    reader.readAsText(selectedFileList.item(0));

    reader.onload = function () {
      // console.log("file content:", reader.result);
      rust.then((m) => m.parse(reader.result.toString())).catch(console.error);
    };

    reader.onerror = function () {
      console.error("error reading file contents:", reader.error);
    };
  });

  // eslint-disable-next-line no-unused-vars
  // const setClipboard = (value) => {
  //   navigator.clipboard.writeText(value).then(
  //     () => {
  //       /* clipboard successfully set */
  //       alert("clipboard successfully set:" + value);
  //     },
  //     () => {
  //       /* clipboard write failed */
  //       alert("clipboard write failed :" + value);
  //     }
  //   );
  // };
  //
  // doUpdate();
};

if (document.readyState !== "loading") {
  // console.log("document is already ready, just execute code here");
  doInit();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    // console.log("document was not ready, place code here");
    doInit();
  });
}
