# Keystroke-Display
A simple overlay for OBS to display consecutive keystrokes. Primarily for programming, especially for Vim.

This will work on Windows and Linux, but the `inputbot` crate unfortunately does not support Mac.

https://user-images.githubusercontent.com/23005874/231338090-107b908e-7d72-41b1-999e-68e858ad2bd7.mp4

---
## Running the overlay locally
I plan on releasing binaries eventually, but for now there are two methods:

### As a standalone
1. Clone or download repo
2. Run `cargo run` (with an optional port argument such as `cargo run -- 8080`)
3. In OBS, add a Browser Source, set it to use a `local file`, and point it to `overlay/index.html`

### As a plugin
1. Clone or download repo
2. Run `cargo build` (optionally with the `--release` flag)
3. Copy `keystroke_overlay.dll`/`keystroke_overlay.so` (located in the `target/debug` or `target/release` directory of the project) into your OBS plugins directory, which should either be `C:/Program Files/obs-studio/obs-plugins/64bit` or `/usr/bin/obs-plugins/` on Windows and Linux respectively. Doing this will automatically start up the server any time that you open OBS.
4. In OBS, add a Browser Source, set it to use a `local file`, and point it to `overlay/index.html`

---
## Important Notes
Avoid typing passwords especially if streaming. If you must, remember to hide the overlay in OBS. Do not forget when this program is running! I will soon add support for a hotkey that tells the input server to start/stop sending inputs.

For any that may be concerned regarding security: while the server does send raw keystrokes over websocket, these will never actually leave your system or hit the network. They are instead sent through the system's loopback interface, additionally meaning that this can be run with or without an internet connection.
