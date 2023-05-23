import { FC } from "react";
import {
  useCreateEnvVarMutation,
  useDeleteEnvVarMutation,
  useGetEnvVarsQuery,
  useUpdateEnvVarMutation,
} from "../../Hooks/GraphQL";
import { EnvironmentVariable } from "../../Types/EnvironmentVariable";
import Variables from "./Variables";
import { useParams } from "react-router-dom";

export interface VariablesWithDataProps {
  serviceId: string;
}

const VariablesWithData: FC<VariablesWithDataProps> = ({ serviceId }) => {
  const { projectId } = useParams();
  const [createEnvVar] = useCreateEnvVarMutation();
  const [deleteEnvVar] = useDeleteEnvVarMutation();
  const [updateEnvVar] = useUpdateEnvVarMutation();
  const { data, loading } = useGetEnvVarsQuery({
    variables: {
      id: serviceId,
      projectId: projectId!,
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
        projectId: projectId!,
      },
    });
  };

  const onEdit = async (variable: EnvironmentVariable) => {
    await updateEnvVar({
      variables: {
        id: data!.service.id,
        name: variable.name,
        value: variable.value,
        projectId: projectId!,
      },
    });
  };

  const onRemove = async (variable: EnvironmentVariable) => {
    await deleteEnvVar({
      variables: {
        id: data!.service.id,
        name: variable.name,
        projectId: projectId!,
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
