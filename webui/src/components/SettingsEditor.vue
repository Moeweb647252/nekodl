<script setup lang="ts">
import { ref } from "vue";
import { Config } from "../bindings/Config";

const settings = ref<Config>();
</script>

<template>
  <div style="height: 100%; display: grid; grid-template-rows: auto 1fr">
    <a-breadcrumb>
      <a-breadcrumb-item>Settings</a-breadcrumb-item>
    </a-breadcrumb>
    <a-card title="Settings" style="height: 100%">
      <a-form
        :label-col="{ span: 4 }"
        :wrapper-col="{ span: 14 }"
        :model="settings"
      >
        <a-form-item label="下载目录" name="download">
          <a-input />
        </a-form-item>
        <a-form-item label="默认更新间隔">
          <a-input />
        </a-form-item>
        <a-form-item label="Trackers">
          <a-textarea></a-textarea>
        </a-form-item>
      </a-form>
    </a-card>
  </div>
</template>
