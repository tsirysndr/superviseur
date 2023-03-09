import styled from "@emotion/styled";
import { Input } from "baseui/input";
import { Select, Value } from "baseui/select";
import { FC, useState } from "react";
import { Checkbox } from "baseui/checkbox";
import { SettingsList, Settings as SettingsData } from "../../Types/Settings";
import { useForm, Controller } from "react-hook-form";
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
};

export interface SettingsRowProps {
  settings: SettingsData;
}

const SettingsRow: FC<SettingsRowProps> = ({ settings }) => {
  const { control, handleSubmit } = useForm({
    defaultValues: {
      [settings.name]: settings.value,
    },
  });
  const [selectedValue, setSelectedValue] = useState<Value>();
  const [autoRestart, setAutoRestart] = useState(false);

  return (
    <SettingsRowContainer>
      <SettingsName>{settings.name} :</SettingsName>
      <SettingsValue>
        {!settings.activable && !settings.multi && (
          <Controller
            render={({ field }) => (
              <Input {...(field as any)} overrides={InputStyles} />
            )}
            control={control}
            name={settings.name}
            defaultValue={settings.value}
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
  settings?: SettingsList;
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
