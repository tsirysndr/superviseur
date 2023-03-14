import { Button, SHAPE } from "baseui/button";
import { StatefulPopover } from "baseui/popover";
import { FC, useState } from "react";
import { Plus } from "@styled-icons/bootstrap/Plus";
import { Ellipsis } from "@styled-icons/fa-solid/Ellipsis";
import styled from "@emotion/styled";
import { EnvironmentVariable } from "../../Types/EnvironmentVariable";
import { uniqueId } from "lodash";
import { Input } from "baseui/input";
import { useForm, Controller } from "react-hook-form";
import { StatefulMenu } from "baseui/menu";
import { Check2 } from "@styled-icons/bootstrap/Check2";
import { CloseOutline } from "@styled-icons/evaicons-outline/CloseOutline";

const Title = styled.div``;

const Header = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
  height: 34px;
`;

const VariablesTable = styled.div`
  margin-top: 20px;
  height: calc(100% - 62px);
`;

const VariableRowContainer = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
  height: 40px;
`;

const VariableName = styled.div`
  flex: 1;
`;

const VariableValue = styled.div`
  flex: 1;
`;

const Actions = styled.div``;

const Placeholder = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  border: 1px dashed #cac9c990;
  height: 250px;
`;

const Edit = styled.button`
  cursor: pointer;
  border: none;
  background-color: transparent;
  &:hover {
    opacity: 0.5;
  }
`;

const CheckButton = styled.button`
  border: none;
  background-color: transparent;
  cursor: pointer;
  :hover {
    opacity: 0.6;
  }
