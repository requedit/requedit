import { Tabs, TabsProps, theme } from "antd";
import StickyBox from "react-sticky-box";

export default function ReqPanel(props: { request: Record<string, any> }) {
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  const tabs = [
    {
      label: "Header",
      key: "header",
      children: (
        <pre className="text-wrap overflow-auto">
          {JSON.stringify(props.request.headers, null, 2)}
        </pre>
      ),
    },
    {
      label: "Query",
      key: "query",
      children: <pre>{props.request.query}</pre>,
    },
    {
      label: "Body",
      key: "body",
      children: <pre>{JSON.stringify(props.request.body, null, 2)}</pre>,
    },
    {
      label: "Cookies",
      key: "cookies",
      children: <pre>敬请期待</pre>,
    },
    {
      label: "Authentication",
      key: "authentication",
      children: <pre>敬请期待</pre>,
    },
    {
      label: "Raw",
      key: "raw",
      children: <pre>{JSON.stringify(props.request, null, 2)}</pre>,
    },
    {
      label: "Code",
      key: "code",
      children: <pre>敬请期待</pre>,
    },
  ];
  const renderTabBar: TabsProps["renderTabBar"] = (props, DefaultTabBar) => (
    <StickyBox offsetTop={0} offsetBottom={20} style={{ zIndex: 10 }}>
      <DefaultTabBar {...props} style={{ background: colorBgContainer }} />
    </StickyBox>
  );

  return (
    <div className="h-full">
      <Tabs
        items={tabs}
        renderTabBar={renderTabBar}
        className="h-full overflow-auto"
      />
    </div>
  );
}
