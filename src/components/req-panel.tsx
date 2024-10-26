import { Space, Tabs } from "antd"

export default function ReqPanel(props: {
  request: Record<string, any>
}) {
  const tabs = [
    {
      label: "Header",
      key: "header",
      children: <pre className="text-wrap overflow-auto">{JSON.stringify(props.request.headers, null, 2)}</pre>
    },
    {
      label: "Query",
      key: "query",
      children: <pre>{props.request.query}</pre>
    },
    {
      label: "Body",
      key: "body",
      children: <pre>{JSON.stringify(props.request.body, null, 2)}</pre>
    },
    {
      label: "Cookies",
      key: "cookies",
      children: <pre>敬请期待</pre>
    },
    {
      label: "Authentication",
      key: "authentication",
      children: <pre>敬请期待</pre>
    },
    {
      label: "Raw",
      key: "raw",
      children: <pre>{JSON.stringify(props.request, null, 2)}</pre>
    },
    {
      label: "Code",
      key: "code",
      children: <pre>敬请期待</pre>
    }
  ]
  return <div className="text-wrap">
      <Tabs items={tabs} />
  </div>
}
