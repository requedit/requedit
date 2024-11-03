import { Badge, Table, message } from "antd";
import { useContextMenu } from "mantine-contextmenu";
import { buildCurlCommand } from "@/utils";
import { RequestRecord } from "@/provides/AppContext";
import { useState } from "react";
import type { TableColumnsType } from "antd";
import type { ResizeCallbackData } from "react-resizable";
import { Resizable } from "react-resizable";
import dayjs from "dayjs";
import "./url-list.less";

interface TitlePropsType {
  width: number;
  onResize: (
    e: React.SyntheticEvent<Element>,
    data: ResizeCallbackData
  ) => void;
}
const ResizableTitle: React.FC<
  Readonly<React.HTMLAttributes<any> & TitlePropsType>
> = (props) => {
  const { onResize, width, ...restProps } = props;

  if (!width) {
    return <th {...restProps} />;
  }

  return (
    <Resizable
      width={width}
      height={0}
      handle={
        <span
          className="react-resizable-handle"
          onClick={(e) => e.stopPropagation()}
        />
      }
      onResize={onResize}
      draggableOpts={{ enableUserSelectHack: false }}
    >
      <th {...restProps} />
    </Resizable>
  );
};

export default function UrlList(props: {
  dataSource: RequestRecord[];
  selectedId?: number;
  onSelect: (data: RequestRecord) => void;
}) {
  const { showContextMenu } = useContextMenu();
  const [columns, setColumns] = useState<TableColumnsType<RequestRecord>>([
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      ellipsis: true,
      width: 80,
    },
    {
      title: "URL",
      dataIndex: "uri",
      key: "uri",
      ellipsis: true,
      width: 240,
      render: (text, record, index) => {
        return <span>{record.req.uri}</span>;
      },
    },
    {
      title: "Method",
      dataIndex: "method",
      key: "method",
      width: 100,
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
      width: 100,
      render: (text, record, index) => {
        if (!record.res) {
          return <Badge status="processing" color={"gold"} text="pending" />;
        }
        return <Badge color={"green"} text={record.res.status} />;
      },
    },

    {
      title: "Time",
      dataIndex: "time",
      key: "time",
      ellipsis: true,
      render: (text, record, index) => {
        return <span>{dayjs(record.req.date).format("HH:mm:ss.SSS")}</span>;
      },
    },
  ]);

  const handleResize =
    (index: number) =>
    (_: React.SyntheticEvent<Element>, { size }: ResizeCallbackData) => {
      const newColumns = [...columns];
      newColumns[index] = {
        ...newColumns[index],
        width: size.width,
      };
      setColumns(newColumns);
    };

  const mergedColumns = columns.map<TableColumnsType<RequestRecord>[number]>(
    (col, index) => ({
      ...col,
      onHeaderCell: (column: TableColumnsType<RequestRecord>[number]) => ({
        width: column.width,
        onResize: handleResize(index) as React.ReactEventHandler<any>,
      }),
    })
  );

  const onCopy = (record: RequestRecord) => {
    navigator.clipboard.writeText(record.req.uri);
    message.success("已复制");
  };
  const onCopyToCurl = (record: RequestRecord) => {
    const curl = buildCurlCommand(
      record.req.uri,
      record.req.method,
      record.req.headers,
      record.req.body
    );
    navigator.clipboard.writeText(curl);
    message.success("已复制");
  };
  return (
    <div className="overflow-y-auto url-list">
      <Table<RequestRecord>
        sticky={true}
        rowKey={"id"}
        components={{ header: { cell: ResizableTitle } }}
        columns={mergedColumns}
        pagination={false}
        dataSource={props.dataSource}
        rowHoverable={false}
        onRow={(record) => {
          return {
            className:
              record.id == props.selectedId ? "bg-blue-600 text-white" : "",
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
