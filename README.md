# Keystroke-Display
A simple overlay for OBS to display consecutive keystrokes. Primarily for programming, especially for Vim.

This will work on Windows and Linux, but the `inputbot` crate unfortunately does not support Mac.

https://user-images.githubusercontent.com/23005874/231338090-107b908e-7d72-41b1-999e-68e858ad2bd7.mp4

---
## Running the overlay locally
I plan on releasing binaries eventually, but for now:

1. Clone or download repo
2. Run `cargo run`
3. In OBS, add a Browser Source, set it to use a `local file`, and point it to `overlay/index.html`

---
## Important Notes
Avoid typing passwords especially if streaming. If you must, remember to hide the overlay in OBS and/or to turn off the input server altogether. Do not forget when this program is running! I will soon add support for a hotkey that tells the input server to start/stop sending inputs.
