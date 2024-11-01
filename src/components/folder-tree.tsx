import { useDomain, useTreeData } from "@/provides/AppContext";
import { Tree, TreeProps } from "antd";

export default function FolderTree() {
  const tree = useTreeData();
  const [domains, setDomains] = useDomain();

  const onSelect: TreeProps['onSelect'] = (selectedKeys) => {
    console.log(selectedKeys)
    setDomains(selectedKeys as string[])
  }
  const onShowDomainContextMenu = () => {
    console.log("onShowDomainContextMenu");
  }
  return (
    <div className="p-2">
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
