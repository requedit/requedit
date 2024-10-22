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

      <div className="border-t border-solid border-gray-200 h-max">
        {/* @ts-ignore */}
        <SplitPane
          split="vertical"
          minSize={100}
          defaultSize={280}
          resizerStyle={{
            background: "#ccc",
            width: "1px",
            cursor: "col-resize",
          }}
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
