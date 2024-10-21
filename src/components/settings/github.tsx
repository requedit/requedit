
export const githubId = "github"
export const githubAnchor = {
  key: githubId,
  href: `#${githubId}`,
  title: "Github",
}
export default function GithubSettings() {
  return <div id={githubId} className="h-28">Github</div>
}
