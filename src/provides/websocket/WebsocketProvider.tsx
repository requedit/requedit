import { useEffect, useState } from "react";
import type { TreeDataNode } from "antd";
import _ from "lodash";
import { listen } from "@tauri-apps/api/event";
import { Message, WebsocketContext } from "./Websocket";

export const WebsocketProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  // const [socket, setSocket] = useState<WebSocket | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [treeData, setTreeData] = useState<TreeDataNode[]>([
    {
      title: "全部",
      key: "all",
      children: [{ title: "域名", key: "domain", children: [] }],
    },
  ]);

  useEffect(() => {
    (async function () {
      await listen<any>("proxy-event", (event) => {
        console.log("Received event from Rust:", event.payload);
        setMessages((prevMessages) =>
          [...prevMessages, event.payload].map((item, index) => ({
            ...item,
            key: index,
          }))
        );
        const newTree = updateTreeData(event.payload, treeData);
        setTreeData(newTree);
      });
    })()
  }, []);
  return (
    <WebsocketContext.Provider
      value={{ messages, treeData, setMessages, setTreeData }}
    >
      {children}
    </WebsocketContext.Provider>
  );
};

function updateTreeData(message: Message, treeData: TreeDataNode[]) {
  const domains = _.get(treeData, "[0].children[0].children");
  if (message.host && !domains!.find((child) => child.title == message.host)) {
    domains?.push({
      title: message.host,
      key: message.host,
      isLeaf: true,
    });
  }
  _.set(treeData, "[0].children[0].children", domains);
  return [...treeData];
}
