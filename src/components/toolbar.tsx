import { useClearFn, useTheme, useToggleTheme } from "@/provides/AppContext";
import {
  PauseOutlined,
  CaretRightOutlined,
  DeleteOutlined,
} from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Button, Flex, Space } from "antd";
import { useState } from "react";
import ThemeSwitcher from "./theme-switch";
export default function Toolbar() {
  const theme = useTheme();
  const toggleTheme = useToggleTheme();
  const [start, setStart] = useState(false);
  const clearFn = useClearFn();
  const onStartProxy = async () => {
    setStart(!start);
    await invoke(!start ? "set_sys_proxy" : "clean_sys_proxy");
  };

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
        <ThemeSwitcher theme={theme} toggleTheme={toggleTheme} />
      </Flex>
    </div>
  );
}
