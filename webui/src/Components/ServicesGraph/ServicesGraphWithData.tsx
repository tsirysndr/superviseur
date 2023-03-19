import { FC, useEffect, useState } from "react";
import {
  useGetServicesQuery,
  useOnRestartSubscription,
  useOnStartSubscription,
  useOnStopSubscription,
} from "../../Hooks/GraphQL";
import ServicesGraph from "./ServicesGraph";
import _ from "lodash";
import { Node } from "../../Types/Node";
import { Edge } from "../../Types/Edge";

const ServicesGraphWithData: FC = () => {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const { data: onStartSubscription } = useOnStartSubscription();
  const { data: onStopSubscription } = useOnStopSubscription();
  const { data: onRestartSubscription } = useOnRestartSubscription();
  const { data, loading } = useGetServicesQuery();

  useEffect(() => {
    if (data?.services) {
      setNodes(
        data?.services.map((service) => ({
          id: service.id,
          label: `${_.startCase(service.status)}\n<b>${service.name}</b>`,
        })) || []
      );
      setEdges(
        data?.services
          .filter((service) => service.dependsOn.length > 0)
          .map((service) => {
            return service.dependsOn.map((dependency) => ({
              from: service.id,
              to: dependency,
              label: "depends on",
            }));
          })
          .flat() || []
      );
    }
  }, [data?.services]);
  return (
    <>
      {(!loading || nodes.length > 0 || edges.length > 0) && (
        <ServicesGraph nodes={nodes} edges={edges} />
      )}
    </>
  );
};

export default ServicesGraphWithData;
