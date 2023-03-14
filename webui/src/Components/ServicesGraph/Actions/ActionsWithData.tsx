import { FC } from "react";
import Actions from "./Actions";

export interface ActionsWithDataProps {
  onStart: () => void;
  onStop: () => void;
  onRestart: () => void;
}

const ActionsWithData: FC<ActionsWithDataProps> = (props) => {
  const allServicesAreRunning = true;
  return <Actions {...props} allServicesAreRunning={allServicesAreRunning} />;
};

export default ActionsWithData;
