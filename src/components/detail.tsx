import SplitPane from "react-split-pane";
import ReqPanel from "./req-panel";
import ResPanel from "./res-panel";
import { forwardRef, useImperativeHandle, useRef } from "react";
import useHeight from "@/hooks/useHeight";
import { Divider, Empty, Space, Tag } from "antd";
import { parseHttpStatus, getStatusColor } from "@/utils";
import { MethodColor } from "@/constants/method";

export default forwardRef(function Detail(
  props: {
    record?: Record<string, any>;
  },
  ref: any
) {
  const { height, ref: detailRef } = useHeight();
  useImperativeHandle(ref, () => {
    return {
      height: height,
    };
  });

  const { record } = props;
  if (!record) {
    return <Empty image={Empty.PRESENTED_IMAGE_SIMPLE} />;
  }

  return (
    <div ref={detailRef}>
      <div>
        <Space>
          <Tag color={MethodColor[record.req.method]}>{record.req.method}</Tag>
          <Tag color={getStatusColor(record.res)}>
            {parseHttpStatus(record.res) || "pending"}
          </Tag>
          <span className="text-xs">{record.req.uri}</span>
        </Space>
      </div>
      <Divider />
      {/* @ts-ignore */}
      <SplitPane
        split="vertical"
        minSize={300}
        defaultSize={"50%"}
        resizerClassName="w-[11px] mx-0 -mx-[5px] border-l-[5px] border-r-[5px] border-l-transparent border-r-transparent cursor-col-resize bg-gray-200 dark:bg-gray-600 z-1 box-border bg-clip-padding"
        pane1Style={{
          height: "100%",
        }}
        pane2Style={{
          height: "100%",
        }}
      >
        <ReqPanel request={record.req} />
        <ResPanel response={record.res} />
      </SplitPane>
    </div>
  );
});
