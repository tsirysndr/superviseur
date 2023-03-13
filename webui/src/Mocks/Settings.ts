import { Service } from "../Hooks/GraphQL";
import { Settings } from "../Types/Settings";
import { services } from "./Services";

export const parseIntoSettings = (service: Service): Settings[] => {
  return [
    {
      name: "Name",
      value: service.name,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Command",
      value: service.command,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Working Directory",
      value: service.workingDirectory,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Type",
      value: [
        {
          id: "exec",
          label: "exec",
        },
      ],
      multi: false,
      activable: false,
      initialValues: [
        { id: "exec", label: "exec" } as any,
        { id: "wasm", label: "wasm" } as any,
        { id: "docker", label: "docker" } as any,
      ],
      selectable: true,
    },
    {
      name: "Description",
      value: service.description,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Depends on",
      value: undefined,
      multi: true,
      activable: false,
      initialValues: services
        .filter((x) => x.id !== service.id)
        .map(
          (service) =>
            ({
              id: service.id,
              label: service.name,
            } as any)
        ),
      selectable: true,
    },
    {
      name: "Log File",
      value: service.logFile,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Error File",
      value: service.stderrFile,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "Port",
      value: service.port,
      multi: false,
      activable: false,
      selectable: false,
    },
    {
      name: "AutoRestart",
      value: service.autoRestart,
      multi: false,
      activable: true,
      selectable: false,
    },
  ];
};

export const settings: Settings[] = [
  {
    name: "Name",
    value: "Service A",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Command",
    value: "npm start",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Working Directory",
    value: "/tmp",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Type",
    value: [
      {
        id: "exec",
        label: "exec",
      },
    ],
    multi: false,
    activable: false,
    initialValues: [
      { id: "exec", label: "exec" } as any,
      { id: "wasm", label: "wasm" } as any,
      { id: "docker", label: "docker" } as any,
    ],
    selectable: true,
  },
  {
    name: "Description",
    value: "",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Depends on",
    value: undefined,
    multi: true,
    activable: false,
    initialValues: services.map(
      (service) =>
        ({
          id: service.id,
          label: service.name,
        } as any)
    ),
    selectable: true,
  },
  {
    name: "Log File",
    value: "/tmp/service-a.log",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Error File",
    value: "/tmp/service-a.err",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "Port",
    value: "8080",
    multi: false,
    activable: false,
    selectable: false,
  },
  {
    name: "AutoRestart",
    value: true,
    multi: false,
    activable: true,
    selectable: false,
  },
];
