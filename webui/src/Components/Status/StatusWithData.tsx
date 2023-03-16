import Status from "./Status";
import { FC } from "react";
import { parseIntoStatuses } from "../../Mocks/ServiceStatuses";
import {
  useGetStatusQuery,
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
  const { data, loading } = useGetStatusQuery({
    variables: {
      id: selectedNode,
    },
  });
  const statuses = loading ? [] : parseIntoStatuses(data!.status);
  const onStart = () => startMutation({ variables: { id: selectedNode } });
  const onRestart = () => restartMutation({ variables: { id: selectedNode } });
  const onStop = () => stopMutation({ variables: { id: selectedNode } });
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
