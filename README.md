# Serial Coms

A Rust / Ratatui starter with a channel sidebar, sample chat messages, and a working message input. Serial data is mocked; there is no hardware connection yet. Messages are kept in memory until the app exits.

```sh
cargo run
```

Use a terminal at least 44 columns wide and 12 rows high.

| Key | Action |
| --- | --- |
| Tab / Shift+Tab | Switch between sidebar and input |
| ↑ / ↓ or j / k | Select a channel while the sidebar is focused |
| Enter / i | Start composing from the sidebar |
| Enter | Send the message while composing |
| Backspace | Remove the last character |
| Esc | Return to the sidebar |
| q | Quit from the sidebar |
| Ctrl+C | Quit from anywhere |

`src/app.rs` owns state and keyboard handling; `src/ui.rs` renders the layout. `src/main.rs` starts the app using Ratatui's terminal setup and restoration helper.
