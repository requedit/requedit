import { useClearFn } from "@/provides/websocket/Websocket";
import {
  PauseOutlined,
  CaretRightOutlined,
  DeleteOutlined,
  SendOutlined,
} from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Button, Space } from "antd";
import { useState } from "react";
export default function Toolbar() {
  const [start, setStart] = useState(false);
  const clearFn = useClearFn()
  const onStartProxy = async () => {
    setStart(!start)
    await invoke(!start ? "start_proxy" : "stop_proxy");
  };

  const onSend = async () => {
    fetch("http://clientservices.googleapis.com");
    // fetch("http://clientservices.googleapis.com/chrome-variations/seed?osname=mac&channel=stable&milestone=128")
    // fetch("https://www.google.com")
  };
  return (
    <div>
      <Space>
        <Button onClick={onStartProxy} type="text">
          {start ? <PauseOutlined /> : <CaretRightOutlined />}
        </Button>
        <Button type="text" onClick={clearFn}>
          <DeleteOutlined />
        </Button>
        <Button onClick={onSend} type="text"><SendOutlined /></Button>
      </Space>
    </div>
  );
}
