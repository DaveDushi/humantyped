// inject.js — Runs in MAIN world (page context) via manifest.
// Monitors CreateTweet requests. Text insertion is handled
// by content.js via paste simulation.
(function () {
  "use strict";

  const originalFetch = window.fetch;

  window.fetch = function (...args) {
    return originalFetch.apply(this, args);
  };
})();
