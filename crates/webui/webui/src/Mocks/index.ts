import { GetLogsMocks, TailLogsMocks } from "./Lines";
import {
  GetProcessesMock,
  GetServiceMock,
  GetServicesMock,
  GetStatusMock,
} from "./Services";
import {
  CreateEnvVarMock,
  DeleteEnvVarMock,
  UpdateEnvVarMock,
} from "./Variables";

export const mocks = [
  GetServicesMock,
  ...GetServiceMock,
  GetProcessesMock,
  ...GetStatusMock,
  GetLogsMocks,
  CreateEnvVarMock,
  ...UpdateEnvVarMock,
  ...DeleteEnvVarMock,
  ...TailLogsMocks,
];
