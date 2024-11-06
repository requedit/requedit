import SplitPane from "react-split-pane";
import UrlList from "./url-list";
import Detail from "./detail";
import useHeight from "@/hooks/useHeight";
import { useRef, useState } from "react";
import { RequestRecord } from "@/provides/AppContext";

export default function MainPanel(props: any) {
  const ref = useRef()
  const [record, setRecord] = useState<RequestRecord>()
  return (
    /* @ts-ignore */
    <SplitPane
      split="horizontal"
      minSize={0}
      maxSize={'100%'}
      defaultSize={280}
      className="z-50"
      resizerClassName="h-[11px] my-0 -my-[5px] border-t-[5px] border-b-[5px] border-t-transparent border-b-transparent cursor-row-resize bg-gray-200 dark:bg-gray-600 z-1 box-border bg-clip-padding"
      pane1Style={{
        margin: '-5px 0'
      }}
    >
      <UrlList {...props} selectedId={record?.id} onSelect={setRecord} />
      <Detail ref={ref} record={record} />
    </SplitPane>
  );
}
