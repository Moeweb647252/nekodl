<script setup lang="ts">
import { onMounted, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
watch(
  () => route.params,
  () => {}
);
onMounted(() => {});
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
  <a-list item-layout="horizontal" :data-source="">
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
