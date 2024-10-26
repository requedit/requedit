import { Tabs } from "antd"


export default function ResPanel(props: {
  response: Record<string, any>
}) {
  if (!props.response) return null
  const tabs = [
    {
      label: "Header",
      key: "header",
      children: <pre className="text-wrap overflow-auto">{JSON.stringify(props.response.headers, null, 2)}</pre>
    },
    {
      label: "Body",
      key: "body",
      children: <pre className="text-wrap overflow-auto">{JSON.stringify(props.response.body, null, 2)}</pre>
    },
    {
      label: 'Row',
      key: 'row',
      children: <pre className="text-wrap overflow-auto">{JSON.stringify(props.response, null, 2)}</pre>
    }
  ]
  return <div className="text-wrap">
  <Tabs items={tabs} />
</div>
}
