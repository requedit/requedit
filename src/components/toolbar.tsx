import { useClearFn, useTheme, useToggleTheme } from "@/provides/AppContext";
import { invoke } from "@tauri-apps/api/core";
import { Button, Divider, Flex, Space, Tooltip, Typography } from "antd";
import { useRef, useState } from "react";
import ThemeSwitcher from "./theme-switch";
import { useMount, useUnmount } from "ahooks";
import { listen } from "@tauri-apps/api/event";
import { RequeditEvent } from "@/constants/event";
import {
  DeleteOutlined,
  EditOutlined,
  PauseOutlined,
  PlayCircleOutlined,
} from "@ant-design/icons";

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
    getStatus();
  };

  useMount(async () => {
    getStatus();
    const config = await invoke<ProxyConfig>("get_config");
    setConfig(config);
  });

  useMount(async () => {
    unListenFnRef.current = await listen<any>(
      RequeditEvent.ProxyStatus,
      (event) => {
        console.log("Received status from Rust:", event.payload);
        setStart(event.payload);
      }
    );
  });

  useUnmount(() => {
    unListenFnRef.current?.();
  });

  const getStatus = async () => {
    const status = await invoke<boolean>("get_proxy_status");
    setStart(status);
  };

  const onEdit = () => {};

  return (
    <div>
      <Flex justify="space-between">
        <Space style={{ gap: 0 }}>
          <Tooltip title={start ? "Stop proxy" : "Start proxy"}>
            <Button onClick={onStartProxy} type="text">
              {start ? (
                <PauseOutlined style={{ fontSize: 14 }} />
              ) : (
                <PlayCircleOutlined style={{ fontSize: 14 }} />
              )}
            </Button>
          </Tooltip>

          <Tooltip title="Clear data">
            <Button type="text" onClick={clearFn}>
              <DeleteOutlined style={{ fontSize: 14 }} />
            </Button>
          </Tooltip>

          <Button type="text" onClick={onEdit} target="_blank" disabled>
            <EditOutlined />
          </Button>
        </Space>
        <div className="proxy-info">
          <div className="flex border border-gray-30 pl-2 rounded-md w-[400px] h-[26px]">
            <Space
              split={<Divider type="vertical" style={{ marginInline: 0 }} />}
              className="text-xs"
            >
              <Typography.Text type="secondary">
                {config?.address}
              </Typography.Text>
              <Typography.Text type="secondary">{config?.port}</Typography.Text>
              {start ? (
                ""
              ) : (
                <Typography.Text type="secondary">暂停</Typography.Text>
              )}
            </Space>
          </div>
        </div>
        <ThemeSwitcher theme={theme} toggleTheme={toggleTheme} />
      </Flex>
    </div>
  );
}
