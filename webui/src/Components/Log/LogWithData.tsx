import { FC } from "react";
import { lines } from "../../Mocks/Lines";
import Log from "./Log";

const LogWithData: FC = () => {
  return <Log lines={lines} />;
};

export default LogWithData;
