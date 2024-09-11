import { defineStore } from "pinia";
import { ref } from "vue";

export const useStore = defineStore(
  "store",
  () => {
    const token = ref("");
    const setToken = (v: string) => {
      token.value = v;
    };
    return { token, setToken };
  },
  {
    persist: true,
  }
);
