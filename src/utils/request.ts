import axios from 'axios'
import { AxiosRequestConfig, AxiosResponse } from 'axios'

const ins = axios.create({
  baseURL: 'http://localhost:9900',
  timeout: 5000
})

export default function request(config: AxiosRequestConfig) {
  return ins.request(config).then((res: AxiosResponse) => {
    return res.data
  })
}
