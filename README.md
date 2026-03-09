# HumanTyped

**Cryptographic proof that a human actually typed that post.**

---

HumanTyped is a Chrome extension + Rust server that watches how you type on X/Twitter, analyzes your keystroke biometrics in real-time, and appends a verifiable certification token to your post. Anyone can click the link to confirm the post was typed by a human -- not pasted from ChatGPT.

## Why This Exists

Copy-pasting AI-generated text into social media is trivially easy and completely undetectable. HumanTyped makes it detectable. If you typed it yourself, prove it.

## How It Works

1. **Monitor** -- The Chrome extension hooks into X/Twitter's compose box and records keystroke timing: dwell times (how long each key is held), flight times (gaps between keystrokes), WPM variance, correction rate, and session duration. Paste and drag-and-drop are blocked.

2. **Score** -- A confidence score (0-100) is computed from your typing biometrics. Superhuman WPM, zero corrections, robotic timing consistency, and impossibly fast sessions all tank the score.

3. **Certify** -- When you hit Post, the extension intercepts the click, generates a 12-character cryptographic token, sends the biometric summary to the server, and injects a `humantyped.com/verify/<token>` link into your tweet.

4. **Verify** -- Anyone seeing the link can click it to view the full typing analysis: WPM, variance, correction rate, session duration, and confidence score. The server also renders SSR verification pages via Leptos.

## Architecture

```
+---------------------+         HTTPS POST /api/certify
|   Chrome Extension  | -------------------------------------> +-----------------------+
|   (Manifest V3)     |                                        |   Rust Server          |
|                     |         GET /api/verify/{token}        |   (axum + Leptos SSR)  |
|  - content.js       | <------------------------------------- |                       |
|    keystroke capture |                                        |  - SQLite (WAL mode)  |
|    paste blocking    |                                        |  - Rate limiting      |
|    confidence score  |                                        |  - Token storage      |
|                     |                                        |                       |
|  - inject.js        |                                        |  Routes:              |
|    fetch monitor     |                                        |  /           landing  |
|                     |                                        |  /feed       feed     |
|  - scanner.js       |         GET /verify/{token}            |  /verify/:t  verify   |
|    tweet badge scan  | -------------------------------------> |                       |
|                     |                                        +-----------+-----------+
|  - background.js    |                                                    |
|    message routing   |                                                    |
|                     |                                            +-------v-------+
|  - popup.html/js    |                                            |  humantyped.db |
|    live stats UI     |                                            |  (SQLite)      |
+---------------------+                                            +---------------+
```

## Tech Stack

| Layer     | Tech                                                        |
|-----------|-------------------------------------------------------------|
| Server    | Rust, axum 0.8, tokio, tokio-rusqlite, SQLite (WAL mode)    |
| SSR Pages | Leptos 0.7 (server-side rendering, no WASM)                 |
| Styles    | Tailwind CSS (built in Docker stage)                        |
| Extension | Chrome Manifest V3, vanilla JS, content scripts + MAIN world injection |
| Deploy    | Docker multi-stage build, Fly.io with persistent volume     |

## Quick Start

### Server

```bash
cd server

# Install Rust if needed: https://rustup.rs
cargo run
# => HumanTyped server running on http://localhost:3000
```

The server creates `humantyped.db` in the working directory on first run. No config needed.

**Environment variables (optional):**
- `PORT` -- Server port (default: `3000`)
- `DATABASE_PATH` -- SQLite file path (default: `humantyped.db`)

### Extension

