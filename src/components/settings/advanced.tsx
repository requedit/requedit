
export const advancedId = 'advanced'
export const advancedAnchor = {
  key: advancedId,
  href: `#${advancedId}`,
  title: "Advanced",
}
export default function AdvancedSettings() {
  return <div id={advancedId} className="h-28">Advanced</div>
}
