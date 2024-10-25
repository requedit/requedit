import Filters from "@/components/filters/index";
import useFilters from "@/components/filters/useFilters";
import FolderTree from "@/components/folder-tree";
import Toolbar from "@/components/toolbar";
import UrlList from "@/components/url-list";
import useHeight from "@/hooks/useHeight";
import { useRequests } from "@/provides/AppContext";
import { Divider } from "antd";
import SplitPane from "react-split-pane";

export default function ListPage() {
  const { filters, handleFiltersChange } = useFilters();
  const requests = useRequests(filters);
  const { height, ref } = useHeight();
  return (
    <div className="h-full">
      <div className="p-2" ref={ref}>
        <Toolbar />
        <Divider />
        <Filters value={filters} onChange={handleFiltersChange} />
      </div>

      <div className="border-t border-solid border-gray-200 dark:border-gray-600 h-max">
        {/* @ts-ignore */}
        <SplitPane
          split="vertical"
          minSize={100}
          maxSize={400}
          defaultSize={280}
          resizerClassName="w-[11px] mx-0 -mx-[5px] border-l-[5px] border-r-[5px] border-l-transparent border-r-transparent cursor-col-resize bg-gray-200 dark:bg-gray-600 z-1 box-border bg-clip-padding"
          style={{
            position: "relative",
            height: `calc(100vh - ${height + 1}px)`,
          }}
        >
          <FolderTree />
          <UrlList dataSource={requests} />
        </SplitPane>
      </div>
    </div>
  );
}
