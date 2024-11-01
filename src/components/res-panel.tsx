import { Tabs, TabsProps, theme } from "antd";
import StickyBox from "react-sticky-box";

export default function ResPanel(props: { response: Record<string, any> }) {
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  if (!props.response) return null;
  const tabs = [
    {
      label: "Header",
      key: "header",
      children: (
        <pre className="text-wrap overflow-auto">
          {JSON.stringify(props.response.headers, null, 2)}
        </pre>
      ),
    },
    {
      label: "Body",
      key: "body",
      children: (
        <pre className="text-wrap overflow-auto">
          {JSON.stringify(props.response.body, null, 2)}
        </pre>
      ),
    },
    {
      label: "Row",
      key: "row",
      children: (
        <pre className="text-wrap overflow-auto">
          {JSON.stringify(props.response, null, 2)}
        </pre>
      ),
    },
  ];
  const renderTabBar: TabsProps["renderTabBar"] = (props, DefaultTabBar) => (
    <StickyBox offsetTop={0} offsetBottom={20} style={{ zIndex: 1 }}>
      <DefaultTabBar {...props} style={{ background: colorBgContainer }} />
    </StickyBox>
  );
  return (
    <div className="h-full">
      <Tabs
        items={tabs}
        renderTabBar={renderTabBar}
        className="overflow-auto"
        style={{
          height: "calc(100% - 40px)",
        }}
      />
    </div>
  );
}
