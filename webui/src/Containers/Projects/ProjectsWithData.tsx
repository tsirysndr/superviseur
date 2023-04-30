import { FC } from "react";
import Projects from "./Projects";
import Navbar from "../../Components/Navbar";
import { useGetProjectsQuery } from "../../Hooks/GraphQL";

const ProjectsWithData: FC = () => {
  const { data } = useGetProjectsQuery();
  console.log(data);
  return (
    <div>
      <Navbar />
      <Projects />
    </div>
  );
};

export default ProjectsWithData;
