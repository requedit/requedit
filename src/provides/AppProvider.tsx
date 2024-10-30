import { useEffect, useState } from "react";
import type { TreeDataNode } from "antd";
import _ from "lodash";
import { listen } from "@tauri-apps/api/event";
import { Request, AppContext, ThemeType } from "./AppContext";
import { invoke } from "@tauri-apps/api/core";
import { useMount } from "ahooks";
import { parseHttpStatus } from "@/utils/parse-code";

export const AppProvider = ({ children }: { children: React.ReactNode }) => {
  const [theme, setTheme] = useState<ThemeType>("auto");
  const [requests, setRequests] = useState<Request[]>([]);
  const [treeData, setTreeData] = useState<TreeDataNode[]>([
    {
      title: "全部",
      key: "all",
      children: [{ title: "域名", key: "domain", children: [] }],
    },
  ]);
  useMount(() => {
    invoke("plugin:theme|set_theme", {
      theme: theme,
    });
  });

  useEffect(() => {
    (async function () {
      await listen<any>("proxy-event", (event) => {
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
    })();
  }, []);

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
        requests,
        treeData,
        setRequests,
        setTreeData,
        toggleTheme,
      }}
    >
      <div className="bg-white text-gray-900 dark:bg-[#141414] dark:text-white">
        {children}
      </div>
    </AppContext.Provider>
  );
};

function updateTreeData(request: Request, treeData: TreeDataNode[]) {
  const domains = _.get(treeData, "[0].children[0].children");
  if (request.host && !domains!.find((child) => child.title == request.host)) {
    domains?.push({
      title: request.host,
      key: request.host,
      isLeaf: true,
    });
  }
  _.set(treeData, "[0].children[0].children", domains);
  return [...treeData];
}
