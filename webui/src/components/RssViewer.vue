<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import { api } from "../api";
import { errNotif } from "../utils";

const route = useRoute();
const rss_info = ref();

watch(
  () => route.params,
  () => {}
);
onMounted(() => {
  console.log(route.params);
  api
    .get_rss_info(parseInt(route.params.id as string))
    .then((res) => {
      rss_info.value = res.data;
      console.log(res);
    })
    .catch((e) => errNotif(e));
});
</script>

<template>
  <div
    style="
      display: grid;
      height: 100%;
      grid-template-rows: auto 1fr;
      padding-bottom: 0.5rem;
    "
  >
    <a-breadcrumb style="margin: 0.5rem">
      <a-breadcrumb-item>Main</a-breadcrumb-item>
      <a-breadcrumb-item>rss</a-breadcrumb-item>
    </a-breadcrumb>
    <a-card
      style="
        height: 100%;
        overflow-y: hidden;
        display: grid;
        grid-template-rows: auto 1fr;
      "
      bodyStyle="height:100%; overflow-y:hidden;"
      title="Test"
    >
      <div style="display: grid; grid-template-rows: auto 1fr; height: 100%">
        <div>
          <a-row>
            <a-col :span="12">
              <a-space direction="vertical">
                <p>Title: {{ rss_info?.title }}</p>
                <p>Link: {{ rss_info?.url }}</p>
                <p>Update Interval: {{ rss_info?.update_interval }}</p>
              </a-space>
            </a-col>
            <a-col :span="12">
              <a-space direction="vertical">
                <p>Description: {{ rss_info?.description }}</p>
                <p>Update Time: {{ rss_info?.update_time }}</p>
              </a-space>
            </a-col>
          </a-row>
        </div>
        <div style="height: 100%; overflow-y: scroll">
          <a-list item-layout="horizontal" :data-source="rss_info?.items">
            <template #renderItem="{ item }">
              <a-list-item>
                <template #extra>
                  <a-button type="primary"> 查看</a-button>
                </template>
                <a-list-item-meta :description="item.description">
                  <template #title>
                    <p>{{ item.title }}</p>
                  </template>
                </a-list-item-meta>
              </a-list-item>
            </template>
          </a-list>
        </div>
      </div>
    </a-card>
  </div>
</template>

<style scoped>
.full {
  height: 100%;
  width: 100%;
}
</style>
