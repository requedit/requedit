import { createContext, useContext } from "react";
import type { TreeDataNode } from 'antd';
import { FiltersValue } from "@/components/filters/types";
import { Category } from "@/constants/catetory";


export type RequestRecord = {
  id: number;
  req: {
    uri: string;
    host: string;
    path: string;
    query: string;
    version: string;
    protocol: string;
    method: string;
    headers: Record<string, string>;
    body: string;
  },
  res?: {
    status: string;
    version: string;
    headers: Record<string, string>;
    body: string;
  }
}
export type ThemeType = 'auto' | 'light' | 'dark'
type AppContextType = {
  theme: ThemeType;
  domains: string[];
  requests: RequestRecord[];
  treeData: TreeDataNode[]
  setRequests: (requests: RequestRecord[]) => void;
  setTreeData: (treeData: TreeDataNode[]) => void;
  toggleTheme: () => void;
  setDomains: (domains: string[]) => void;
};

export const AppContext = createContext<AppContextType>({
  theme: 'auto',
  domains: [],
  requests: [],
  treeData: [],
  setRequests: (_message: RequestRecord[]) => { },
  setTreeData: (_treeData: TreeDataNode[]) => { },
  toggleTheme: () => { },
  setDomains: (_domains: string[]) => { },
})

export const useRequests = (filters: FiltersValue) => {
  const { requests, domains } = useContext(AppContext);
  const domain = domains[0];
  return filterRequest(requests, filters).filter((req) => {
    if (domain == 'all' || !domain) return true;
    return req.req.host === domain;
  });
}


export const useTreeData = () => {
  return useContext(AppContext).treeData;
}

export const useClearFn = () => {
  const { setRequests, setTreeData } = useContext(AppContext);
  return () => {
    setRequests([]);
    setTreeData([]);
  }
}

export const useTheme = () => {
  const { theme } = useContext(AppContext);
  return theme;
}

export const useToggleTheme = () => {
  const { toggleTheme } = useContext(AppContext);
  return toggleTheme;
}

export const useDomain = () => {
  const { domains, setDomains } = useContext(AppContext);
  return [domains, setDomains] as [string[], (domain: string[]) => void];
}


function filterRequest(requests: RequestRecord[], filters: FiltersValue) {
  const baseFilter = (req: RequestRecord) => req.req.uri.includes(filters.search);

  const categoryConditions: Record<Category, (req: RequestRecord) => boolean> = {
    [Category.ALL]: baseFilter,
    [Category.XHR]: (req) =>
      baseFilter(req) && req.req.headers["x-requested-with"] === "XMLHttpRequest",
    [Category.DOC]: (req) =>
      baseFilter(req) && req.req.headers["content-type"]?.includes("text/html"),
    [Category.CSS]: (req) =>
      baseFilter(req) && req.req.headers["content-type"]?.includes("text/css"),
    [Category.JS]: (req) =>
      baseFilter(req) && req.req.headers["content-type"]?.includes("application/javascript"),
    [Category.MEDIA]: (req) =>
      baseFilter(req) &&
      ["image/jpeg", "image/png", "image/gif", "video/mp4", "audio/mpeg"].some((type) =>
        req.req.headers["content-type"]?.includes(type)
      ),
    [Category.WEBSOCKET]: (req) => baseFilter(req) && req.req.protocol === "websocket",
  };

  return requests.filter(categoryConditions[filters.category as Category] || baseFilter);
}
