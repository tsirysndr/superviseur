import { FC, useEffect, useState } from "react";
import { useTailLogsQuery } from "../../Hooks/GraphQL";
import Log from "./Log";

export interface LogWithDataProps {
  serviceId: string;
}

const LogWithData: FC<LogWithDataProps> = ({ serviceId }) => {
  const [lines, setLines] = useState<string[]>([]);
  const { data, startPolling, stopPolling } = useTailLogsQuery({
    variables: {
      id: serviceId,
      numLines: 1,
    },
  });

  useEffect(() => {
    startPolling(200);
    return () => {
      stopPolling();
    };
  });

  useEffect(() => {
    if (data?.tail.lines) {
      setLines((prev: string[]) => [...prev, ...data.tail.lines]);
    }
  }, [data?.tail.lines]);
  return <Log lines={lines} />;
};

export default LogWithData;
