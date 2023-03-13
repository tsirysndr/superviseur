import { FC } from "react";
import { parseIntoSettings, settings } from "../../Mocks/Settings";
import Settings from "./Settings";
import { Settings as SettingsData } from "../../Types/Settings";
import { useGetServiceQuery } from "../../Hooks/GraphQL";

const SettingsWithData: FC = () => {
  const { data, loading } = useGetServiceQuery({
    variables: {
      id: "1",
    },
  });
  const settings = loading ? [] : parseIntoSettings(data!.service);
  return <Settings settings={settings} onSave={(x: SettingsData) => {}} />;
};

export default SettingsWithData;