1. Open `chrome://extensions` in Chrome
2. Enable "Developer mode" (top right)
3. Click "Load unpacked" and select the `extension/` directory
4. Navigate to [x.com](https://x.com) -- you should see the blue "H" badge in compose boxes

The extension talks to `http://localhost:3000` by default. Change `API_BASE` in `background.js` and `scanner.js` for production.

### Docker

```bash
cd server
docker build -t humantyped .
docker run -p 3000:3000 -v humantyped_data:/app/data humantyped
```

### Deploy to Fly.io

```bash
cd server
fly launch    # first time
fly deploy    # subsequent deploys
```

The `fly.toml` configures a persistent volume at `/app/data` for the SQLite database.

## API Endpoints

### `POST /api/certify`

Submit a typing certification. Called by the extension when you post a tweet.

**Request body:**
```json
{
  "token": "aB3xK9mQ2pLw",
  "wpm_average": 72.45,
  "wpm_variance": 18.32,
  "correction_rate": 0.087,
  "session_duration_ms": 34520,
  "character_count": 142,
  "confidence_score": 85.00
}
```

**Response (201):**
```json
{
  "success": true,
  "token": "aB3xK9mQ2pLw",
  "verify_url": "/verify/aB3xK9mQ2pLw"
}
```

**Errors:** `400` (bad token format or invalid score), `409` (duplicate token), `429` (rate limited -- 100 req/hour per IP).

### `GET /api/verify/{token}`

Verify a token and retrieve its biometric data.

**Response (valid):**
```json
{
  "valid": true,
  "token": "aB3xK9mQ2pLw",
  "wpm_average": 72.45,
  "wpm_variance": 18.32,
  "correction_rate": 0.087,
  "session_duration_ms": 34520,
  "character_count": 142,
  "confidence_score": 85.00,
  "created_at": "2026-03-09 14:22:31"
}
```

**Response (not found):**
```json
{
  "valid": false
}
```

### `GET /api/tokens`

List recent certifications (last 50) with total count.

### HTML Pages

| Route              | Description                          |
|--------------------|--------------------------------------|
| `/`                | Landing page                         |
| `/feed`            | Public feed of recent certifications |
| `/verify/{token}`  | Human-readable verification page     |

## How Verification Scoring Works

The confidence score starts at 100 and gets penalized for suspicious patterns:

| Signal                         | Penalty | Why                                    |
|--------------------------------|---------|----------------------------------------|
| WPM variance < 5              | -30     | Humans don't type at constant speed    |
| WPM average > 150             | -25     | Faster than competitive typists        |
| Zero corrections (50+ chars)  | -20     | Humans make typos and fix them         |
| Session too fast for chars     | -25     | Duration vs character count mismatch   |
| Flight time std dev < 10ms    | -20     | Robotic inter-key timing              |

A score of 70+ generally indicates genuine human typing. Lower scores suggest paste, automation, or AI-assisted composition.

## Extension Features

- **Paste blocking** -- `Ctrl+V`, drag-and-drop, and `beforeinput` paste events are all intercepted and blocked in the compose box
- **Live stats popup** -- Shows real-time WPM, character count, correction rate, and confidence score
- **Scanner mode** -- Optionally scans your timeline for `humantyped.com/verify/` links and replaces them with inline verification badges
- **Per-site toggle** -- Disable HumanTyped on specific sites from the popup
- **Token history** -- Last 5 certifications stored locally with status

## Privacy

**What is collected (server-side):**
- Aggregate keystroke statistics: average WPM, WPM variance, correction rate
- Session metadata: duration, character count
- Computed confidence score
- A random 12-character token (not tied to your identity)

**What is NOT collected:**
- The actual text you typed
- Individual keystrokes or their timings
- Your Twitter/X username or any account info
- Cookies, IP addresses (beyond rate limiting, which is in-memory and not persisted)
- Any data from pages you browse without typing

The server stores only the summary metrics needed to display on the verification page. No raw biometric data leaves the extension.

## Project Structure

```
humantyped/
  extension/
    manifest.json      # Chrome MV3 manifest
    content.js         # Keystroke capture, paste blocking, token injection
    inject.js          # MAIN world fetch monitor for CreateTweet
    scanner.js         # Timeline scanning for verification badges
    background.js      # Service worker, message routing, API calls
    popup.html/js/css  # Extension popup UI
  server/
    src/
      main.rs          # axum router, server setup
      api.rs           # JSON API handlers
      db.rs            # SQLite schema and queries
      pages.rs         # Leptos SSR page handlers
      rate_limit.rs    # IP-based rate limiter
      components/      # Leptos SSR components (landing, verify, feed, etc.)
    Cargo.toml
    Dockerfile         # Multi-stage: Tailwind CSS -> Rust build -> slim runtime
    fly.toml           # Fly.io deployment config
```

## Contributing

This is an early-stage project. If you want to help:

1. Fork and clone
2. Pick an issue or open one describing what you want to work on
3. Keep PRs small and focused
4. Test with the extension loaded against `localhost:3000`

Some areas that need work:
- Support for more platforms beyond X/Twitter
- Browser extension for Firefox
- More sophisticated biometric analysis (n-graph patterns, pressure sensitivity)
- Public API for third-party verification widgets
- Improved confidence scoring with ML-based anomaly detection

## License

MIT
