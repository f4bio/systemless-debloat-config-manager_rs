import "bootstrap";
import "@fortawesome/fontawesome-free";
import "@fontsource/fira-mono";

import wasmLogo from "./assets/webassembly-icon.svg";
import rustLogo from "./assets/rust-lang-icon.png";
import bootstrapLogo from "./assets/bootstrap-icon.svg";

const doInit = () => {
  document.getElementById("rustLangIcon").src = rustLogo;
  document.getElementById("webassemblyIcon").src = wasmLogo;
  document.getElementById("bootstrapIcon").src = bootstrapLogo;

  setTimeout(() => {
    document.getElementById("loading-container")
      .toggleAttribute("hidden");
    document.getElementById("input-container")
      .toggleAttribute("hidden");
  }, 1000);

  document.getElementById("selectFile")
    .addEventListener("change", (event) => {
      const selectedFileList = event.target["files"];
      // console.log("selectedFileList:", selectedFileList);

      const reader = new FileReader();
      reader.readAsText(selectedFileList.item(0));

      reader.onload = function() {
        // console.log("file content:", reader.result);
        const inputText = reader.result.toString();
        const splitted = inputText.split(/System packages:/);
        const systemApps = splitted[1].split("\n");
        // const systemPackages = splitted[0].split("System apps, not debloated:")[1].split("\n");

        const resultList = document.getElementById("result-list");

        systemApps.forEach((item) => {
          if (item.startsWith("/system/app/")) {
            const packageSplit = item.split("=");
            const appName = packageSplit[0]
              .replace(/\/system\/app\/.*\//, "")
              .replace(".apk", "");
            const appId = packageSplit[1];

            console.log("appName:", appName);
            console.log("appId:", appId);

            const liElement = document.createElement("li");
            liElement.className = "list-group-item d-flex justify-content-between align-items-start";
            liElement.innerHTML = "<input class=\"form-check-input me-1\" type=\"checkbox\" value=\"\" aria-label=\"...\">"
              + "<div class=\"ms-2 me-auto text-start\">"
              + "<div class=\"fw-bold\">" + appName + "</div>"
              + "<span class=\"\">" + appId + "</span>"
              + "</div>"
              + "<span class=\"badge bg-primary rounded-pill\">14</span>";
            resultList.appendChild(liElement);
          }
        });

        // systemPackages.forEach((item) => {
        //   if(item.startsWith("/system/app/")) {
        //     const name = item.split("/system/app/.*/")[1];
        //
        //     const liElement = document.createElement("li");
        //     liElement.className = "list-group-item d-flex justify-content-between align-items-start";
        //     liElement.innerHTML = "<div class=\"ms-2 me-auto\">" +
        //       "<div class=\"fw-bold\">Subheading</div>"
        //       + name +
        //       "</div><span class=\"badge bg-primary rounded-pill\">14</span>";
        //     resultList.appendChild(liElement);
        //   }
        // });

        document.getElementById("input-container")
          .toggleAttribute("hidden");
        document.getElementById("result-container")
          .toggleAttribute("hidden");

        // parse(reader.result.toString());
      };

      reader.onerror = function() {
        console.error("error reading file contents:", reader.error);
      };
    });
};

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    navigator.serviceWorker.register("/service-worker.js")
      .then((registration) => {
        console.debug("SW registered: ", registration);
      }).catch((registrationError) => {
      console.error("SW registration failed: ", registrationError);
    });
  });
}

if (document.readyState !== "loading") {
  // console.log("document is already ready, just execute code here");
  doInit();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    // console.log("document was not ready, place code here");
    doInit();
  });
}
