import { Category } from "@/constants/catetory";

export const categories = [
  Category.ALL,
  Category.XHR,
  Category.DOC,
  Category.CSS,
  Category.JS,
  Category.MEDIA,
  Category.WEBSOCKET,
];

export type FiltersValue = {
  search: string
  category: Category
}
