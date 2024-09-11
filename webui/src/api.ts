import "axios";
import { Axios } from "axios";
import { Sha256 } from "@aws-crypto/sha256-browser";

type ApiResponse = {
  code: number;
  msg: string;
  data: any;
};

class Api {
  baseUrl: string;
  axios: Axios;
  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
    this.axios = new Axios({
      baseURL: baseUrl,
    });
  }

  async reqBase(path: string, data: any): Promise<ApiResponse> {
    let resp = await this.axios.post(path, data);
    if (resp.data.code === 200) {
      return resp.data.data.token;
    } else {
      throw new Error(resp.data.msg);
    }
  }

  async login(username: string, password: string): Promise<string> {
    const hash = new Sha256();
    hash.update(password, "utf8");
    return (
      await this.reqBase("login", {
        username: username,
        password: (await hash.digest()).toString(),
      })
    ).data.token;
  }
}
