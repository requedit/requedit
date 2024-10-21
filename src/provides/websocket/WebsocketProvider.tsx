import { useEffect, useState } from "react";
import type { TreeDataNode } from "antd";
import _ from "lodash";
import { listen } from '@tauri-apps/api/event';
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
    // 监听 `custom-event` 事件
    listen("proxy-event", (event) => {
      console.log("Received event from Rust:", event.payload);
    });
    // const socket = new WebSocket("ws://localhost:9001");
    // setSocket(socket);

    // socket.onmessage = (event) => {
    //   const message = JSON.parse(event.data) as Message;
    //   setMessages((prevMessages) =>
    //     [...prevMessages, message].map((item, index) => ({
    //       ...item,
    //       key: index,
    //     }))
    //   );
    //   const newTree = updateTreeData(message, treeData);
    //   setTreeData(newTree);
    // };
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
