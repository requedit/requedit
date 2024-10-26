import SplitPane from "react-split-pane";
import ReqPanel from "./req-panel";
import ResPanel from "./res-panel";
import { forwardRef, useImperativeHandle, useRef } from "react";
import useHeight from "@/hooks/useHeight";
import { Divider, Space, Tag } from "antd";

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
    return null;
  }
  return (
    <div ref={detailRef}>
      <div>
        <Space>
          <Tag color="gray">{record.req.method}</Tag>
          <Tag color="green">{record.res ? record.res.status : "pending"}</Tag>
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
      >
        <ReqPanel request={record.req} />
        <ResPanel response={record.res} />
      </SplitPane>
    </div>
  );
});
