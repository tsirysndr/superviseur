import styled from "@emotion/styled";
import { FC, useState } from "react";
import { FlexGrid, FlexGridItem } from "baseui/flex-grid";
import { BlockProps } from "baseui/block";
import { Tag, KIND, VARIANT } from "baseui/tag";
import { Delete } from "@styled-icons/fluentui-system-regular";
import { CodeSlash } from "@styled-icons/bootstrap";

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
  font-weight: 500;
`;

const itemProps: BlockProps = {
  backgroundColor: "#a0cfe812",
  height: "150px",
  display: "flex",
  color: "#310755",
  overrides: {
    Block: {
      style: {
        borderRadius: "5px",
        cursor: "pointer",
      },
    },
  },
};

const Actions = styled.div`
  display: flex;
  position: absolute;
  top: 0;
  right: 0;
  flex-direction: row;
  padding: 10px;
  background-color: #f8fcfd;
`;

const Button = styled.div`
  border: none;
  background-color: initial;
  z-index: 2;
  cursor: pointer;
  padding: 2px;
`;

export type ProjectProps = {
  data: any;
  onOpenProject: (id: string) => void;
};

const Project: FC<ProjectProps> = (props) => {
  const [displayMenu, setDisplayMenu] = useState(false);
  const { data, onOpenProject } = props;
  return (
    <FlexGridItem
      {...itemProps}
      {...props}
      onClick={() => onOpenProject(data.id)}
      onMouseEnter={() => setDisplayMenu(true)}
      onMouseLeave={() => setDisplayMenu(false)}
    >
      <div style={{ padding: 15, flex: 1, position: "relative" }}>
        {displayMenu && (
          <Actions>
            <Button
              onClick={(event) => {
                event.stopPropagation();
              }}
            >
              <CodeSlash size={20} color="#630be2" />
            </Button>
            <Button
              onClick={(event) => {
                event.stopPropagation();
              }}
            >
              <Delete size={20} color="#630be2" />
            </Button>
          </Actions>
        )}
        <div style={{ height: "calc(100% - 34px)" }}>
          <Name>{data.name}</Name>
        </div>
        <div>
          <Tag
            color="#630be2"
            kind={KIND.custom}
            closeable={false}
            variant={VARIANT.outlined}
            overrides={{
              Root: {
                style: {
                  lineHeight: "11px",
                  paddingTop: "0px",
                  paddingBottom: "0px",
                },
              },
            }}
          >
            devbox
          </Tag>
        </div>
      </div>
    </FlexGridItem>
  );
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
            <Project data={project} onOpenProject={onOpenProject} />
          ))}
        </FlexGrid>
      )}
    </>
  );
};

export default AllProjects;
