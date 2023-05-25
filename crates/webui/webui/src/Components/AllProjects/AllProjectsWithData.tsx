import { FC } from "react";
import { useNavigate } from "react-router-dom";
import { useGetProjectsQuery } from "../../Hooks/GraphQL";
import AllProjects from "./AllProjects";

const AllProjectsWithData: FC = () => {
  const navigate = useNavigate();
  const { data, loading } = useGetProjectsQuery();
  const onCreateProject = () => navigate("/new");
  const onOpenProject = (projectId: string) =>
    navigate(`/projects/${projectId}`);

  return (
    <>
      {!loading && (
        <AllProjects
          onCreateProject={onCreateProject}
          onOpenProject={onOpenProject}
          projects={data?.projects || []}
        />
      )}
    </>
  );
};

export default AllProjectsWithData;
