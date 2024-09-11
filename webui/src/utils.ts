import { notification } from "ant-design-vue";
import { AxiosError } from "axios";

export const errNotif = (e: any) => {
  if (e instanceof AxiosError) {
    notification.error({
      message: "网络错误",
      placement: "topRight",
    });
    throw e;
  } else {
    notification.error({
      message: e.toString(),
      placement: "topRight",
    });
  }
};

//Bytes to hex string
export function bytesToHex(bytes: Uint8Array): string {
  let hex = "";
  for (let i = 0; i < bytes.length; i++) {
    hex += bytes[i].toString(16).padStart(2, "0");
  }
  return hex;
}
