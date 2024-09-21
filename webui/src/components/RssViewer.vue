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
      <a-space direction="vertical"></a-space>
    </a-col>
  </a-row>
</template>
