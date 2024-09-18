<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
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
    .add_new_rss(new_rss_info.value.url)
    .then(() => {})
    .catch((e) => errNotif(e));
};

const fetch_rsses = () => {};

const data = ref([]);

const rss_add_modal = ref(false);

onMounted(() => {
  fetch_rsses();
});
</script>

<template style="height: 100%">
  <a-card title="RSS订阅" class="full">
    <template #extra>
      <a-button type="primary" @click="rss_add_modal = true">添加</a-button>
    </template>
    <a-list item-layout="horizontal" :data-source="data">
      <template #renderItem="{ item }">
        <a-list-item>
          <a-list-item-meta :description="item.description">
            <template #title>
              <p>{{ item.title }}</p>
            </template>
          </a-list-item-meta>
        </a-list-item>
      </template>
    </a-list>
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
