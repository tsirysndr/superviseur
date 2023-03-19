import Status from "./Status";
import { FC, useEffect } from "react";
import { parseIntoStatuses } from "../../Mocks/ServiceStatuses";
import {
  useGetStatusQuery,
  useOnRestartSubscription,
  useOnStartSubscription,
  useOnStopSubscription,
  useRestartMutation,
  useStartMutation,
  useStopMutation,
} from "../../Hooks/GraphQL";
import { useSnackbar } from "baseui/snackbar";

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
  const [startMutation] = useStartMutation();
  const [stopMutation] = useStopMutation();
  const [restartMutation] = useRestartMutation();
  const { data: onStartSubscription } = useOnStartSubscription();
  const { data: onStopSubscription } = useOnStopSubscription();
  const { data: onRestartSubscription } = useOnRestartSubscription();
  const { data, loading, refetch } = useGetStatusQuery({
    variables: {
      id: selectedNode,
    },
  });
  const statuses = loading ? [] : parseIntoStatuses(data!.status);
  const onStart = () => startMutation({ variables: { id: selectedNode } });
  const onRestart = () => restartMutation({ variables: { id: selectedNode } });
  const onStop = () => stopMutation({ variables: { id: selectedNode } });

  useEffect(() => {
    if (
      onStartSubscription?.onStart ||
      onStopSubscription?.onStop ||
      onRestartSubscription?.onRestart
    ) {
      refetch();
    }
  }, [onStartSubscription, onStopSubscription, onRestartSubscription, refetch]);

  useEffect(() => {
    if (onStartSubscription?.onStart) {
      enqueue({
        message: `'${onStartSubscription?.onStart.payload.name}' successfully started`,
        overrides: styles.snackbar,
      });
    }
  }, [onStartSubscription?.onStart]);

  useEffect(() => {
    if (onStopSubscription?.onStop) {
      enqueue({
        message: `'${onStopSubscription?.onStop.payload.name}' successfully stopped`,
        overrides: styles.snackbar,
      });
    }
  }, [onStopSubscription?.onStop]);

  useEffect(() => {
    if (onRestartSubscription?.onRestart) {
      enqueue({
        message: `'${onRestartSubscription?.onRestart.payload.name}' successfully restarted`,
        overrides: styles.snackbar,
      });
    }
  }, [onRestartSubscription?.onRestart]);

  return (
    <>
      {!loading && (
        <Status
          statuses={statuses}
          onStart={onStart}
          onRestart={onRestart}
          onStop={onStop}
        />
      )}
    </>
  );
};

export default StatusWithData;
