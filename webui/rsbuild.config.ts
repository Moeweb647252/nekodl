import { defineConfig } from "@rsbuild/core";
import Components from "unplugin-vue-components/rspack";
import { pluginVue } from "@rsbuild/plugin-vue";
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers";

export default defineConfig({
  plugins: [pluginVue()],
  tools: {
    rspack: {
      plugins: [
        Components({
          resolvers: [
            AntDesignVueResolver({
              importStyle: false,
            }),
          ],
        }),
      ],
    },
  },
});
