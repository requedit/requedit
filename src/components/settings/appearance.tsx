
export const appearanceId = 'appearance'
export const appearanceAnchor = {
  key: appearanceId,
  href: `#${appearanceId}`,
  title: "Appearance",
}
export default function Appearance() {
  return  <div id={appearanceId} className="h-28">Appearance</div>
}
