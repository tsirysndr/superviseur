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

export interface StatusWithDataProps {
  selectedNode: string;
}

const StatusWithData: FC<StatusWithDataProps> = ({ selectedNode }) => {
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
