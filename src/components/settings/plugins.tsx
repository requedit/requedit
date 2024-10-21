
export const pluginsId = 'plugins'

export const pluginsAnchor = {
  key: pluginsId,
  href: `#${pluginsId}`,
  title: "Plugins",
}

export default function Plugins() {
  return <div id={pluginsId} className="h-28">Plugins</div>
}
