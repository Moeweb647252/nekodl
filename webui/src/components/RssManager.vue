<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
import { api } from "../api";
import { errNotif } from "../utils";
import { useRouter } from "vue-router";

const router = useRouter();

const new_rss_info: Ref<{
  url: string;
}> = ref({
  url: "",
});

const submit_new_rss = () => {
  api
    .add_rss_sub(new_rss_info.value.url)
    .then(() => {
      fetch_rsse_list();
      rss_add_modal.value = false;
      new_rss_info.value.url = "";
    })
    .catch((e) => errNotif(e));
};

const fetch_rsse_list = () => {
  api
    .get_rss_list()
    .then((res) => {
      data.value = res.data.rss_list;
    })
    .catch((e) => errNotif(e));
};

const data = ref([]);

const rss_add_modal = ref(false);

onMounted(() => {
  fetch_rsse_list();
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
          <template #extra>
            <a-button
              type="primary"
              @click="router.push(`/rss/${item.id}/view`)"
              >查看</a-button
            >
          </template>
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
