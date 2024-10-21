import { Table } from "antd";

type DataType = {
  key: number;
  url: string;
  method: string;
  statusCode: number;
  status: string;
};

export default function UrlList(props: { dataSource: DataType[] }) {
  const columns = [
    {
      title: "URL",
      dataIndex: "url",
      key: "url",
    },
    {
      title: "Method",
      dataIndex: "method",
      key: "method",
    },
    {
      title: "StatusCode",
      dataIndex: "statusCode",
      key: "statusCode",
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
    },
  ];
  return <Table<DataType> rowKey={(record) => record.key} columns={columns} dataSource={props.dataSource} />;
}
