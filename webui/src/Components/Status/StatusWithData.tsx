import Status from "./Status";
import { FC } from "react";
import { parseIntoStatuses, statuses } from "../../Mocks/ServiceStatuses";
import { useGetStatusQuery } from "../../Hooks/GraphQL";

const StatusWithData: FC = () => {
  const { data, loading } = useGetStatusQuery({
    variables: {
      id: "1",
    },
  });
  const statuses = loading ? [] : parseIntoStatuses(data!.status);
  return (
    <>
      {!loading && (
        <Status
          statuses={statuses}
          onStart={() => {}}
          onRestart={() => {}}
          onStop={() => {}}
        />
      )}
    </>
  );
};

export default StatusWithData;
