import { FC, useEffect, useMemo, useState } from "react";
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
import { useParams } from "react-router-dom";

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
  const { projectId } = useParams();
  const { enqueue } = useSnackbar();
  const [starting, setStarting] = useState(false);
  const [stopping, setStopping] = useState(false);
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
  } = useGetServicesQuery({
    variables: { projectId: projectId! },
  });
  const allServicesAreRunning = useMemo(
    () =>
      !getServicesLoading &&
      getServicesData!.services.every(
        (service) => service.status === "RUNNING"
      ),
    [getServicesData, getServicesLoading]
  );
  const onStartAll = () => {
    setStarting(true);
    startMutation({
      variables: {
        projectId: projectId!,
      },
    });
  };
  const onRestartAll = () => {
    setStarting(true);
    restartMutation({
      variables: {
        projectId: projectId!,
      },
    });
  };
  const onStopAll = () => {
    setStopping(true);
    stopMutation({
      variables: {
        projectId: projectId!,
      },
    });
  };

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
    if (allServicesAreRunning) {
      setStarting(false);
      enqueue({
        message: "All services successfully started",
        overrides: styles.snackbar,
      });
    }
  }, [allServicesAreRunning]);

  useEffect(() => {
    if (allServicesAreStopped) {
      setStopping(false);
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
          starting={starting}
          stopping={stopping}
        />
      )}
    </>
  );
};

export default ActionsWithData;
