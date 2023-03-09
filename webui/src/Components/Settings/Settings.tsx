import styled from "@emotion/styled";
import { Input } from "baseui/input";
import { Select, Value } from "baseui/select";
import { FC, useEffect, useState } from "react";
import { Checkbox } from "baseui/checkbox";
import { SettingsList, Settings as SettingsData } from "../../Types/Settings";
import { useForm, Controller } from "react-hook-form";
import { Check2 } from "@styled-icons/bootstrap/Check2";
import { CloseOutline } from "@styled-icons/evaicons-outline/CloseOutline";
import _ from "lodash";

const Container = styled.div`
  padding-top: 20px;
  height: 100%;
  overflow: auto;
`;

const SettingsName = styled.div`
  flex: 0.5;
  display: flex;
  justify-content: flex-end;
  margin-right: 15px;
`;

const SettingsValue = styled.div`
  flex: 1;
`;

const SettingsRowContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 25px;
`;

const CheckButton = styled.button`
  border: none;
  background-color: transparent;
  cursor: pointer;
`;

const InputStyles = {
  Root: {
    style: {
      height: "34px",
      borderTop: "none",
      borderLeft: "none",
      borderRight: "none",
      borderRadius: "0px",
      backgroundColor: "#fff",
    },
  },
  Input: {
    style: {
      fontSize: "14px",
    },
  },
  InputContainer: {
    style: {
      backgroundColor: "#fff",
    },
  },
  EndEnhancer: {
    style: {
      backgroundColor: "#fff",
    },
  },
};

export interface SettingsRowProps {
  settings: SettingsData;
}

const SettingsRow: FC<SettingsRowProps> = ({ settings }) => {
  const [showSaveButtons, setShowSaveButtons] = useState<boolean>(false);
  const { control, handleSubmit, reset, watch } = useForm({
    defaultValues: {
      [settings.name]: settings.value,
    },
  });

  useEffect(() => {
    const subscription = watch((value, { name, type }) => {
      if (type === "change") {
        setShowSaveButtons(true);
      }
    });
    return () => subscription.unsubscribe();
  }, [watch]);

  const onSave = () => {
    handleSubmit(
      (data) => {
        console.log(data);
        setShowSaveButtons(false);
      },
      (x) => {
        console.log(">>", x);
      }
    )();
  };

  const onDiscard = () => {
    setShowSaveButtons(false);
    reset({
      [settings.name]: settings.value,
    });
  };

  return (
    <SettingsRowContainer>
      <SettingsName>{settings.name} :</SettingsName>
      <SettingsValue>
        {!settings.activable && !settings.multi && (
          <Controller
            render={({ field }) => (
              <Input
                {...(field as any)}
                overrides={InputStyles}
                endEnhancer={() => (
                  <>
                    {showSaveButtons && (
                      <>
                        <CheckButton onClick={onSave}>
                          <Check2 size={20} color="#000" />
                        </CheckButton>
                        <CheckButton onClick={onDiscard}>
                          <CloseOutline size={20} color="#000" />
                        </CheckButton>
                      </>
                    )}
                  </>
                )}
              />
            )}
            control={control}
            name={settings.name}
            defaultValue={settings.value}
            rules={{ required: true }}
          />
        )}
        {settings.activable && (
          <Controller
            render={({ field }) => (
              <Checkbox
                checked={field.value as boolean}
                onChange={() => field.onChange(!field.value)}
                overrides={{
                  Label: {
                    style: {
                      color: "#b03aff",
                    },
                  },
                  Checkmark: {
                    style: ({ $checked }) => ({
                      borderLeftColor: "#b03aff",
                      borderRightColor: "#b03aff",
                      borderTopColor: "#b03aff",
                      borderBottomColor: "#b03aff",
                      backgroundColor: $checked ? "#b03aff" : null,
                    }),
                  },
                }}
              />
            )}
            control={control}
            name={settings.name}
            defaultValue={settings.value}
          />
        )}
        {settings.multi && (
          <Controller
            render={({ field }) => (
              <Select
                {...(field as any)}
                creatable
                multi
                options={settings.initialValues!}
                labelKey="label"
                valueKey="id"
                onChange={({ value }) => {
                  field.onChange(value);
                }}
                overrides={{
                  Tag: {
                    props: {
                      overrides: {
                        Root: {
                          style: {
                            backgroundColor: "#b03aff",
                          },
                        },
                      },
                    },
                  },
                  ControlContainer: {
                    style: {
                      minHeight: "34px",
                      borderTop: "none",
                      borderLeft: "none",
                      borderRight: "none",
                      borderRadius: "0px",
                      backgroundColor: "#fff",
                      padding: "0px",
                      paddingLeft: "10px",
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
            control={control}
            name={settings.name}
            defaultValue={settings.value}
          />
        )}
      </SettingsValue>
    </SettingsRowContainer>
  );
};

export interface SettingsProps {
  settings: SettingsList;
  onSave: (data: SettingsData) => void;
}

const Settings: FC<SettingsProps> = ({ settings }) => {
  return (
    <Container>
      {settings!.map((item) => (
        <SettingsRow settings={item} key={_.uniqueId()} />
      ))}
    </Container>
  );
};

Settings.defaultProps = {
  settings: [],
};

export default Settings;
