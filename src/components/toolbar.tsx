import { useClearFn, useTheme, useToggleTheme } from "@/provides/AppContext";
import {
  PauseOutlined,
  CaretRightOutlined,
  DeleteOutlined,
} from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Button, Divider, Flex, Space, Typography } from "antd";
import { useRef, useState } from "react";
import ThemeSwitcher from "./theme-switch";
import { useMount, useUnmount } from "ahooks";
import { listen } from "@tauri-apps/api/event";
import { RequeditEvent } from "@/constants/event";

type ProxyConfig = {
  address: string;
  port: number;
};

export default function Toolbar() {
  const theme = useTheme();
  const toggleTheme = useToggleTheme();
  const [start, setStart] = useState(false);
  const [config, setConfig] = useState<ProxyConfig>();
  const unListenFnRef = useRef<() => void>();
  const clearFn = useClearFn();
  const onStartProxy = async () => {
    await invoke(!start ? "set_sys_proxy" : "clean_sys_proxy");
    getStatus()
  };

  useMount(async () => {
    getStatus()
    const config = await invoke<ProxyConfig>("get_config");
    setConfig(config);
  });

  useMount(async () => {
    unListenFnRef.current =  await listen<any>(RequeditEvent.ProxyStatus, (event) => {
      console.log("Received status from Rust:", event.payload);
      setStart(event.payload);
    });
  });

  useUnmount(() => {
    unListenFnRef.current?.();
  })

  const getStatus = async () => {
    const status = await invoke<boolean>("get_proxy_status");
    setStart(status);
  }

  return (
    <div>
      <Flex justify="space-between">
        <Space>
          <Button onClick={onStartProxy} type="text">
            {start ? <PauseOutlined /> : <CaretRightOutlined />}
          </Button>
          <Button type="text" onClick={clearFn}>
            <DeleteOutlined />
          </Button>
        </Space>
        <div className="proxy-info">
          <div className="flex border border-gray-30 pl-2 rounded-md w-[400px] h-[26px]">
            <Space split={<Divider type="vertical" style={{ marginInline: 0 }} />} className="text-xs">
              <Typography.Text type="secondary">{config?.address}</Typography.Text>
              <Typography.Text type="secondary">{config?.port}</Typography.Text>
              {start ? "" : <Typography.Text type="secondary">暂停</Typography.Text>}
            </Space>
          </div>
        </div>
        <ThemeSwitcher theme={theme} toggleTheme={toggleTheme} />
      </Flex>
    </div>
  );
}
