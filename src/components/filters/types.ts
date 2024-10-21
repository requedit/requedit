
export const categories = [
  "All",
  "Http",
  "Https",
  "Websocket",
  "Json",
  "Form",
  "XML",
  "JS",
  "CSS",
  "GraphQL",
  "Doc",
  "Media",
  "Other",
];

export type FiltersValue = {
  search: string
  category: typeof categories[number]
}
