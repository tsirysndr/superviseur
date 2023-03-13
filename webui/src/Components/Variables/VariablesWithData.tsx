import { FC } from "react";
import { useGetServiceQuery } from "../../Hooks/GraphQL";
import Variables from "./Variables";

const VariablesWithData: FC = () => {
  const { data } = useGetServiceQuery({
    variables: {
      id: "1",
    },
  });
  const variables =
    data?.service.env.map((env) => ({
      name: env.split("=")[0],
      value: env.split("=")[1],
    })) || [];
  return <Variables variables={variables} />;
};

export default VariablesWithData;
