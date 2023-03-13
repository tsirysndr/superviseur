import { FC } from "react";
import { useGetLogsQuery } from "../../Hooks/GraphQL";
import Log from "./Log";

const LogWithData: FC = () => {
  const { data } = useGetLogsQuery({
    variables: {
      id: "1",
    },
  });
  return <Log lines={data?.logs.lines || []} />;
};

export default LogWithData;
