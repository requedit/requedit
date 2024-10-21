
import AdvancedSettings, { advancedAnchor } from "./advanced";
import GeneralSettings, { generalAnchor } from "./general";
import AppearanceSetting, { appearanceAnchor } from "./appearance";
import GithubSettings, { githubAnchor } from "./github";
import PluginsSetting, { pluginsAnchor } from "./plugins";
import PrivacySetting, { privacyAnchor } from "./privacy";


export const settingItems = [
  generalAnchor,
  appearanceAnchor,
  pluginsAnchor,
  githubAnchor,
  advancedAnchor,
  privacyAnchor
]
export const Settings = () => {
  return (
    <>
      <GeneralSettings />
      <AppearanceSetting />
      <PluginsSetting />
      <GithubSettings />
      <AdvancedSettings />
      <PrivacySetting />
    </>
  );
}
