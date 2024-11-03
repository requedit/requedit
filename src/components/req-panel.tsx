import { Tabs, TabsProps, theme } from "antd";
import StickyBox from "react-sticky-box";
import Body from "./panels/body";

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
      children: <Body body={props.request.body} />,
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
        className="overflow-auto"
        style={{
          height: "calc(100% - 40px)",
        }}
      />
    </div>
  );
}
