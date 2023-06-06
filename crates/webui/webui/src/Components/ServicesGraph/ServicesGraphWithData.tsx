import { FC, useEffect, useState } from "react";
import {
  useGetServicesQuery,
  useOnRestartSubscription,
  useOnStartSubscription,
  useOnStopSubscription,
  useOnStoppingSubscription,
  useOnStartingSubscription,
} from "../../Hooks/GraphQL";
import ServicesGraph from "./ServicesGraph";
import _ from "lodash";
import { Node } from "../../Types/Node";
import { Edge } from "../../Types/Edge";
import { useNavigate, useParams } from "react-router-dom";
import { useHotkeys } from "react-hotkeys-hook";

const ServicesGraphWithData: FC = () => {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const { data: _onStartSubscription } = useOnStartSubscription();
  const { data: _onStopSubscription } = useOnStopSubscription();
  const { data: _onRestartSubscription } = useOnRestartSubscription();
  const { data: _onStartingSubscription } = useOnStartingSubscription();
  const { data: _onStoppingSubscription } = useOnStoppingSubscription();
  const { projectId } = useParams();
  const { data, loading } = useGetServicesQuery({
    variables: { projectId: projectId! },
  });

  const navigate = useNavigate();

  useHotkeys("ctrl+n", (e) => {
    e.preventDefault();
    navigate("/new");
  });

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
