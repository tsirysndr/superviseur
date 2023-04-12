import { FC } from "react";
import { parseIntoSettings } from "../../Mocks/Settings";
import Settings from "./Settings";
import { Settings as SettingsData } from "../../Types/Settings";
import { useGetServiceQuery, useGetServicesQuery } from "../../Hooks/GraphQL";
import { useParams } from "react-router-dom";
export interface SettingsWithDataProps {
  serviceId: string;
}

const SettingsWithData: FC<SettingsWithDataProps> = ({ serviceId }) => {
  const { projectId } = useParams();
  const { data: getServicesData, loading: getServicesLoading } =
    useGetServicesQuery({
      variables: { projectId: projectId! },
    });
  const { data, loading } = useGetServiceQuery({
    variables: {
      id: serviceId,
      projectId: projectId!,
    },
  });
  const settings =
    (loading && getServicesLoading) || !data
      ? []
      : parseIntoSettings(data!.service, getServicesData!.services);
  return <Settings settings={settings} onSave={(x: SettingsData) => {}} />;
};

export default SettingsWithData;
