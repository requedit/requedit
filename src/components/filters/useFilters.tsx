import { useState } from "react";
import { FiltersValue } from "./types";
import { Category } from "@/constants/catetory";

const useFilters = () => {
  const [filters, setFilters] = useState<FiltersValue>({
    search: "",
    category: Category.ALL,
  });

  const handleFiltersChange = (filters: FiltersValue) => {
    setFilters(filters);
  };

  return {
    filters,
    handleFiltersChange,
  };
};

export default useFilters;
