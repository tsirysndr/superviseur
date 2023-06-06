import { FC, useEffect } from "react";
import CurrentProject from "./CurrentProject";
import { useParams } from "react-router-dom";
import { useGetProjectQuery } from "../../../Hooks/GraphQL";
import { useRecoilState } from "recoil";
import { currentProjectState } from "./CurrentProjectState";

const CurrentProjectWithData: FC = () => {
  const [state, setState] = useRecoilState(currentProjectState);
  const { projectId } = useParams();
  const { data } = useGetProjectQuery({ variables: { id: projectId! } });
  const name = data?.project?.name;

  useEffect(() => {
    if (name) {
      setState({ ...state, name });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [name]);

  const setProjectName = (name: string) => {
    setState({ ...state, name });
  };

  return (
    <CurrentProject
      projectId={projectId}
      projectName={state.name}
      onEdit={setProjectName}
    />
  );
};

export default CurrentProjectWithData;
