(() => {
  "use strict";

  const API_BASE = "https://humantyped.fly.dev";
  const VERIFY_BASE = "https://humantyped.fly.dev/verify";
  const TOKEN_REGEX =
    /(?:\[HumanTyped:\s*([A-Za-z0-9]{12})(?:\s*—\s*unverified)?\]|humantyped\.fly\.dev\/verify\/([A-Za-z0-9]{12}))/g;
  const TWEET_TEXT_SELECTOR = '[data-testid="tweetText"]';
  const SCANNED_ATTR = "data-humantyped-scanned";

  let scannerEnabled = false;

  // ─── Badge Creation ────────────────────────────────────────────

  function createBadge(token, valid) {
    const badge = document.createElement("a");
    badge.href = `${VERIFY_BASE}/${token}`;
    badge.target = "_blank";
    badge.rel = "noopener noreferrer";

    if (valid) {
      badge.textContent = " ✅ HumanTyped ";
      Object.assign(badge.style, {
        display: "inline-flex",
        alignItems: "center",
        gap: "2px",
        padding: "1px 6px",
        borderRadius: "4px",
        backgroundColor: "rgba(16, 185, 129, 0.1)",
        color: "#10b981",
        fontSize: "12px",
        fontWeight: "600",
        textDecoration: "none",
        border: "1px solid rgba(16, 185, 129, 0.2)",
        cursor: "pointer",
        fontFamily: "sans-serif",
        lineHeight: "1.5",
      });
    } else {
      badge.textContent = " ⚠️ Unverified ";
      Object.assign(badge.style, {
        display: "inline-flex",
        alignItems: "center",
        gap: "2px",
        padding: "1px 6px",
        borderRadius: "4px",
        backgroundColor: "rgba(107, 114, 128, 0.1)",
        color: "#9ca3af",
        fontSize: "12px",
        fontWeight: "600",
        textDecoration: "none",
        border: "1px solid rgba(107, 114, 128, 0.2)",
        cursor: "pointer",
        fontFamily: "sans-serif",
        lineHeight: "1.5",
      });
    }

    badge.addEventListener("mouseenter", () => {
      badge.style.opacity = "0.8";
    });
    badge.addEventListener("mouseleave", () => {
      badge.style.opacity = "1";
    });

    return badge;
  }

  // ─── Token Scanning ────────────────────────────────────────────

  async function verifyToken(token) {
    try {
      const response = await fetch(`${API_BASE}/api/verify/${token}`);
      const data = await response.json();
      return data.valid === true;
    } catch {
      return false;
    }
  }

  async function scanElement(element) {
    if (element.hasAttribute(SCANNED_ATTR)) return;
    element.setAttribute(SCANNED_ATTR, "true");

    const walker = document.createTreeWalker(
      element,
      NodeFilter.SHOW_TEXT,
      null
    );

    const textNodes = [];
    let node;
    while ((node = walker.nextNode())) {
      if (TOKEN_REGEX.test(node.textContent)) {
        textNodes.push(node);
      }
      TOKEN_REGEX.lastIndex = 0;
    }

    for (const textNode of textNodes) {
      const text = textNode.textContent;
      const parts = [];
      let lastIndex = 0;
      let match;

      TOKEN_REGEX.lastIndex = 0;
      while ((match = TOKEN_REGEX.exec(text)) !== null) {
        // Text before the match
        if (match.index > lastIndex) {
          parts.push(document.createTextNode(text.slice(lastIndex, match.index)));
        }

        const token = match[1] || match[2];
        const valid = await verifyToken(token);
        parts.push(createBadge(token, valid));

        lastIndex = match.index + match[0].length;
      }

      // Remaining text after last match
      if (lastIndex < text.length) {
        parts.push(document.createTextNode(text.slice(lastIndex)));
      }

      if (parts.length > 0) {
        const fragment = document.createDocumentFragment();
        parts.forEach((p) => fragment.appendChild(p));
        textNode.parentNode.replaceChild(fragment, textNode);
      }
    }
  }

  function scanAllTweets() {
    if (!scannerEnabled) return;
    const tweets = document.querySelectorAll(TWEET_TEXT_SELECTOR);
    tweets.forEach(scanElement);
  }

  // ─── MutationObserver ──────────────────────────────────────────

  let observer = null;

  function startObserver() {
    if (observer) return;
    observer = new MutationObserver((mutations) => {
      for (const mutation of mutations) {
        for (const node of mutation.addedNodes) {
          if (node.nodeType !== Node.ELEMENT_NODE) continue;
          const tweets = node.querySelectorAll
            ? node.querySelectorAll(TWEET_TEXT_SELECTOR)
            : [];
          tweets.forEach(scanElement);
          if (node.matches && node.matches(TWEET_TEXT_SELECTOR)) {
            scanElement(node);
          }
        }
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true,
    });
  }

  function stopObserver() {
    if (observer) {
      observer.disconnect();
      observer = null;
    }
  }

  // ─── Enable/Disable ───────────────────────────────────────────

  function enableScanner() {
    scannerEnabled = true;
    scanAllTweets();
    startObserver();
  }

  function disableScanner() {
    scannerEnabled = false;
    stopObserver();
  }

  // ─── Init ──────────────────────────────────────────────────────

  chrome.storage.local.get({ scannerEnabled: false }, (result) => {
    if (result.scannerEnabled) {
      enableScanner();
    }
  });

  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.type === "SCANNER_TOGGLE") {
      if (msg.enabled) {
        enableScanner();
      } else {
        disableScanner();
      }
    }
  });
})();
