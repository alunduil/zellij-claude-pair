# Build the plugin

Build the plugin from source and load it into a running Zellij session.
You need `rustup` and Zellij.

1. Build the plugin:

   ```sh
   cargo build --release
   ```

   `rust-toolchain.toml` pins the compiler and the `wasm32-wasip1` target,
   and `rustup` installs both on the first build. The plugin lands at
   `target/wasm32-wasip1/release/zellij_claude_pair.wasm`.

2. From inside a Zellij session, open the plugin in a floating pane:

   ```sh
   zellij plugin --floating --skip-plugin-cache -- \
     "file:$PWD/target/wasm32-wasip1/release/zellij_claude_pair.wasm"
   ```

   `--skip-plugin-cache` makes Zellij load the fresh build instead of a
   cached copy from an earlier one.
