import { FC } from "react";
import Templates from "./Templates";
import { useRecoilState } from "recoil";
import { templatesState } from "./TemplatesState";
import { useNewProjectMutation } from "../../../Hooks/GraphQL";
import generate from "boring-name-generator";
import { useNavigate } from "react-router-dom";

const TemplatesWithData: FC = () => {
  const navigate = useNavigate();
  const [templates] = useRecoilState(templatesState);
  const [newProject] = useNewProjectMutation();

  const onCreateNewProject = async (template: any) => {
    console.log(template);
    const { data } = await newProject({
      variables: {
        name: generate({ number: true }).dashed,
      },
    });
    const id = data?.newProject?.id;
    navigate(`/projects/${id}`);
  };

  return (
    <>
      <Templates
        templates={templates}
        onCreateNewProject={onCreateNewProject}
      />
    </>
  );
};

export default TemplatesWithData;
