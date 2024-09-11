<script setup lang="ts">
import { ref } from "vue";
import {
  UserOutlined,
  EyeOutlined,
  LockOutlined,
  EyeInvisibleOutlined,
} from "@ant-design/icons-vue";

const password = ref("");
const username = ref("");
const pwdInpTyp = ref(true);

const togglePwd = () => {
  pwdInpTyp.value = !pwdInpTyp.value;
};

const login = () => {};
</script>

<template>
  <a-flex justify="center" align="center" class="bg">
    <a-card title="登录" :head-style="{ 'text-align': 'center' }">
      <a-space direction="vertical">
        <a-input v-model:value="username" placeholder="用户名">
          <template #prefix>
            <UserOutlined />
          </template>
        </a-input>
        <a-input
          v-model:value="password"
          placeholder="密码"
          :type="pwdInpTyp ? 'password' : 'text'"
        >
          <template #prefix>
            <LockOutlined />
          </template>
          <template #suffix>
            <EyeOutlined v-if="pwdInpTyp" @click="togglePwd()" />
            <EyeInvisibleOutlined v-if="!pwdInpTyp" @click="togglePwd()" />
          </template>
        </a-input>
        <a-flex justify="center"
          ><a-button type="primary" @click="login()">登录</a-button></a-flex
        >
      </a-space>
    </a-card>
  </a-flex>
</template>
