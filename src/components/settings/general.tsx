export const generalId = "general"
export const generalAnchor = {
  key: generalId,
  href: `#${generalId}`,
  title: "General",
}
export default function GeneralSettings() {
    return <div id={generalId} className="h-28">General</div>
}
