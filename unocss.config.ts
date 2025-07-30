import { defineConfig } from "unocss";
import presetUno from "@unocss/preset-uno";
import presetRemToPx from '@unocss/preset-rem-to-px'

export default defineConfig({
  shortcuts: {
  },
  presets: [
    presetUno(),
    presetRemToPx()
  ],
  transformers: [
  ],
});
