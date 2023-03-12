import { FC } from "react";
import { settings } from "../../Mocks/Settings";
import Settings from "./Settings";
import { Settings as SettingsData } from "../../Types/Settings";

const SettingsWithData: FC = () => {
  return <Settings settings={settings} onSave={(x: SettingsData) => {}} />;
};

export default SettingsWithData;
