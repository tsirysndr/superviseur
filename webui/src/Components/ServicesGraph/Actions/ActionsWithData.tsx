import { FC, useEffect } from "react";
import {
  useGetServicesQuery,
  useOnRestartAllSubscription,
  useOnStartAllSubscription,
  useOnStopAllSubscription,
  useRestartMutation,
  useStartMutation,
  useStopMutation,
} from "../../../Hooks/GraphQL";
import Actions from "./Actions";

const ActionsWithData: FC = () => {
  const { data: onStartAllSubscription } = useOnStartAllSubscription();
  const { data: onStopAllSubscription } = useOnStopAllSubscription();
  const { data: onRestartAllSubscription } = useOnRestartAllSubscription();
  const [startMutation] = useStartMutation();
  const [stopMutation] = useStopMutation();
  const [restartMutation] = useRestartMutation();
  const {
    data: getServicesData,
    loading: getServicesLoading,
    refetch,
  } = useGetServicesQuery();
  const allServicesAreRunning =
    !getServicesLoading &&
    getServicesData!.services.every((service) => service.status === "RUNNING");
  const onStartAll = () => startMutation();
  const onRestartAll = () => restartMutation();
  const onStopAll = () => stopMutation();

  useEffect(() => {
    if (
      onStartAllSubscription?.onStartAll ||
      onStopAllSubscription?.onStopAll ||
      onRestartAllSubscription?.onRestartAll
    ) {
      refetch();
    }
  }, [
    onStartAllSubscription,
    onStopAllSubscription,
    onRestartAllSubscription,
    refetch,
  ]);

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
