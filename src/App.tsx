import { Outlet } from "react-router-dom";
import { AppProvider } from "@/provides/AppProvider";
import { ConfigProvider, theme } from "antd";
import { useTheme } from "./provides/AppContext";

const { darkAlgorithm, defaultAlgorithm } = theme;

function App() {
  const theme = useTheme();
  console.log(theme);
  return (
    <ConfigProvider
      componentSize="small"
      theme={{
        algorithm: theme === "dark" ? darkAlgorithm : defaultAlgorithm,
        components: {
          Input: {
            paddingBlockLG: 2,
            inputFontSizeSM: 12,
          },
          Tag: {
            fontSize: 12,
            lineHeightSM: 1.2,
            marginXS: 2,
          },
          Divider: {
            marginLG: 10,
          },
        },
      }}
    >
      <Outlet />
    </ConfigProvider>
  );
}

export default App;
