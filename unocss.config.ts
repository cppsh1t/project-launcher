import { defineConfig } from "unocss";
import presetWind3 from '@unocss/preset-wind3'
import presetRemToPx from '@unocss/preset-rem-to-px'

export default defineConfig({
  shortcuts: {
  },
  presets: [
    presetWind3,
    presetRemToPx()
  ],
  transformers: [
  ],
});
