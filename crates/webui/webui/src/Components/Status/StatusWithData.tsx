import Status from "./Status";
import { FC, useEffect, useState } from "react";
import { parseIntoStatuses } from "../../Mocks/ServiceStatuses";
import {
  useGetStatusQuery,
  useOnRestartSubscription,
  useOnStartSubscription,
  useOnStopSubscription,
  useOnStartingSubscription,
  useOnStoppingSubscription,
  useRestartMutation,
  useStartMutation,
  useStopMutation,
} from "../../Hooks/GraphQL";
import { useSnackbar } from "baseui/snackbar";
import { ServiceStatus } from "../../Types/ServiceStatus";
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

export interface StatusWithDataProps {
  selectedNode: string;
}

const StatusWithData: FC<StatusWithDataProps> = ({ selectedNode }) => {
  const { enqueue } = useSnackbar();
  const { projectId } = useParams();
  const [statuses, setStatuses] = useState<ServiceStatus[]>([]);
  const [startMutation] = useStartMutation();
  const [stopMutation] = useStopMutation();
  const [restartMutation] = useRestartMutation();
  const { data: onStartSubscription } = useOnStartSubscription();
  const { data: onStopSubscription } = useOnStopSubscription();
  const { data: onRestartSubscription } = useOnRestartSubscription();
  const { data: onStartingSubscription } = useOnStartingSubscription();
  const { data: onStoppingSubscription } = useOnStoppingSubscription();
  const { data, loading } = useGetStatusQuery({
    variables: {
      id: selectedNode,
    },
    fetchPolicy: "network-only",
  });
  const onStart = () =>
    startMutation({ variables: { id: selectedNode, projectId: projectId! } });
  const onRestart = () =>
    restartMutation({ variables: { id: selectedNode, projectId: projectId! } });
  const onStop = () =>
    stopMutation({ variables: { id: selectedNode, projectId: projectId! } });

  useEffect(() => {
    setStatuses(loading ? [] : parseIntoStatuses(data!.status));
  }, [loading, data]);

  useEffect(() => {
    if (onStartSubscription?.onStart) {
      setStatuses(parseIntoStatuses(onStartSubscription?.onStart.process));
      enqueue({
        message: `'${onStartSubscription?.onStart.payload.name}' successfully started`,
        overrides: styles.snackbar,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onStartSubscription?.onStart]);

  useEffect(() => {
    if (onStopSubscription?.onStop) {
      setStatuses(parseIntoStatuses(onStopSubscription?.onStop.process));
      enqueue({
        message: `'${onStopSubscription?.onStop.payload.name}' successfully stopped`,
        overrides: styles.snackbar,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onStopSubscription?.onStop]);

  useEffect(() => {
    if (onRestartSubscription?.onRestart) {
      enqueue({
        message: `'${onRestartSubscription?.onRestart.payload.name}' successfully restarted`,
        overrides: styles.snackbar,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onRestartSubscription?.onRestart]);

  useEffect(() => {
    if (onStartingSubscription?.onStarting) {
      setStatuses(
        parseIntoStatuses(onStartingSubscription?.onStarting.process)
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onStartingSubscription?.onStarting]);

  useEffect(() => {
    if (onStoppingSubscription?.onStopping) {
      setStatuses(
        parseIntoStatuses(onStoppingSubscription?.onStopping.process)
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onStoppingSubscription?.onStopping]);

  const _onStart = () => {
    onStart();
    setStatuses(
      statuses.map((status) =>
        status.name === "Active" ? { ...status, status: "Starting" } : status
      )
    );
  };

  const _onRestart = () => {
    onRestart();
    setStatuses(
      statuses.map((status) =>
        status.name === "Active" ? { ...status, status: "Starting" } : status
      )
    );
  };

  const _onStop = () => {
    onStop();
    setStatuses(
      statuses.map((status) =>
        status.name === "Active" ? { ...status, status: "Stopping" } : status
      )
    );
  };

  return (
    <>
      {!loading && (
        <Status
          statuses={statuses}
          onStart={_onStart}
          onRestart={_onRestart}
          onStop={_onStop}
        />
      )}
    </>
  );
};

export default StatusWithData;
