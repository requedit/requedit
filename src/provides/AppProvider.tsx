import { useRef, useState } from "react";
import type { TreeDataNode } from "antd";
import _ from "lodash";
import { listen } from "@tauri-apps/api/event";
import { RequestRecord, AppContext, ThemeType } from "./AppContext";
import { invoke } from "@tauri-apps/api/core";
import { useMount, useUnmount } from "ahooks";

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [theme, setTheme] = useState<ThemeType>("auto");
  const [domains, setDomains] = useState<string[]>([]);
  const [requests, setRequests] = useState<RequestRecord[]>([]);
  const unListenFnRef = useRef<() => void>();
  const [treeData, setTreeData] = useState<TreeDataNode[]>([
    {
      title: "全部",
      key: "all",
      children: [],
    },
  ]);
  useMount(async () => {
    // get system theme
    const theme = await invoke<ThemeType>("plugin:theme|get_theme");
    setTheme(theme);
  });


  useMount(async () => {
    unListenFnRef.current =  await listen<any>("proxy-event", (event) => {
      console.log("Received event from Rust:", event.payload);
      setRequests((preRequests) => {
        const reqs = [event.payload, ...preRequests]
          .filter((item) => item)
          .map((item, index) => ({
            ...item,
            key: index,
          }));
        return _.unionBy(reqs, "id");
      });
      const newTree = updateTreeData(event.payload, treeData);
      setTreeData(newTree);
    });
  });

  useUnmount(() => {
    unListenFnRef.current?.();
  })

  const toggleTheme = async () => {
    const _theme = theme === "light" ? "dark" : "light";
    invoke("plugin:theme|set_theme", {
      theme: _theme,
    });
    setTheme(_theme);
    const doc = document.documentElement;
    if (_theme === "dark") {
      doc.classList.add("dark");
    } else {
      doc.classList.remove("dark");
    }
  };
  return (
    <AppContext.Provider
      value={{
        theme,
        domains,
        requests,
        treeData,
        setRequests,
        setTreeData,
        toggleTheme,
        setDomains,
      }}
    >
      <div className="bg-white text-gray-900 dark:bg-[#141414] dark:text-white">
        {children}
      </div>
    </AppContext.Provider>
  );
};

function updateTreeData(request: RequestRecord, treeData: TreeDataNode[]) {
  const domains = _.get(treeData, "[0].children");
  if (
    request.req.host &&
    !domains!.find((child) => child.title == request.req.host)
  ) {
    domains?.push({
      title: request.req.host,
      key: request.req.host,
      isLeaf: true,
    });
  }
  _.set(treeData, "[0].children", domains);
  return [...treeData];
}
