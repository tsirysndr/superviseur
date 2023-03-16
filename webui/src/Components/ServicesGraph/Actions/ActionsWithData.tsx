import { FC } from "react";
import {
  useGetServicesQuery,
  useRestartMutation,
  useStartMutation,
  useStopMutation,
} from "../../../Hooks/GraphQL";
import Actions from "./Actions";

const ActionsWithData: FC = (props) => {
  const [startMutation] = useStartMutation();
  const [stopMutation] = useStopMutation();
  const [restartMutation] = useRestartMutation();
  const { data: getServicesData, loading: getServicesLoading } =
    useGetServicesQuery();
  const allServicesAreRunning =
    !getServicesLoading &&
    getServicesData!.services.every((service) => service.status === "RUNNING");
  const onStartAll = () => startMutation();
  const onRestartAll = () => restartMutation();
  const onStopAll = () => stopMutation();
  return (
    <>
      {!getServicesLoading && (
        <Actions
          onStart={onStartAll}
          onRestart={onRestartAll}
          onStop={onStopAll}
          allServicesAreRunning={allServicesAreRunning}
        />
      )}
    </>
  );
};

export default ActionsWithData;
