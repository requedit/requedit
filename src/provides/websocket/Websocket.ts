import { createContext, useContext } from "react";
import type { TreeDataNode } from 'antd';
import { FiltersValue } from "@/components/filters/types";

export type Message = {
  key: number;
  url: string;
  method: string;
  statusCode: number;
  status: string;
  host: string;
  category: string;
}
type WebsocketContextType = {
  messages: Message[];
  treeData: TreeDataNode[]
  setMessages: (messages: Message[]) => void;
  setTreeData: (treeData: TreeDataNode[]) => void;
};

export const WebsocketContext = createContext<WebsocketContextType>({
  messages: [],
  treeData: [],
  setMessages: (_message: Message[]) => { },
  setTreeData: (_treeData: TreeDataNode[]) => { }
})
export const useWebsocket = () => {
  return useContext(WebsocketContext);
}
export const useMessage = (filters: FiltersValue) => {
  const { messages } = useContext(WebsocketContext);
  return messages.filter(m => m.url.includes(filters.search) && (filters.category == 'All' || m.category == filters.category))
}


export const useTreeData = () => {
  return useContext(WebsocketContext).treeData;
}

export const useClearFn = () => {
  const { setMessages, setTreeData } = useContext(WebsocketContext);
  return () => {
    setMessages([]);
    setTreeData([]);
  }
}
