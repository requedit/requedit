import { useTreeData } from "@/provides/AppContext";
import { Tree } from "antd";

export default function FolderTree() {
  const tree = useTreeData();
  const onSelect = () => {

  }
  const onShowDomainContextMenu = () => {
    console.log("onShowDomainContextMenu");
  }
  return (
    <div className="p-2">
      <Tree
        autoExpandParent
        onSelect={onSelect}
        treeData={tree}
        onRightClick={onShowDomainContextMenu}
      />
    </div>
  );
}
