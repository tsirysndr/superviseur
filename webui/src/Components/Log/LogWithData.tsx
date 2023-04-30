import { FC, useEffect, useState } from "react";
import {
  useGetLogsQuery,
  useGetStatusQuery,
  useLogsSubscription,
  useTailLogsQuery,
} from "../../Hooks/GraphQL";
import Log from "./Log";
import { useParams } from "react-router-dom";

export interface LogWithDataProps {
  serviceId: string;
}

const LogWithData: FC<LogWithDataProps> = ({ serviceId }) => {
  const { projectId } = useParams();
  const { data: getStatusData } = useGetStatusQuery({
    variables: {
      id: serviceId,
    },
  });
  const { data: tailLogs } = useTailLogsQuery({
    variables: {
      id: serviceId,
      numLines: 100,
      projectId: projectId!,
    },
  });
  const [lines, setLines] = useState<string[]>([]);
  const { data, loading } = useLogsSubscription({
    variables: {
      id: serviceId,
      projectId: projectId!,
    },
  });

  useEffect(() => {
    if (data?.logs.line) {
      setLines((prev: string[]) => [...prev, data.logs.line]);
    }
  }, [data?.logs.line]);

  useEffect(() => {
    if (getStatusData?.status.state === "Stopped") {
      setLines(tailLogs?.tail.lines || []);
    }
  }, [getStatusData?.status, tailLogs?.tail]);

  return (
    <>
      {(!loading || getStatusData?.status.state === "Stopped") && (
        <Log lines={lines} />
      )}
    </>
  );
};

export default LogWithData;
