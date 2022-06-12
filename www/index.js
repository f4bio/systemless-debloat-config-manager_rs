import "bootstrap";
import "@fortawesome/fontawesome-free";
import "@fontsource/fira-mono";

import "./assets/placeholder-loading.svg";
import wasmLogo from "./assets/webassembly-icon.svg";
import rustLogo from "./assets/rust-lang-icon.png";
import bootstrapLogo from "./assets/bootstrap-icon.svg";

import { parse } from "../pkg";

const doInit = () => {
  document.querySelector("#rustLangIcon").src = rustLogo;
  document.querySelector("#webassemblyIcon").src = wasmLogo;
  document.querySelector("#bootstrapIcon").src = bootstrapLogo;

  if ("serviceWorker" in navigator) {
    window.addEventListener("load", () => {
      navigator.serviceWorker.register("/service-worker.js").then((registration) => {
        console.debug("SW registered: ", registration);
      }).catch(registrationError => {
        console.error("SW registration failed: ", registrationError);
      });
    });
  }
  const selectFile = document.getElementById("selectFile");
  selectFile.addEventListener("change", (event) => {
    const selectedFileList = event.target["files"];
    // console.log("selectedFileList:", selectedFileList);

    const reader = new FileReader();
    reader.readAsText(selectedFileList.item(0));

    reader.onload = function() {
      // console.log("file content:", reader.result);
      parse(reader.result.toString());
    };

    reader.onerror = function() {
      console.error("error reading file contents:", reader.error);
    };
  });
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
