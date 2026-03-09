<p align="center">
  <strong>Human</strong>Typed
</p>

<h3 align="center">Prove Your Posts Are Human</h3>

<p align="center">
  Keystroke biometric verification for social media.<br/>
  Free &amp; open source.
</p>

<p align="center">
  <a href="https://github.com/DaveDushi/humantyped">GitHub</a>&nbsp;&nbsp;&bull;&nbsp;&nbsp;<a href="https://humantyped.fly.dev">Website</a>&nbsp;&nbsp;&bull;&nbsp;&nbsp;<a href="https://humantyped.fly.dev/feed">Verified Posts</a>
</p>

---

HumanTyped analyzes your typing patterns — not your content — to certify social media posts were genuinely typed by a human. Every certified post gets a verifiable token anyone can check.

## AI Can Write. But It Can't Type.

AI-generated text is flooding social media. Copy-pasting from ChatGPT is trivially easy and completely undetectable. HumanTyped makes it detectable — by analyzing the one thing AI can't fake: **the way your fingers move across the keyboard.**

## How It Works

Three steps. Two minutes. Zero data collected.

**1. Install the Extension** — Add HumanTyped to Chrome in 2 minutes. It runs silently in the background on X/Twitter — no setup, no account.

**2. Type Your Post** — Write naturally. We monitor timing between keys — never the keys themselves. Pasted content is detected and excluded.

**3. Anyone Can Verify** — A unique token is appended to your post. Anyone can click it to see your typing analysis and human confidence score.

## What Gets Verified

Anonymized metrics that prove human typing patterns:

| Metric | Example |
|---|---|
| Words per minute | 72 |
| WPM variance | 14.2 |
| Correction rate | 8% |
| Confidence score | 94% |

The confidence score starts at 100 and gets penalized for suspicious patterns — robotic timing consistency, superhuman speed, zero corrections, and impossibly fast sessions. **70+** generally means a real human typed it.

## Privacy First. Always.

Built with a simple rule: **never collect what you don't need.**

**What we analyze:**
- Timing between keystrokes — dwell and flight times only
- Typing speed variations — average WPM and variance
- Correction patterns — backspace rate and rhythm
- Session duration — how long you spent typing

**What we never touch:**
- The keys you press — we never know which keys
- Your post content — text never leaves your browser
- Your username or identity — no accounts, no tracking
- IP address or cookies — zero server-side tracking

> This is not a keylogger. We never record which keys you press — only the timing between them.
>
> Don't trust us? [Read the code.](https://github.com/DaveDushi/humantyped)

## Install in 2 Minutes

No Chrome Web Store needed. No account required.

**1. Download the extension**

Clone the repo or download the ZIP:

```
git clone https://github.com/DaveDushi/humantyped.git
```

No Git? [Download ZIP instead.](https://github.com/DaveDushi/humantyped/archive/refs/heads/main.zip)

**2. Open Chrome Extensions**

Type this in your address bar:

```
chrome://extensions
```

Or go to Menu > Extensions > Manage Extensions.

**3. Enable Developer Mode & Load**

Toggle **Developer Mode** on (top right), then click **Load Unpacked** and select the `extension/` folder.

**4. Start Posting!**

Go to [x.com](https://x.com) and compose a tweet. Look for the blue **H** badge. Type your post naturally — a verification link is appended automatically when you hit Post.

<details>
<summary><strong>Is Developer Mode safe?</strong></summary>
<br/>
Yes. Developer Mode is how all Chrome extensions are developed and tested. It's used by millions of developers daily and doesn't compromise your browser's security. You can disable it anytime.
</details>

<details>
<summary><strong>I don't have Git installed</strong></summary>
<br/>
No problem! Use the "Download ZIP" link in step 1. Extract the ZIP file, then select the <code>extension/</code> folder inside when loading unpacked. No terminal needed.
</details>

<details>
<summary><strong>How do I uninstall?</strong></summary>
<br/>
Go to <code>chrome://extensions</code> and click Remove on the HumanTyped card. That's it — completely gone in one click.
</details>

<details>
<summary><strong>Why isn't this on the Chrome Web Store?</strong></summary>
<br/>
We're working on it! In the meantime, installing from source means you can verify every line of code yourself.
</details>

## Contributing

1. Fork and clone
2. Pick an issue or open one describing what you want to work on
3. Keep PRs small and focused
4. Test with the extension loaded against the live server or `localhost:3000`

> After making changes to extension code, go to `chrome://extensions` and click the reload icon on the HumanTyped card to pick up your changes.

### Running the Server Locally

```bash
cd server
cargo run
# => http://localhost:3000
```

### Areas That Need Work

- Support for more platforms beyond X/Twitter
- Firefox extension
- More sophisticated biometric analysis (n-graph patterns, pressure sensitivity)
- Public API for third-party verification widgets
- ML-based anomaly detection for confidence scoring

### Tech Stack

| Layer | Tech |
|---|---|
| Server | Rust, axum, tokio, tokio-rusqlite, SQLite (WAL) |
| SSR | Leptos 0.7 (server-side rendering, no WASM) |
| Styles | Tailwind CSS |
| Extension | Chrome Manifest V3, vanilla JS |

### Project Structure

```
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

## License

MIT

---

<p align="center">
  <sub>No keystrokes stored. No identity tracked. Just proof of humanity.</sub>
</p>
