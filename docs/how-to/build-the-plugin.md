# Build the plugin

Build the plugin from source and load it into a running Zellij session.
You need `rustup` and Zellij.

1. Clone the repository:

   ```sh
   git clone https://github.com/alunduil/zellij-claude-pair.git
   cd zellij-claude-pair
   ```

2. Build the plugin:

   ```sh
   cargo build --release
   ```

   The first build downloads the pinned Rust compiler.

3. From inside a Zellij session, open the plugin in a floating pane:

   ```sh
   zellij plugin --floating --skip-plugin-cache \
     --configuration surface=branch -- \
     "file:$PWD/target/wasm32-wasip1/release/zellij-claude-pair.wasm"
   ```

   `--skip-plugin-cache` loads this build, not a cached earlier one.
   `surface` picks what the instance shows: `branch`, `pr`, or `picker`.

To load several surfaces in a new session, run from the repository root:

```sh
zellij --layout examples/separate-instances.kdl
```
