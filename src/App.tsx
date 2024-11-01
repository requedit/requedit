import { Outlet } from "react-router-dom";
import { ConfigProvider, theme } from "antd";
import { useTheme } from "./provides/AppContext";
import { MantineProvider } from "@mantine/core";
import { ContextMenuProvider } from "mantine-contextmenu";

const { darkAlgorithm, defaultAlgorithm } = theme;

function App() {
  const theme = useTheme();
  return (
    <ConfigProvider
      componentSize="small"
      theme={{
        algorithm: theme === "dark" ? darkAlgorithm : defaultAlgorithm,
        token: {
          fontSize: 12,
          marginLG: 8,
          margin: 4,
          borderRadiusSM: 50,
          lineHeight: 1.2,
          lineHeightSM: 1.4,
        },
      }}
    >
      <MantineProvider>
        <ContextMenuProvider>
          <Outlet />
        </ContextMenuProvider>
      </MantineProvider>
    </ConfigProvider>
  );
}

export default App;
