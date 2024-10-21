
export const privacyId = "privacy"
export const privacyAnchor = {
  key: privacyId,
  href: `#${privacyId}`,
  title: "Privacy",
}
export default function Privacy() {
    return <div id={privacyId} className="h-28">Privacy</div>
}
