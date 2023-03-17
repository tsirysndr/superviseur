import { FC } from "react";
import {
  useCreateEnvVarMutation,
  useDeleteEnvVarMutation,
  useGetEnvVarsQuery,
  useUpdateEnvVarMutation,
} from "../../Hooks/GraphQL";
import { EnvironmentVariable } from "../../Types/EnvironmentVariable";
import Variables from "./Variables";

export interface VariablesWithDataProps {
  serviceId: string;
}

const VariablesWithData: FC<VariablesWithDataProps> = ({ serviceId }) => {
  const [createEnvVar] = useCreateEnvVarMutation();
  const [deleteEnvVar] = useDeleteEnvVarMutation();
  const [updateEnvVar] = useUpdateEnvVarMutation();
  const { data, loading } = useGetEnvVarsQuery({
    variables: {
      id: serviceId,
    },
  });
  const variables =
    data?.service.env.map((env) => ({
      name: env.split("=")[0],
      value: env.split("=")[1],
    })) || [];

  const onAdd = async (variable: EnvironmentVariable) => {
    await createEnvVar({
      variables: {
        id: data!.service.id,
        name: variable.name,
        value: variable.value,
      },
    });
  };

  const onEdit = async (variable: EnvironmentVariable) => {
    await updateEnvVar({
      variables: {
        id: data!.service.id,
        name: variable.name,
        value: variable.value,
      },
    });
  };

  const onRemove = async (variable: EnvironmentVariable) => {
    await deleteEnvVar({
      variables: {
        id: data!.service.id,
        name: variable.name,
      },
    });
  };

  return (
    <>
      {!loading && (
        <Variables
          variables={variables}
          onAdd={onAdd}
          onEdit={onEdit}
          onRemove={onRemove}
        />
      )}
    </>
  );
};

export default VariablesWithData;
