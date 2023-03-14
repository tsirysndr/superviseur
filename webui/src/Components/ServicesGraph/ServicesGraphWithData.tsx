import { FC } from "react";
import { useGetServicesQuery } from "../../Hooks/GraphQL";
import ServicesGraph from "./ServicesGraph";
import _ from "lodash";

const ServicesGraphWithData: FC = () => {
  const { data, loading } = useGetServicesQuery();
  const nodes =
    data?.services.map((service) => ({
      id: service.id,
      label: `${_.startCase(service.status)}\n<b>${service.name}</b>`,
    })) || [];
  const edges =
    data?.services
      .filter((service) => service.dependsOn.length > 0)
      .map((service) => {
        return service.dependsOn.map((dependency) => ({
          from: service.id,
          to: dependency,
          label: "depends on",
        }));
      })
      .flat() || [];
  return <>{!loading && <ServicesGraph nodes={nodes} edges={edges} />}</>;
};

export default ServicesGraphWithData;
