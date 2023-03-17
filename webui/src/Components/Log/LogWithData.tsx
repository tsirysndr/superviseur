import { FC, useEffect, useState } from "react";
import {
  useGetLogsQuery,
  useGetStatusQuery,
  useLogsSubscription,
  useTailLogsQuery,
} from "../../Hooks/GraphQL";
import Log from "./Log";

export interface LogWithDataProps {
  serviceId: string;
}

const LogWithData: FC<LogWithDataProps> = ({ serviceId }) => {
  const { data: getStatusData } = useGetStatusQuery({
    variables: {
      id: serviceId,
    },
  });
  const { data: tailLogs } = useTailLogsQuery({
    variables: {
      id: serviceId,
      numLines: 100,
    },
  });
  const [lines, setLines] = useState<string[]>([]);
  const { data, loading } = useLogsSubscription({
    variables: {
      id: serviceId,
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
