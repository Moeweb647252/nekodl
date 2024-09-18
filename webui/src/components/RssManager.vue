<script setup lang="ts">
import { Ref, ref } from "vue";
import { api } from "../api";
import { errNotif } from "../utils";

const new_rss_info: Ref<{
  name: string;
  url: string;
}> = ref({
  name: "",
  url: "",
});

const submit_new_rss = () => {
  api
    .add_new_rss(new_rss_info.value.name, new_rss_info.value.url)
    .then(() => {})
    .catch((e) => errNotif(e));
};

const rss_add_modal = ref(false);
</script>

<template style="height: 100%">
  <a-card title="RSS订阅" class="full">
    <template #extra>
      <a-button type="primary" @click="rss_add_modal = true">添加</a-button>
    </template>
  </a-card>
  <a-modal
    v-model:open="rss_add_modal"
    title="添加RSS订阅"
    @ok="submit_new_rss"
  >
    <a-space direction="vertical" style="width: 100%">
      <div>名称: <a-input v-model:value="new_rss_info.name"></a-input></div>
      <div>链接: <a-input v-model:value="new_rss_info.url"></a-input></div>
    </a-space>
  </a-modal>
</template>

<style scoped>
.full {
  height: 100%;
  width: 100%;
}
</style>
