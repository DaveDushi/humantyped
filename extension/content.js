(() => {
  "use strict";

  // ─── Hidden element for communication with inject.js ────────────
  // inject.js runs in MAIN world (page context) via manifest.
  // We share data through a hidden DOM element.

  const dataEl = document.createElement("div");
  dataEl.id = "__humantyped_data";
  dataEl.style.display = "none";
  document.documentElement.appendChild(dataEl);

  // Watch for inject.js confirming it modified the request
  const sentObserver = new MutationObserver((mutations) => {
    for (const m of mutations) {
      if (m.attributeName === "data-sent") {
        const token = dataEl.getAttribute("data-sent");
        if (token) {
          dataEl.removeAttribute("data-sent");
          dataEl.removeAttribute("data-payload");
          resetSession();
        }
      }
    }
  });
  sentObserver.observe(dataEl, { attributes: true });

  // ─── Session State (global, survives React re-renders) ───────────

  const session = {
    start: null,
    lastKeyUp: null,
    keyDownTimes: new Map(),
    flightTimes: [],
    dwellTimes: [],
    totalKeys: 0,
    backspaceCount: 0,
    charCount: 0,
    wpmSamples: [],
    lastSampleTime: null,
    charsAtLastSample: 0,
    activeComposeBox: null,
  };

  function resetSession() {
    session.start = null;
    session.lastKeyUp = null;
    session.keyDownTimes.clear();
    session.flightTimes = [];
    session.dwellTimes = [];
    session.totalKeys = 0;
    session.backspaceCount = 0;
    session.charCount = 0;
    session.wpmSamples = [];
    session.lastSampleTime = null;
    session.charsAtLastSample = 0;
    session.activeComposeBox = null;
    certifying = false;
  }

  // ─── Compose Box Detection ───────────────────────────────────────

  function getComposeBox(target) {
    if (!target || !target.closest) return null;
    if (target.matches && target.matches('[role="textbox"][contenteditable="true"]')) return target;
    const ce = target.closest('[role="textbox"][contenteditable="true"]');
    if (ce) return ce;
    const ce2 = target.closest('[contenteditable="true"]');
    if (ce2 && (ce2.closest('[data-testid*="tweetTextarea"]') || ce2.closest('[data-testid*="TextArea"]'))) return ce2;
    return null;
  }

  function findAnyComposeBox() {
    const selectors = [
      '[role="textbox"][contenteditable="true"]',
      '[data-testid*="tweetTextarea"] [contenteditable="true"]',
      'div[contenteditable="true"][data-contents="true"]',
    ];
    for (const sel of selectors) {
      const el = document.querySelector(sel);
      if (el) return el;
    }
    return null;
  }

  // ─── Paste Blocking (delegated) ──────────────────────────────────

  let allowOwnPaste = false; // bypass flag for our own paste-insertion

  document.addEventListener("paste", (e) => {
    if (allowOwnPaste) return; // let our own paste through
    if (getComposeBox(e.target)) {
      e.preventDefault();
      e.stopPropagation();
      showTooltip("Paste disabled — HumanTyped requires manual typing");
    }
  }, true);

  document.addEventListener("drop", (e) => {
    if (e.dataTransfer && e.dataTransfer.types.includes("text/plain") && getComposeBox(e.target)) {
      e.preventDefault();
      e.stopPropagation();
    }
  }, true);

  document.addEventListener("beforeinput", (e) => {
    if (allowOwnPaste) return; // let our own paste through
    if ((e.inputType === "insertFromPaste" || e.inputType === "insertFromDrop") && getComposeBox(e.target)) {
      e.preventDefault();
      e.stopPropagation();
    }
  }, true);

  // ─── Keystroke Tracking (delegated on document) ──────────────────

  document.addEventListener("keydown", (e) => {
    const composeBox = getComposeBox(e.target);
    if (!composeBox) return;

    if (["Control", "Meta", "Shift", "Alt"].includes(e.key)) return;

    // Block Ctrl+V / Cmd+V
    if ((e.ctrlKey || e.metaKey) && e.key === "v") {
      e.preventDefault();
      showTooltip("Paste disabled — HumanTyped requires manual typing");
      return;
    }

    const now = performance.now();

    if (!session.start) {
      session.start = now;
      session.lastSampleTime = now;
    }

    session.activeComposeBox = composeBox;

    if (!session.keyDownTimes.has(e.key)) {
      session.keyDownTimes.set(e.key, now);
    }

    if (e.key === "Backspace" || e.key === "Delete") {
      session.backspaceCount++;
    }

    session.totalKeys++;
  }, true);

  document.addEventListener("keyup", (e) => {
    const composeBox = getComposeBox(e.target);
    if (!composeBox) return;
    if (["Control", "Meta", "Shift", "Alt"].includes(e.key)) return;

    const now = performance.now();
    const downTime = session.keyDownTimes.get(e.key);

    if (downTime) {
      session.dwellTimes.push(now - downTime);
      session.keyDownTimes.delete(e.key);
    }

    if (session.lastKeyUp) {
      session.flightTimes.push(now - session.lastKeyUp);
    }
    session.lastKeyUp = now;

    if (e.key.length === 1) {
      session.charCount++;
    }

    // WPM sampling every 5s
    if (session.lastSampleTime && now - session.lastSampleTime >= 5000 && session.charCount > 0) {
      const elapsedMin = (now - session.lastSampleTime) / 60000;
      const charsInWindow = session.charCount - session.charsAtLastSample;
      if (elapsedMin > 0 && charsInWindow > 0) {
        session.wpmSamples.push((charsInWindow / 5) / elapsedMin);
      }
      session.lastSampleTime = now;
      session.charsAtLastSample = session.charCount;
    }

    session.activeComposeBox = composeBox;
    sendStats();
  }, true);

  // ─── Metrics ─────────────────────────────────────────────────────

  function getSessionDurationMs() {
    return session.start ? performance.now() - session.start : 0;
  }

  function getWpmAverage() {
    const elapsed = getSessionDurationMs() / 60000;
    return elapsed > 0 ? (session.charCount / 5) / elapsed : 0;
  }

  function getWpmVariance() {
    if (session.wpmSamples.length < 2) return 0;
    const mean = session.wpmSamples.reduce((a, b) => a + b, 0) / session.wpmSamples.length;
    return session.wpmSamples.reduce((s, v) => s + (v - mean) ** 2, 0) / session.wpmSamples.length;
  }

  function getCorrectionRate() {
    return session.totalKeys === 0 ? 0 : session.backspaceCount / session.totalKeys;
  }

  function getFlightTimeStdDev() {
    if (session.flightTimes.length < 2) return 0;
    const mean = session.flightTimes.reduce((a, b) => a + b, 0) / session.flightTimes.length;
    return Math.sqrt(session.flightTimes.reduce((s, t) => s + (t - mean) ** 2, 0) / session.flightTimes.length);
  }

  function getConfidenceScore() {
    let score = 100;

    // Robotic WPM consistency (humans vary naturally)
    if (session.wpmSamples.length >= 2 && getWpmVariance() < 5) score -= 35;

    // Superhuman typing speed
    if (getWpmAverage() > 150) score -= 25;

    // Zero corrections is a huge red flag for any real typing
    if (getCorrectionRate() === 0 && session.charCount > 30) score -= 30;

    // Session too fast for character count
    if (getSessionDurationMs() < session.charCount * 30) score -= 25;

    // Robotic inter-key timing (browser automation adds ~20-50ms random delays)
    if (session.flightTimes.length >= 10 && getFlightTimeStdDev() < 30) score -= 25;

    // Perfectly uniform flight times (coefficient of variation < 15%)
    if (session.flightTimes.length >= 10) {
      const mean = session.flightTimes.reduce((a, b) => a + b, 0) / session.flightTimes.length;
      if (mean > 0 && getFlightTimeStdDev() / mean < 0.15) score -= 20;
    }

    // No WPM samples at all on a long session is suspicious (session tracking issues)
    if (session.wpmSamples.length < 2 && session.charCount > 50 && getSessionDurationMs() > 15000) score -= 15;

    return Math.max(0, Math.min(100, score));
  }

  // ─── Token Generation (synchronous) ──────────────────────────────

  function generateToken() {
    const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const values = crypto.getRandomValues(new Uint8Array(12));
    let token = "";
    for (let i = 0; i < 12; i++) {
      token += chars[values[i] % chars.length];
    }
    return token;
  }

  // ─── Text Insertion via Paste Simulation ─────────────────────────
  // From tweetGPT: Draft.js has a built-in paste handler that properly
  // updates its internal EditorState. Dispatching a synthetic ClipboardEvent
  // with DataTransfer is the most reliable way to insert text.

  function insertViaPaste(element, text) {
    element.focus();

    // Move cursor to end
    const sel = window.getSelection();
    if (sel && element.childNodes.length > 0) {
      const range = document.createRange();
      range.selectNodeContents(element);
      range.collapse(false);
      sel.removeAllRanges();
      sel.addRange(range);
    }

    const dt = new DataTransfer();
    dt.setData("text/plain", text);

    // Temporarily allow our own paste through the blocker
    allowOwnPaste = true;

    element.dispatchEvent(
      new ClipboardEvent("paste", {
        clipboardData: dt,
        bubbles: true,
        cancelable: true,
      })
    );

    allowOwnPaste = false;
    dt.clearData();
  }

  // ─── Post Button Interception ────────────────────────────────────
  //
  // Dual strategy:
  // 1. PRIMARY: inject.js intercepts CreateTweet fetch and modifies body
  // 2. FALLBACK: Paste-simulate the token into compose box (tweetGPT approach)
  //
  // We do BOTH: store token for inject.js AND paste into compose box.
  // Belt and suspenders — at least one will work.

  let certifying = false;

  document.addEventListener("click", (e) => {
    const button =
      e.target.closest('[data-testid="tweetButton"]') ||
      e.target.closest('[data-testid="tweetButtonInline"]');

    if (!button) return;
    if (certifying) return;

    if (session.charCount < 5) {
      return;
    }

    // STOP the click from reaching X's handler
    e.preventDefault();
    e.stopImmediatePropagation();

    certifying = true;

    const token = generateToken();

    const metrics = {
      token,
      wpm_average: Math.round(getWpmAverage() * 100) / 100,
      wpm_variance: Math.round(getWpmVariance() * 100) / 100,
      correction_rate: Math.round(getCorrectionRate() * 1000) / 1000,
      session_duration_ms: Math.round(getSessionDurationMs()),
      character_count: session.charCount,
      confidence_score: Math.round(getConfidenceScore() * 100) / 100,
    };

    // Paste-insert a clickable verification URL into compose box
    const score = metrics.confidence_score;
    let label;
    if (score >= 70) label = "Typed by a human \u2713";
    else if (score >= 40) label = "Possibly typed by a human \u26a0";
    else label = "Not typed by a human \u2717";

    const verifyText = label + " humantyped.fly.dev/v/" + token;
    const composeBox = session.activeComposeBox || findAnyComposeBox();
    if (composeBox) {
      insertViaPaste(composeBox, "\n\n" + verifyText);
    }

    // Send certification to server
    try {
      chrome.runtime.sendMessage(
        { type: "CERTIFY_POST", data: metrics },
        () => {}
      );
    } catch (_) {}

    // Wait for paste to propagate through Draft.js, then re-click.
    // IMPORTANT: certifying stays true during re-click so our handler
    // skips it. We reset AFTER button.click() returns (synchronous).
    setTimeout(() => {
      button.click();
      certifying = false;
      resetSession();
    }, 200);
  }, true);

  // ─── Badge Injection ─────────────────────────────────────────────

  let badge = null;

  function injectBadge(composeBox) {
    if (badge && document.body.contains(badge)) return;

    badge = document.createElement("div");
    badge.className = "humantyped-badge";
    badge.title = "HumanTyped — monitoring keystrokes";
    Object.assign(badge.style, {
      position: "absolute",
      bottom: "4px",
      right: "4px",
      width: "20px",
      height: "20px",
      borderRadius: "6px",
      backgroundColor: "#3b82f6",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      fontSize: "10px",
      fontWeight: "bold",
      color: "#fff",
      zIndex: "9999",
      cursor: "default",
      transition: "background-color 0.3s",
      fontFamily: "sans-serif",
      lineHeight: "1",
    });
    badge.textContent = "H";

    const parent = composeBox.closest('[data-testid]') || composeBox.parentElement;
    if (parent) {
      if (getComputedStyle(parent).position === "static") {
        parent.style.position = "relative";
      }
      parent.appendChild(badge);
    }
  }

  // ─── Tooltip ─────────────────────────────────────────────────────

  let tooltipTimeout = null;

  function showTooltip(text) {
    if (tooltipTimeout) return;
    const tooltip = document.createElement("div");
    tooltip.textContent = text;
    Object.assign(tooltip.style, {
      position: "fixed",
      bottom: "20px",
      left: "50%",
      transform: "translateX(-50%)",
      background: "#1a1a1a",
      color: "#f5f5f5",
      padding: "8px 16px",
      borderRadius: "8px",
      fontSize: "13px",
      fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
      zIndex: "999999",
      border: "1px solid #333",
      boxShadow: "0 4px 12px rgba(0,0,0,0.3)",
      transition: "opacity 0.2s",
    });
    document.body.appendChild(tooltip);
    tooltipTimeout = setTimeout(() => {
      tooltip.style.opacity = "0";
      setTimeout(() => {
        tooltip.remove();
        tooltipTimeout = null;
      }, 200);
    }, 2000);
  }

  // ─── Stats Sync to Popup ─────────────────────────────────────────

  function sendStats() {
    try {
      chrome.runtime.sendMessage({
        type: "UPDATE_SESSION",
        data: {
          charCount: session.charCount,
          wpmAverage: Math.round(getWpmAverage()),
          correctionRate: Math.round(getCorrectionRate() * 100),
          confidenceScore: getConfidenceScore(),
          sessionDurationMs: Math.round(getSessionDurationMs()),
        },
      });
    } catch (_) {}
  }

  // ─── MutationObserver (badge injection on compose box appear) ────

  const observer = new MutationObserver(() => {
    const box = findAnyComposeBox();
    if (box) injectBadge(box);
  });

  // ─── Init ────────────────────────────────────────────────────────

  function init() {
    try {
      chrome.storage.local.get({ disabledSites: [] }, (result) => {
        if (chrome.runtime.lastError) {
          startMonitoring();
          return;
        }
        if (result.disabledSites.includes(location.hostname)) {
          return;
        }
        startMonitoring();
      });
    } catch (_) {
      startMonitoring();
    }
  }

  function startMonitoring() {
    const box = findAnyComposeBox();
    if (box) {
      injectBadge(box);
    }

    observer.observe(document.body || document.documentElement, {
      childList: true,
      subtree: true,
    });

    try {
      chrome.runtime.onMessage.addListener((msg) => {
        if (msg.type === "SETTINGS_CHANGED" && msg.data && msg.data.disabledSites) {
          if (msg.data.disabledSites.includes(location.hostname)) {
            observer.disconnect();
            document.querySelectorAll(".humantyped-badge").forEach((b) => b.remove());
          }
        }
      });
    } catch (_) {}
  }

  init();
})();
