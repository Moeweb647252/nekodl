<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import RssManager from "../components/RssManager.vue";
import { api } from "../api";
import { useRouter } from "vue-router";

const router = useRouter();
const selectedKey = ref([1]);
const component = computed(() => {
  return [RssManager][selectedKey.value[0] - 1];
});

onMounted(() => {
  api.authorize().catch(() => {
    router.push({ path: "/login" });
  });
  router.push({ path: "/rss" });
});
</script>

<template>
  <a-layout>
    <a-layout-header></a-layout-header>
    <a-layout>
      <a-layout-sider style="background: #fff; box-shadow: 5px 0 5px -4px #888">
        <a-menu v-model:selectedKeys="selectedKey" :multiple="false">
          <a-menu-item key="1"
            ><router-link to="/rss">RSS</router-link></a-menu-item
          >
          <a-divider style="margin: 0px"></a-divider>
          <a-menu-item key="2">下载进度</a-menu-item>
        </a-menu>
      </a-layout-sider>
      <a-layout-content :style="{ padding: 0, margin: '0px 16px' }">
        <router-view></router-view>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>
