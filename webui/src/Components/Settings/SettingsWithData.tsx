import { FC } from "react";
import { parseIntoSettings } from "../../Mocks/Settings";
import Settings from "./Settings";
import { Settings as SettingsData } from "../../Types/Settings";
import { useGetServiceQuery, useGetServicesQuery } from "../../Hooks/GraphQL";

export interface SettingsWithDataProps {
  serviceId: string;
}

const SettingsWithData: FC<SettingsWithDataProps> = ({ serviceId }) => {
  const { data: getServicesData, loading: getServicesLoading } =
    useGetServicesQuery();
  const { data, loading } = useGetServiceQuery({
    variables: {
      id: serviceId,
    },
  });
  const settings =
    (loading && getServicesLoading) || !data
      ? []
      : parseIntoSettings(data!.service, getServicesData!.services);
  return <Settings settings={settings} onSave={(x: SettingsData) => {}} />;
};

export default SettingsWithData;
