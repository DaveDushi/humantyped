const API_BASE = "https://humantyped.fly.dev";

// In-memory session stats from the active content script
let currentSession = null;

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  switch (msg.type) {
    case "UPDATE_SESSION":
      currentSession = msg.data;
      return false;

    case "GET_SESSION_STATS":
      sendResponse({ session: currentSession });
      return false;

    case "CERTIFY_POST":
      handleCertify(msg.data).then(sendResponse);
      return true; // async

    case "GET_TOKENS_HISTORY":
      chrome.storage.local.get({ tokenHistory: [] }, (result) => {
        sendResponse({ tokens: result.tokenHistory });
      });
      return true;

    case "GET_SETTINGS":
      chrome.storage.local.get(
        { scannerEnabled: false, disabledSites: [] },
        (result) => {
          sendResponse({
            scannerEnabled: result.scannerEnabled,
            disabledSites: result.disabledSites,
          });
        }
      );
      return true;

    case "UPDATE_SETTINGS":
      chrome.storage.local.set(msg.data, () => {
        // Notify all content scripts of setting changes
        chrome.tabs.query(
          { url: ["https://twitter.com/*", "https://x.com/*"] },
          (tabs) => {
            for (const tab of tabs) {
              chrome.tabs.sendMessage(tab.id, {
                type: "SETTINGS_CHANGED",
                data: msg.data,
              });
            }
          }
        );
        sendResponse({ success: true });
      });
      return true;

    case "TOGGLE_SCANNER":
      chrome.storage.local.set(
        { scannerEnabled: msg.enabled },
        () => {
          chrome.tabs.query(
            { url: ["https://twitter.com/*", "https://x.com/*"] },
            (tabs) => {
              for (const tab of tabs) {
                chrome.tabs.sendMessage(tab.id, {
                  type: "SCANNER_TOGGLE",
                  enabled: msg.enabled,
                });
              }
            }
          );
          sendResponse({ success: true });
        }
      );
      return true;
  }
});

async function handleCertify(data) {
  try {
    const response = await fetch(`${API_BASE}/api/certify`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });

    const result = await response.json();

    if (result.success) {
      // Save to token history (keep last 5)
      const { tokenHistory = [] } = await chrome.storage.local.get({
        tokenHistory: [],
      });
      tokenHistory.unshift({
        token: data.token,
        timestamp: Date.now(),
        confidence_score: data.confidence_score,
        verified: true,
      });
      if (tokenHistory.length > 5) tokenHistory.length = 5;
      await chrome.storage.local.set({ tokenHistory });
    }

    return { success: result.success, token: data.token, verified: true };
  } catch (err) {
    // Server unreachable — still save token as unverified
    const { tokenHistory = [] } = await chrome.storage.local.get({
      tokenHistory: [],
    });
    tokenHistory.unshift({
      token: data.token,
      timestamp: Date.now(),
      confidence_score: data.confidence_score,
      verified: false,
    });
    if (tokenHistory.length > 5) tokenHistory.length = 5;
    await chrome.storage.local.set({ tokenHistory });

    return { success: false, token: data.token, verified: false };
  }
}
