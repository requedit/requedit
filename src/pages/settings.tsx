import { Anchor, Layout } from "antd";
import { settingItems, Settings } from "@/components/settings";
const { Sider, Content } = Layout;

export default function SettingsPage() {
  return (
    <Layout>
      <Sider
        style={{
          background: "#fff",
        }}
      >
        <Anchor items={settingItems} />
      </Sider>
      <Content>
        <Settings />
      </Content>
    </Layout>
  );
}
