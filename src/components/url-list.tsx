import { Badge, Table } from "antd";
import { useContextMenu } from "mantine-contextmenu";
import { ColumnType } from "antd/es/table";
import { getStatusColor } from "@/utils";

type DataType = {
  key: number;
  url: string;
  method: string;
  statusCode: number;
  status: string;
};

const isSuccess = (res: any) => {
  return;
};

export default function UrlList(props: {
  dataSource: DataType[];
  onSelect: (data: DataType) => void;
}) {
  console.log(props.dataSource);
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
    // {
    //   title: "客户端",
    //   dataIndex: "client",
    //   key: "client",
    //   ellipsis: true,
    // },
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
        return (
          <Badge
            color={getStatusColor(record.res)}
            text={record.res ? record.res.status : "pending"}
          />
        );
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

  return (
    <div className="overflow-y-auto">
      <Table<DataType>
        sticky={true}
        rowKey={(record) => record.key}
        columns={columns}
        pagination={false}
        dataSource={props.dataSource}
        // scroll={{y: '300px'}}
        onRow={(record) => {
          return {
            onContextMenu: showContextMenu([
              {
                key: "copy",
                // icon: <IconCopy size={16} />,
                title: "拷贝网址",
                onClick: () => console.log,
              },
              {
                key: "copy_curl",
                // icon: <IconDownload size={16} />,
                title: "拷贝 cURL",
                onClick: () => console.log,
              },
            ]),
            onClick: () => props.onSelect(record),
          };
        }}
      />
    </div>
  );
}
