import Status from "./Status";
import { FC } from "react";
import { statuses } from "../../Mocks/ServiceStatuses";

const StatusWithData: FC = () => {
  return (
    <Status
      statuses={statuses}
      onStart={() => {}}
      onRestart={() => {}}
      onStop={() => {}}
    />
  );
};

export default StatusWithData;
