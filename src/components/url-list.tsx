import { Table } from "antd";
import { useContextMenu } from 'mantine-contextmenu';

type DataType = {
  key: number;
  url: string;
  method: string;
  statusCode: number;
  status: string;
};

export default function UrlList(props: { dataSource: DataType[] }) {
  console.log(props.dataSource);
  const { showContextMenu } = useContextMenu();
  const columns = [
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
    },
    {
      title: "客户端",
      dataIndex: "client",
      key: "client",
      ellipsis: true,
    },
    {
      title: "Method",
      dataIndex: "method",
      key: "method",
      ellipsis: true,
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
      ellipsis: true,
    },
    {
      title: "StatusCode",
      dataIndex: "statusCode",
      key: "statusCode",
      ellipsis: true,
    },
    {
      title: 'Time',
      dataIndex: 'time',
      key: 'time',
      ellipsis: true,
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
      ellipsis: true,
    }
  ];
  return (
    <div>
      <Table<DataType>
      rowKey={(record) => record.key}
      columns={columns}
      pagination={false}
      dataSource={props.dataSource}
      scroll={{x: 'max-content'}}
      onRow={(record) => {
        return {
          onContextMenu: showContextMenu([
            {
              key: 'copy',
              // icon: <IconCopy size={16} />,
              title: '拷贝网址',
              onClick: () => console.log,
            },
            {
              key: 'copy_curl',
              // icon: <IconDownload size={16} />,
              title: '拷贝 cURL',
              onClick: () => console.log,
            },
          ]),
        };
      }}
    />
    </div>
  );
}
