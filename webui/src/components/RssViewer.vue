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
  api
    .get_rss_info(route.params.id as string)
    .then((res) => {
      rss_info.value = res.data;
      console.log(res);
    })
    .catch((e) => errNotif(e));
});
</script>

<template>
  <a-row>
    <a-col :span="4">
      <a-space direction="vertical">
        <p>Title: {{}}</p>
        <p>Link: {{}}</p>
        <p>Update Interval: {{}}</p>
      </a-space>
    </a-col>
    <a-col :span="4">
      <a-space direction="vertical">
        <p>Description: {{}}</p>
        <p>Update Time: {{}}</p>
      </a-space>
    </a-col>
  </a-row>
  <hr />
  <a-list item-layout="horizontal">
    <template #renderItem="{ item }">
      <a-list-item>
        <template #extra>
          <a-button type="primary">查看</a-button>
        </template>
        <a-list-item-meta :description="item.description">
          <template #title>
            <p>{{ item.title }}</p>
          </template>
        </a-list-item-meta>
      </a-list-item>
    </template>
  </a-list>
</template>
