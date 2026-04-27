# flatpak-launcher

This is to wrap around the Fracture tool to catch unrecoverable errors thrown by pipewire, the event loop, WebGPU, or GTK. It works within a flatpak container. If it catches a panic, it starts again with the safest possible options.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
