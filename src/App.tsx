import { Outlet } from "react-router-dom";
import { WebsocketProvider } from "@/provides/websocket/WebsocketProvider";
import { ConfigProvider } from "antd";

function App() {
  return (
    <ConfigProvider
      componentSize="small"
      theme={{
        components: {
          Input: {
            paddingBlockLG: 2,
            inputFontSizeSM: 12
          },
          Tag: {
            fontSize: 12,
            lineHeightSM: 1.2,
            marginXS: 2
          },
          Divider: {
            marginLG: 10
          },
        },
      }}
    >
      <WebsocketProvider>
        {/* <div>
          <Link to="/login">login</Link>
        </div> */}
        <Outlet />
      </WebsocketProvider>
    </ConfigProvider>
  );
}

export default App;
