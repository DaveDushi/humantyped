const VERIFY_BASE = "http://localhost:3000/verify";

const $charCount = document.getElementById("charCount");
const $wpmAverage = document.getElementById("wpmAverage");
const $correctionRate = document.getElementById("correctionRate");
const $confidenceScore = document.getElementById("confidenceScore");
const $confidenceFill = document.getElementById("confidenceFill");
const $trustDot = document.getElementById("trustDot");
const $trustText = document.getElementById("trustText");
const $tokenList = document.getElementById("tokenList");
const $siteToggle = document.getElementById("siteToggle");
const $scannerToggle = document.getElementById("scannerToggle");

// ─── Session Stats ─────────────────────────────────────────────

function updateSessionUI(session) {
  if (!session) {
    $charCount.textContent = "0";
    $wpmAverage.textContent = "0";
    $correctionRate.textContent = "0%";
    $confidenceScore.textContent = "—";
    $confidenceFill.style.width = "0%";
    $trustDot.className = "trust-dot";
    $trustText.textContent = "No active session";
    return;
  }

  $charCount.textContent = session.charCount || 0;
  $wpmAverage.textContent = session.wpmAverage || 0;
  $correctionRate.textContent = (session.correctionRate || 0) + "%";

  const confidence = session.confidenceScore || 0;
  $confidenceScore.textContent = Math.round(confidence) + "%";
  $confidenceFill.style.width = confidence + "%";

  // Color the confidence bar
  $confidenceFill.className = "confidence-fill";
  if (confidence < 50) {
    $confidenceFill.classList.add("red");
  } else if (confidence < 80) {
    $confidenceFill.classList.add("yellow");
  }

  // Trust indicator
  if (confidence >= 80) {
    $trustDot.className = "trust-dot green";
    $trustText.textContent = "Human typing detected";
  } else if (confidence >= 50) {
    $trustDot.className = "trust-dot yellow";
    $trustText.textContent = "Moderate confidence";
  } else {
    $trustDot.className = "trust-dot red";
    $trustText.textContent = "Suspicious activity";
  }
}

function pollStats() {
  chrome.runtime.sendMessage({ type: "GET_SESSION_STATS" }, (response) => {
    if (response) updateSessionUI(response.session);
  });
}

// ─── Token History ─────────────────────────────────────────────

function renderTokenHistory(tokens) {
  if (!tokens || tokens.length === 0) {
    $tokenList.innerHTML = '<p class="empty-text">No certifications yet</p>';
    return;
  }

  $tokenList.innerHTML = "";
  for (const t of tokens) {
    const a = document.createElement("a");
    a.className = "token-item";
    a.href = `${VERIFY_BASE}/${t.token}`;
    a.target = "_blank";
    a.rel = "noopener noreferrer";

    const timeAgo = formatTimeAgo(t.timestamp);

    a.innerHTML = `
      <span class="token-code">${t.token}</span>
      <span class="token-meta">
        <span class="token-time">${timeAgo}</span>
        <span class="token-status">${t.verified ? "✅" : "⚠️"}</span>
      </span>
    `;

    $tokenList.appendChild(a);
  }
}

function formatTimeAgo(timestamp) {
  const seconds = Math.floor((Date.now() - timestamp) / 1000);
  if (seconds < 60) return "just now";
  if (seconds < 3600) return Math.floor(seconds / 60) + "m ago";
  if (seconds < 86400) return Math.floor(seconds / 3600) + "h ago";
  return Math.floor(seconds / 86400) + "d ago";
}

// ─── Toggles ───────────────────────────────────────────────────

async function initToggles() {
  // Get current tab hostname
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  const hostname = tab?.url ? new URL(tab.url).hostname : "";

  chrome.runtime.sendMessage({ type: "GET_SETTINGS" }, (response) => {
    if (!response) return;

    const isDisabled = (response.disabledSites || []).includes(hostname);
    $siteToggle.checked = !isDisabled;
    $scannerToggle.checked = response.scannerEnabled || false;
  });

  $siteToggle.addEventListener("change", () => {
    chrome.runtime.sendMessage({ type: "GET_SETTINGS" }, (response) => {
      const sites = response.disabledSites || [];
      if ($siteToggle.checked) {
        const idx = sites.indexOf(hostname);
        if (idx > -1) sites.splice(idx, 1);
      } else {
        if (!sites.includes(hostname)) sites.push(hostname);
      }
      chrome.runtime.sendMessage({
        type: "UPDATE_SETTINGS",
        data: { disabledSites: sites },
      });
    });
  });

  $scannerToggle.addEventListener("change", () => {
    chrome.runtime.sendMessage({
      type: "TOGGLE_SCANNER",
      enabled: $scannerToggle.checked,
    });
  });
}

// ─── Init ──────────────────────────────────────────────────────

pollStats();
setInterval(pollStats, 1000);

chrome.runtime.sendMessage({ type: "GET_TOKENS_HISTORY" }, (response) => {
  if (response) renderTokenHistory(response.tokens);
});

initToggles();
