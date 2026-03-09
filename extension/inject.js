// inject.js — Runs in MAIN world (page context) via manifest.
// Logs CreateTweet requests for debugging. Text insertion is handled
// by content.js via paste simulation (the reliable approach).
(function () {
  "use strict";

  const LOG = "[HumanTyped:inject]";
  const originalFetch = window.fetch;

  window.fetch = function (...args) {
    try {
      let url = "";
      if (typeof args[0] === "string") {
        url = args[0];
      } else if (args[0] && args[0].url) {
        url = args[0].url;
      }

      if (url.includes("/graphql/") && url.includes("CreateTweet")) {
        console.log(LOG, "CreateTweet request detected");
      }
    } catch (_) {}

    return originalFetch.apply(this, args);
  };

  console.log(LOG, "Fetch monitor active");
})();
