import { defineConfig } from "@rsbuild/core";
import { pluginWasmPack } from "rsbuild-plugin-wasmpack";

export default defineConfig({
  plugins: [
    pluginWasmPack({
      crates: [
        {
          path: "wasm",
          target: "web",
        },
      ],
    }),
  ],
});
