
import request from "@/utils/request";

export const login = () => {
  return request({
    url: "/api/auth/login",
    method: "get",
  });
}
