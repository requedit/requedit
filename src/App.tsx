import { Outlet } from "react-router-dom";
import { AppProvider } from "@/provides/AppProvider";
import { ConfigProvider, theme } from "antd";
import { useTheme } from "./provides/AppContext";
import { MantineProvider } from "@mantine/core";
import { ContextMenuProvider } from 'mantine-contextmenu';

const { darkAlgorithm, defaultAlgorithm } = theme;

function App() {
  const theme = useTheme();
  console.log(theme);
  return (
    <ConfigProvider
      componentSize="small"
      theme={{
        algorithm: theme === "dark" ? darkAlgorithm : defaultAlgorithm,
        token: {
          fontSize: 14,
          marginLG: 8,
          margin: 4,
          borderRadiusSM: 50,
          lineHeight: 1.2,
          lineHeightSM: 1.4
        },
        components: {
          Table: {
          },

        }
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
