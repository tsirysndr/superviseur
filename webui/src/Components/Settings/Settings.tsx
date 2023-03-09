import styled from "@emotion/styled";
import { Input } from "baseui/input";
import { Select, Value } from "baseui/select";
import { FC, useState } from "react";
import { services } from "../../Mocks/Services";
import { Checkbox } from "baseui/checkbox";
import { SettingsList } from "../../Types/Settings";

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

const SettingsRow = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 25px;
`;

const InputStyles = {
  Root: {
    style: {
      height: "34px",
      borderRadius: "2px",
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

export interface SettingsProps {
  settings?: SettingsList;
}

const Settings: FC<SettingsProps> = ({ settings }) => {
  const [selectedValue, setSelectedValue] = useState<Value>();
  const [autoRestart, setAutoRestart] = useState(false);
  return (
    <Container>
      {settings!.map((item) => (
        <SettingsRow>
          <SettingsName>{item.name} :</SettingsName>
          <SettingsValue>
            {!item.activable && !item.multi && (
              <Input overrides={InputStyles} value={item.value as any} />
            )}
            {item.activable && (
              <Checkbox
                checked={autoRestart}
                onChange={() => setAutoRestart(!autoRestart)}
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
            {item.multi && (
              <Select
                creatable
                multi
                options={item.initialValues!}
                labelKey="label"
                valueKey="id"
                onChange={({ value }) => setSelectedValue(value)}
                value={selectedValue}
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
                      borderRadius: "2px",
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
          </SettingsValue>
        </SettingsRow>
      ))}
    </Container>
  );
};

Settings.defaultProps = {
  settings: [],
};

export default Settings;
