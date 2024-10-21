import { useState } from "react";
import { FiltersValue } from "./types";

const useFilters = () => {
  const [filters, setFilters] = useState<FiltersValue>({
    search: "",
    category: "All",
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
