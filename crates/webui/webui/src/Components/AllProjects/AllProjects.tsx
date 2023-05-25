import styled from "@emotion/styled";
import { FC } from "react";
import { FlexGrid, FlexGridItem } from "baseui/flex-grid";
import { BlockProps } from "baseui/block";

const NoProjects = styled.div`
  height: 400px;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 18px;
  border: 1px dashed #a0cfe8;
  border-radius: 15px;
  cursor: pointer;
  &:hover {
    border: 1.3px dashed #a0cfe8;
  }
`;

const Name = styled.div`
  font-weight: 600;
`;

const itemProps: BlockProps = {
  backgroundColor: "#a0cfe812",
  height: "150px",
  display: "flex",
  padding: "25px",
  color: "#2be6e6",
  overrides: {
    Block: {
      style: {
        borderRadius: "5px",
        cursor: "pointer",
      },
    },
  },
};

export type AllProjectsProps = {
  onCreateProject: () => void;
  onOpenProject: (id: string) => void;
  projects: any[];
};

const AllProjects: FC<AllProjectsProps> = ({
  onCreateProject,
  projects,
  onOpenProject,
}) => {
  return (
    <>
      {projects.length === 0 && (
        <NoProjects onClick={onCreateProject}>
          <div>No projects yet. Create a new project to get started.</div>
        </NoProjects>
      )}
      {projects.length > 0 && (
        <FlexGrid
          flexGridColumnCount={3}
          flexGridColumnGap="scale800"
          flexGridRowGap="scale800"
        >
          {projects.map((project) => (
            <FlexGridItem
              {...itemProps}
              onClick={() => onOpenProject(project.id)}
            >
              <div>
                <Name>{project.name}</Name>
              </div>
            </FlexGridItem>
          ))}
        </FlexGrid>
      )}
    </>
  );
};

export default AllProjects;
