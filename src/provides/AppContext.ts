import { createContext, useContext } from "react";
import type { TreeDataNode } from 'antd';
import { FiltersValue } from "@/components/filters/types";
import  { HttpStatusCode } from "axios";

enum RequestStatus {
  success = 'success',
  error = 'error',
  pending = 'pending',
}
export type Request = {
  key: number;
  url: string;
  method: string;
  statusCode: HttpStatusCode;
  status: RequestStatus;
  host: string;
  category: string;
}
export type ThemeType = 'auto' | 'light' | 'dark'
type AppContextType = {
  theme: ThemeType;
  requests: Request[];
  treeData: TreeDataNode[]
  setRequests: (requests: Request[]) => void;
  setTreeData: (treeData: TreeDataNode[]) => void;
  toggleTheme: () => void;
};

export const AppContext = createContext<AppContextType>({
  theme: 'auto',
  requests: [],
  treeData: [],
  setRequests: (_message: Request[]) => { },
  setTreeData: (_treeData: TreeDataNode[]) => { },
  toggleTheme: () => { },
})

export const useRequests = (filters: FiltersValue) => {
  const { requests } = useContext(AppContext);
  return requests
  // return messages.filter(m => m.url.includes(filters.search) && (filters.category == 'All' || m.category == filters.category))
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
