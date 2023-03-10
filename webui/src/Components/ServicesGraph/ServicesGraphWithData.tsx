import { FC } from "react";
import { edges } from "../../Mocks/Edges";
import { nodes } from "../../Mocks/Nodes";
import ServicesGraph from "./ServicesGraph";

const ServicesGraphWithData: FC = () => {
  return <ServicesGraph nodes={nodes} edges={edges} />;
};

export default ServicesGraphWithData;
