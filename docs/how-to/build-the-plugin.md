# Build the plugin

Build the plugin from source and load it into a running Zellij session.
You need `rustup` and Zellij.

1. Build the plugin:

   ```sh
   cargo build --release
   ```

   The first build downloads the pinned Rust compiler.

2. From inside a Zellij session, open the plugin in a floating pane:

   ```sh
   zellij plugin --floating --skip-plugin-cache -- \
     "file:$PWD/target/wasm32-wasip1/release/zellij_claude_pair.wasm"
   ```

   `--skip-plugin-cache` loads this build, not a cached earlier one.
