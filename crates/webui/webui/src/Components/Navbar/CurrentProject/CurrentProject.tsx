import { FC, useEffect, useState } from "react";
import { Folder } from "@styled-icons/bootstrap";
import { Edit } from "@styled-icons/remix-line";
import styled from "@emotion/styled";
import { Controller, useForm } from "react-hook-form";

const ProjectName = styled.div`
  color: #fff;
`;

const EditIcon = styled(Edit)`
  margin-left: 5px;
  color: #ffffff86;
  &:hover {
    color: #fff;
  }
`;

const EditButton = styled.button`
  background-color: transparent;
  border: none;
  cursor: pointer;
`;

const Input = styled.input`
  background-color: transparent;
  border: none;
  color: #fff;
  font-size: 16px;
  font-family: RockfordSansRegular;
  padding: 0;
  &:focus {
    outline: none;
  }
`;

export type CurrentProjectProps = {
  projectId?: string;
  projectName?: string;
  onEdit: (projectName: string) => void;
};

const CurrentProject: FC<CurrentProjectProps> = ({
  projectId,
  projectName,
  onEdit,
}) => {
  const [edit, setEdit] = useState(false);
  const { control, watch } = useForm();

  useEffect(() => {
    const subscription = watch(
      (value, { name }) => name && onEdit(value[name])
    );
    return () => subscription.unsubscribe();
  }, [watch, onEdit]);

  useEffect(() => {
    const keyDownHandler = (event: any) => {
      console.log("User pressed: ", event.key);

      if (event.key === "Enter") {
        event.preventDefault();
        setEdit(false);
      }
    };

    document.addEventListener("keydown", keyDownHandler);

    return () => {
      document.removeEventListener("keydown", keyDownHandler);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      {projectId && (
        <>
          <Folder size={18} color="#fff" style={{ marginRight: 10 }} />
          {!edit && <ProjectName>{projectName}</ProjectName>}
          {edit && (
            <Controller
              render={({ field }) => (
                <Input
                  {...(field as any)}
                  autoFocus={true}
                  onBlur={() => setEdit(false)}
                />
              )}
              name="projectName"
              control={control}
              defaultValue={projectName}
            />
          )}
          {!edit && (
            <EditButton onClick={() => setEdit(true)}>
              <EditIcon size={18} />
            </EditButton>
          )}
        </>
      )}
    </>
  );
};

export default CurrentProject;
