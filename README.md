# HumanTyped

**Cryptographic proof that a human actually typed that post.**

HumanTyped is a Chrome extension + Rust server that watches how you type on X/Twitter, analyzes your keystroke biometrics in real-time, and appends a verifiable certification link to your post. Anyone can click the link to confirm the post was typed by a human — not pasted from ChatGPT.

---

## Why?

Copy-pasting AI-generated text into social media is trivially easy and completely undetectable. HumanTyped makes it detectable. If you typed it yourself, prove it.

## How It Works

1. **Type** — The extension monitors your keystrokes in the X/Twitter compose box: dwell times, flight times, WPM variance, correction rate, and session duration. Paste and drag-and-drop are blocked.

2. **Score** — A confidence score (0–100) is computed from your typing biometrics. Superhuman WPM, zero corrections, robotic timing, and impossibly fast sessions all tank the score.

3. **Certify** — When you hit Post, the extension generates a cryptographic token, sends your biometric summary to the server, and injects a `humantyped.com/verify/<token>` link into your tweet.

4. **Verify** — Anyone can click the link to see the full typing analysis: WPM, variance, correction rate, session duration, and confidence score.

## Confidence Scoring

The score starts at 100 and gets penalized for suspicious patterns:

| Signal | Penalty | Rationale |
|---|---|---|
| WPM variance < 5 | -30 | Humans don't type at constant speed |
| WPM > 150 | -25 | Faster than competitive typists |
| Zero corrections (50+ chars) | -20 | Humans make typos and fix them |
| Session too fast for char count | -25 | Duration vs character count mismatch |
| Flight time std dev < 10ms | -20 | Robotic inter-key timing |

A score of **70+** generally indicates genuine human typing.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (for the server)
- Chrome or Chromium-based browser (for the extension)

### Server

```bash
cd server
cargo run
```

The server starts on `http://localhost:3000` and creates a SQLite database automatically. No config needed.

**Optional environment variables:**

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Server port |
| `DATABASE_PATH` | `humantyped.db` | SQLite file path |

### Extension

1. Open `chrome://extensions`
2. Enable **Developer mode** (top right)
3. Click **Load unpacked** and select the `extension/` directory
4. Go to [x.com](https://x.com) — you should see the blue "H" badge in compose boxes

The extension talks to `localhost:3000` by default. Change `API_BASE` in `background.js` and `scanner.js` to point elsewhere.

## Architecture

```
+---------------------+        HTTPS         +-----------------------+
|  Chrome Extension   | ------------------> |   Rust Server          |
|  (Manifest V3)      |  POST /api/certify  |   (axum + Leptos SSR)  |
|                     |  GET  /api/verify   |                       |
|  content.js          |                     |  SQLite (WAL mode)    |
|   keystroke capture  | <------------------ |  Rate limiting        |
|   paste blocking     |   verification      |  Token storage        |
|   confidence scoring |   responses         |                       |
|                     |                     |  Routes:              |
|  inject.js           |                     |   /          landing  |
|   fetch interception |                     |   /feed      feed     |
|                     |                     |   /verify/:t verify   |
|  scanner.js          |                     |                       |
|   timeline badges    |                     +-----------------------+
|                     |                              |
|  background.js       |                      +------v------+
|   message routing    |                      | humantyped.db|
|                     |                      +-------------+
|  popup.html/js       |
|   live stats UI      |
+---------------------+
```

## Tech Stack

| Layer | Tech |
|---|---|
| Server | Rust, axum, tokio, tokio-rusqlite, SQLite (WAL) |
| SSR | Leptos 0.7 (server-side rendering, no WASM) |
| Styles | Tailwind CSS |
| Extension | Chrome Manifest V3, vanilla JS |

## Project Structure

```
humantyped/
  extension/
    manifest.json        Chrome MV3 manifest
    content.js           Keystroke capture, paste blocking, token injection
    inject.js            MAIN world fetch monitor for CreateTweet
    scanner.js           Timeline scanning for verification badges
    background.js        Service worker, message routing, API calls
    popup.html/js/css    Extension popup UI
  server/
    src/
      main.rs            axum router, server setup
      api.rs             JSON API handlers
      db.rs              SQLite schema and queries
      pages.rs           Leptos SSR page handlers
      rate_limit.rs      IP-based rate limiter
      components/        Leptos SSR components
    Cargo.toml
```

## Privacy

**Collected (server-side):**
- Aggregate keystroke stats: average WPM, variance, correction rate
- Session metadata: duration, character count
- Confidence score
- A random 12-character token — not tied to your identity

**Not collected:**
- The actual text you typed
- Individual keystrokes or their timings
- Your X/Twitter username or any account info
- Cookies or IP addresses (rate limiting is in-memory only)

No raw biometric data leaves the extension. The server stores only the summary metrics needed for the verification page.

## Contributing

1. Fork and clone
2. Pick an issue or open one describing what you want to work on
3. Keep PRs small and focused
4. Test with the extension loaded against `localhost:3000`

### Areas That Need Work

- Support for more platforms beyond X/Twitter
- Firefox extension
- More sophisticated biometric analysis (n-graph patterns, pressure sensitivity)
- Public API for third-party verification widgets
- ML-based anomaly detection for confidence scoring

## License

MIT
