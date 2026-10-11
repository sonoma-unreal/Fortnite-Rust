# fn-web-demo

This crate is a deliberately small browser/WASM scaffold created to test the first realistic step of porting `Fortnite-Rust` out of its native desktop assumptions.

It does not try to run the game. Instead, it proves that the project can at least reach a browser target and initialize a canvas before the real blockers begin:

- native `winit` windowing and desktop event loop
- filesystem-based asset loading (`--assets`, `FN_ASSETS`, `fn-pak` direct disk reads)
- native Windows-only app startup / message-box flow
- desktop audio backends and OS input APIs
- CPU/GPU assumptions in the renderer that are not browser-safe

The stopping point is the following:

1. `fn-app` is hard-wired to `winit` and Win32-style startup.
2. Game assets are loaded from local disk and pak files, not Web fetches.
3. `fn-render` is built around `wgpu` desktop targets and a native event loop.
4. Browser/WASM support is not a small plugin; it requires a web-specific runtime and renderer split.

This crate is kept intentionally tiny so the project remains useful as a reference point before the full browser port is attempted.

## Try it

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown -p fn-web-demo
```

The build is a checkpoint, not a game. It is the boundary where the project stops being a desktop-only game and starts becoming a browser porting project.
