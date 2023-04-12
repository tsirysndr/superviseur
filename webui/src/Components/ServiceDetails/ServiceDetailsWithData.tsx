import { FC } from "react";
import { useGetServicesQuery } from "../../Hooks/GraphQL";
import ServiceDetails from "./ServiceDetails";
import _ from "lodash";
import { useParams } from "react-router-dom";
export interface ServiceDetailsProps {
  selectedNode?: string;
}

const ServiceDetailsWithData: FC<ServiceDetailsProps> = (props) => {
  const { projectId } = useParams();
  const { data, loading } = useGetServicesQuery({
    variables: { projectId: projectId! },
  });
  const nodes = loading
    ? []
    : data?.services.map((service) => ({
        id: service.id,
        label: `${_.startCase(service.status)}\n<b>${service.name}</b>`,
      })) || [];
  return <>{!loading && <ServiceDetails nodes={nodes} {...props} />}</>;
};

export default ServiceDetailsWithData;
