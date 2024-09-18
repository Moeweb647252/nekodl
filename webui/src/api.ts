import axios, { Axios, Method } from "axios";
import { Sha256 } from "@aws-crypto/sha256-browser";
import { bytesToHex } from "./utils";

type ApiResponse = {
  code: number;
  msg: string;
  data: any;
};

export class Api {
  baseUrl: string;
  axios: Axios;
  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
    this.axios = new Axios({
      baseURL: this.baseUrl,
      transformRequest: axios.defaults.transformRequest,
      transformResponse: axios.defaults.transformResponse,
    });
  }

  async reqBase(
    path: string,
    data: { [key: string]: string },
    method: Method = "post"
  ): Promise<ApiResponse> {
    console.log(data);
    let resp = await this.axios.request({
      method: method,
      url: path,
      data: data,
    });
    if (resp.data.code === 200) {
      return resp.data;
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
        password: bytesToHex(await hash.digest()).toUpperCase(),
      })
    ).data.token;
  }

  async add_new_rss(url: string): Promise<ApiResponse> {
    return await this.reqBase("add_new_rss", { url: url });
  }

  async get_rss_list(): Promise<ApiResponse> {
    return await this.reqBase("get_rss_list", {}, "get");
  }

  async authorize(): Promise<ApiResponse> {
    return await this.reqBase("auth", {}, "get");
  }
}

export const api = new Api("http://localhost:8001/api");
