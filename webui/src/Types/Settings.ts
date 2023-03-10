import { Value } from "baseui/select";

export type SettingsList = Settings[];

export type Settings = {
  name: string;
  value?: string | boolean | Value;
  multi: boolean;
  selectable: boolean;
  activable: boolean;
  initialValues?: Value[];
};
