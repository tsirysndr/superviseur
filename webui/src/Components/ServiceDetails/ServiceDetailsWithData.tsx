import { FC } from "react";
import { nodes } from "../../Mocks/Nodes";
import ServiceDetails from "./ServiceDetails";

export interface ServiceDetailsProps {
  selectedNode?: string;
}

const ServiceDetailsWithData: FC<ServiceDetailsProps> = (props) => {
  return <ServiceDetails nodes={nodes} {...props} />;
};

export default ServiceDetailsWithData;
