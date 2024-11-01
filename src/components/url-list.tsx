import { Badge, Table, message } from "antd";
import { useContextMenu } from "mantine-contextmenu";
import { ColumnType } from "antd/es/table";
import { buildCurlCommand, getStatusColor } from "@/utils";
import { RequestRecord } from "@/provides/AppContext";


export default function UrlList(props: {
  dataSource: RequestRecord[];
  selectedId?: number;
  onSelect: (data: RequestRecord) => void;
}) {

  const { showContextMenu } = useContextMenu();

  const columns: ColumnType<any>[] = [
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      ellipsis: true,
    },
    {
      title: "URL",
      dataIndex: "uri",
      key: "uri",
      ellipsis: true,
      render: (text, record, index) => {
        return <span>{record.req.uri}</span>;
      },
    },
    {
      title: "Method",
      dataIndex: "method",
      key: "method",
      ellipsis: true,
      render: (text, record, index) => {
        return <span>{record.req.method}</span>;
      },
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
      ellipsis: true,
      render: (text, record, index) => {
        return <span>{record.res ? record.res.status : "pending"}</span>
      },
    },

    {
      title: "Time",
      dataIndex: "time",
      key: "time",
      ellipsis: true,
    },
    {
      title: "Version",
      dataIndex: "version",
      key: "version",
      ellipsis: true,
    },
  ];
  const onCopy = (record: RequestRecord) => {
    navigator.clipboard.writeText(record.req.uri);
    message.success("已复制");
  }
  const onCopyToCurl = (record: RequestRecord) => {
    const curl = buildCurlCommand(record.req.uri, record.req.method, record.req.headers, record.req.body);
    navigator.clipboard.writeText(curl);
    message.success("已复制");
  }
  return (
    <div className="overflow-y-auto">
      <Table<RequestRecord>
        sticky={true}
        rowKey={'id'}
        columns={columns}
        pagination={false}
        dataSource={props.dataSource}
        rowHoverable={false}
        onRow={(record) => {
          return {
            className: record.id == props.selectedId ? "bg-blue-600 text-white" : "",
            onContextMenu: showContextMenu([
              {
                key: "copy",
                title: "拷贝网址",
                onClick: () => onCopy(record),
              },
              {
                key: "copy_curl",
                title: "拷贝 cURL",
                onClick: () => onCopyToCurl(record),
              },
            ]),
            onClick: () => props.onSelect(record),
          };
        }}
      />
    </div>
  );
}
