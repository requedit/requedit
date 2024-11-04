import { useDomain, useTreeData } from "@/provides/AppContext";
import { Tree, TreeProps } from "antd";
import "./folder-tree.less"

export default function FolderTree() {
  const tree = useTreeData();
  const [domains, setDomains] = useDomain();

  const onSelect: TreeProps['onSelect'] = (selectedKeys) => {
    setDomains(selectedKeys as string[])
  }
  const onShowDomainContextMenu = () => {
    console.log("onShowDomainContextMenu");
  }
  return (
    <div className="folder-tree h-full p-2 overflow-auto">
      <Tree
        autoExpandParent
        blockNode
        onSelect={onSelect}
        treeData={tree}
        checkedKeys={domains}
        onRightClick={onShowDomainContextMenu}
      />
    </div>
  );
}