`;

const ButtonStyles = {
  BaseButton: {
    style: {
      height: "30px",
      width: "122px",
      fontSize: "12px",
      padding: "0px",
      backgroundColor: "#fff",
      color: "#630be2",
      fontFamily: "RockfordSansMedium",
      border: "2px solid #630be2",
      ":hover": {
        backgroundColor: "#fff",
        opacity: 0.6,
      },
    },
  },
  StartEnhancer: {
    style: {
      marginRight: "3px",
    },
  },
};

export interface VariableRowProps {
  variable: EnvironmentVariable;
  onRemove: (variable: EnvironmentVariable) => void;
  onEdit: (variable: EnvironmentVariable) => void;
}

const VariableRow: FC<VariableRowProps> = (props) => {
  const { variable, onRemove, onEdit } = props;
  const [isEditing, setIsEditing] = useState(false);
  const { control, handleSubmit, reset } = useForm({
    defaultValues: {
      envValue: variable.value,
    },
  });

  const onAdd = () => {
    handleSubmit(
      async (data) => {
        await onEdit({
          name: variable.name,
          value: data.envValue,
        });
        reset();
        setIsEditing(false);
      },
      (x) => {
        console.log(">>", x);
      }
    )();
  };

  const onCancel = () => {
    reset();
    setIsEditing(false);
  };

  return (
    <VariableRowContainer>
      <VariableName>{variable.name}</VariableName>
      {!isEditing && (
        <>
          <VariableValue>{variable.value}</VariableValue>
          <Actions>
            <StatefulPopover
              content={() => (
                <StatefulMenu
                  items={[
                    { id: "edit", label: "Edit" },
                    { id: "remove", label: "Remove" },
                  ]}
                  onItemSelect={({ item }) => {
                    switch (item.id) {
                      case "edit":
                        setIsEditing(true);
                        break;
                      case "remove":
                        onRemove(variable);
                        break;
                    }
                  }}
                />
              )}
              overrides={{
                Inner: {
                  style: {
                    backgroundColor: "#fff",
                  },
                },
                Body: {
                  style: {
                    zIndex: 1,
                  },
                },
              }}
            >
              <Edit>
                <Ellipsis size={20} color="#000" />
              </Edit>
            </StatefulPopover>
          </Actions>
        </>
      )}
      {isEditing && (
        <div
          style={{
            display: "flex",
            flex: 1,
          }}
        >
          <Controller
            render={({ field }) => (
              <Input
                {...(field as any)}
                overrides={{
                  Root: {
                    style: {
                      height: "34px",
                      borderRadius: "2px",
                      width: "186px",
                    },
                  },
                  Input: {
                    style: {
                      fontSize: "14px",
                    },
                  },
                }}
                autoFocus
                clearable
              />
            )}
            control={control}
            name="envValue"
            rules={{ required: true }}
          />
          <>
            <CheckButton onClick={onAdd}>
              <Check2 size={20} color="#000" />
            </CheckButton>
            <CheckButton onClick={onCancel}>
              <CloseOutline size={20} color="#000" />
            </CheckButton>
          </>
        </div>
      )}
    </VariableRowContainer>
  );
};

export interface VariablesProps {
  variables: EnvironmentVariable[];
  onAdd: (variable: EnvironmentVariable) => void;
  onRemove: (variable: EnvironmentVariable) => void;
  onEdit: (variable: EnvironmentVariable) => void;
}

const Variables: FC<VariablesProps> = (props) => {
  const { variables, onAdd, onRemove, onEdit } = props;
  const { control, handleSubmit, reset } = useForm();
  const [showNewVariableInput, setShowNewVariableInput] =
    useState<boolean>(false);
  const [newVariableName, setNewVariableName] = useState<string>("");
  const [newVariableValue, setNewVariableValue] = useState<string>("");

  const _onAdd = () => {
    handleSubmit(
      async (data) => {
        await onAdd({
          name: data.name,
          value: data.value,
        });
        setShowNewVariableInput(false);
        reset();
      },
      (x) => {
        console.log(">>", x);
      }
    )();
  };

  return (
    <div style={{ height: "100%" }}>
      <Header>
        {showNewVariableInput && (
          <>
            <Controller
              render={({ field }) => (
                <Input
                  {...field}
                  placeholder="NEW VARIABLE"
                  overrides={{
                    Root: {
                      style: {
                        height: "34px",
                        borderRadius: "2px",
                        marginRight: "10px",
                      },
                    },
                    Input: {
                      style: {
                        fontSize: "14px",
                      },
                    },
                  }}
                />
              )}
              name="name"
              control={control}
              rules={{ required: true }}
            />
            <Controller
              render={({ field }) => (
                <Input
                  {...field}
                  placeholder="VALUE"
                  overrides={{
                    Root: {
                      style: {
                        height: "34px",
                        borderRadius: "2px",
                        marginRight: "10px",
                        fontSize: "14px",
                      },
                    },
                    Input: {
                      style: {
                        fontSize: "14px",
                      },
                    },
                  }}
                />
              )}
              name="value"
              control={control}
              rules={{ required: true }}
            />
            <Button
              onClick={_onAdd}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "167px",
                    fontSize: "12px",
                    padding: "0px",
                    fontFamily: "RockfordSansMedium",
                    borderRadius: "2px",
                    marginRight: "10px",
                    backgroundColor: "#630be2",
                    ":hover": {
                      backgroundColor: "#630be2",
                      opacity: 0.8,
                    },
                  },
                },
              }}
            >
              Add
            </Button>
            <Button
              onClick={() => setShowNewVariableInput(false)}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "167px",
                    fontSize: "12px",
                    padding: "0px",
                    fontFamily: "RockfordSansMedium",
                    borderRadius: "2px",
                    color: "#630be2",
                    backgroundColor: "#fff",
                    border: "2px solid #630be2",
                    ":hover": {
                      backgroundColor: "#fff",
                      opacity: 0.6,
                    },
                  },
                },
              }}
            >
              Cancel
            </Button>
          </>
        )}
        {!showNewVariableInput && (
          <>
            <Title></Title>
            <Button
              onClick={() => setShowNewVariableInput(true)}
              shape={SHAPE.pill}
              startEnhancer={() => <Plus size={24} color="#630be2" />}
              overrides={ButtonStyles}
            >
              New Variable
            </Button>
          </>
        )}
      </Header>
      <hr style={{ border: "1px solid #cac9c934", marginTop: 10 }} />
      <VariablesTable>
        {variables.length === 0 && (
          <Placeholder>No Environment Variables</Placeholder>
        )}
        {variables.map((variable) => (
          <VariableRow
            key={uniqueId()}
            variable={variable}
            onRemove={onRemove}
            onEdit={onEdit}
          />
        ))}
      </VariablesTable>
    </div>
  );
};

Variables.defaultProps = {
  variables: [],
};

export default Variables;
