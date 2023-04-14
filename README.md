# Keystroke-Display
A simple overlay for OBS to display consecutive keystrokes. Primarily for programming, especially for Vim.

This has as of yet only been tested on Windows, but I do not believe there are any compatibility concerns for Mac or Linux.

https://user-images.githubusercontent.com/23005874/231338090-107b908e-7d72-41b1-999e-68e858ad2bd7.mp4

---
## Running the overlay locally
I plan on releasing binaries eventually, but for now:

1. Clone or download repo
2. Run `cargo run`
3. In OBS, add a Browser Source, set it to use a `local file`, and point it to `overlay/index.html`

---
## Important Notes
This program is by no means secure, and should only be used on trusted networks. Obviously avoid typing passwords especially if streaming, if you must, remember to hide the overlay in OBS and/or to turn off the input server altogether. Do not forget when this program is running! I will soon add support for a hotkey that tells the input server to start/stop sending inputs over the network. I will likely also have it automatically stop if there is no display (in other words if OBS is closed), but this won't stop anyone on the network from viewing inputs by simply going to the display page.
