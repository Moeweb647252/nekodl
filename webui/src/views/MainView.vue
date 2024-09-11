<script setup lang="ts">
import { computed, ref } from "vue";
import RssManager from "../components/RssManager.vue";

const selectedKey = ref([1]);
const component = computed(() => {
  return [RssManager][selectedKey.value[0] - 1];
});
</script>

<template>
  <a-layout>
    <a-layout-header></a-layout-header>
    <a-layout>
      <a-layout-sider style="background: #fff; box-shadow: 5px 0 5px -4px #888">
        <a-menu v-model:selectedKeys="selectedKey" :multiple="false">
          <a-menu-item key="1">RSS</a-menu-item>
          <a-divider style="margin: 0px"></a-divider>
          <a-menu-item key="2">下载进度</a-menu-item>
        </a-menu>
      </a-layout-sider>
      <a-layout-content :style="{ padding: 0, margin: '24px' }">
        <component :is="component"></component>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>
