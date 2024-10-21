import { Divider, Flex, Input, Tag } from "antd";
import { useState } from "react";
import { FilterOutlined } from "@ant-design/icons";
import { FiltersValue, categories } from "./types";



export default function Filters(props: {
  value: FiltersValue,
  onChange: (value: FiltersValue) => void
}) {

  const [selected, setSelected] = useState<string>(props.value.category);
  return (
    <Flex gap={4} wrap align="center">
      <Input
        size="small"
        placeholder="Filter"
        prefix={<FilterOutlined />}
        style={{ width: 200 }}
        value={props.value.search}
        onChange={e => {
          props.onChange({
            ...props.value,
            search: e.target.value
          })
        }}
      />
      <Divider type="vertical" />
      <Flex>
        {categories.map<React.ReactNode>((tag) => (
          <Tag.CheckableTag
            key={tag}
            checked={tag == selected}
            onChange={() => {
              setSelected(tag)
              props.onChange({
                ...props.value,
                category: tag
              })
            }}
          >
            {tag}
          </Tag.CheckableTag>
        ))}
      </Flex>
    </Flex>
  );
}
