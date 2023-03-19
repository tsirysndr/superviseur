import { FC, useEffect, useMemo } from "react";
import {
  useGetServicesQuery,
  useOnRestartAllSubscription,
  useOnStartAllSubscription,
  useOnStopAllSubscription,
  useRestartMutation,
  useStartMutation,
  useStopMutation,
} from "../../../Hooks/GraphQL";
import { useSnackbar } from "baseui/snackbar";
import Actions from "./Actions";

const styles = {
  snackbar: {
    Content: {
      style: {
        width: "100%",
      },
    },
    Message: {
      style: {
        width: "100%",
        textAlign: "center",
      },
    },
  },
};

const ActionsWithData: FC = () => {
  const { enqueue } = useSnackbar();
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

  const allServicesAreStarted = useMemo(
    () => onStartAllSubscription?.onStartAll,
    [onStartAllSubscription?.onStartAll]
  );

  const allServicesAreStopped = useMemo(
    () => onStopAllSubscription?.onStopAll,
    [onStopAllSubscription?.onStopAll]
  );

  const allServicesAreRestarted = useMemo(
    () => onRestartAllSubscription?.onRestartAll,
    [onRestartAllSubscription?.onRestartAll]
  );

  useEffect(() => {
    if (
      allServicesAreRestarted ||
      allServicesAreStarted ||
      allServicesAreStopped
    ) {
      refetch();
    }
  }, [allServicesAreRestarted, allServicesAreStarted, allServicesAreStopped]);

  useEffect(() => {
    if (allServicesAreStarted) {
      enqueue({
        message: "All services successfully started",
        overrides: styles.snackbar,
      });
    }
  }, [allServicesAreStarted]);

  useEffect(() => {
    if (allServicesAreStopped) {
      enqueue({
        message: "All services successfully stopped",
        overrides: styles.snackbar,
      });
    }
  }, [allServicesAreStopped]);

  useEffect(() => {
    if (allServicesAreRestarted) {
      enqueue({
        message: "All services successfully restarted",
        overrides: styles.snackbar,
      });
    }
  }, [allServicesAreRestarted]);

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
