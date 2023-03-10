import { FC } from "react";
import { variables } from "../../Mocks/Variables";
import Variables from "./Variables";

const VariablesWithData: FC = () => {
  return <Variables variables={variables} />;
};

export default VariablesWithData;
