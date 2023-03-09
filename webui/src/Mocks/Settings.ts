import { Settings } from "../Types/Settings";
import { services } from "./Services";

export const settings: Settings[] = [
  {
    name: "Name",
    value: "Service A",
    multi: false,
    activable: false,
  },
  {
    name: "Command",
    value: "npm start",
    multi: false,
    activable: false,
  },
  {
    name: "Working Directory",
    value: "/tmp",
    multi: false,
    activable: false,
  },
  {
    name: "Type",
    value: "exec",
    multi: false,
    activable: false,
  },
  {
    name: "Description",
    value: "",
    multi: false,
    activable: false,
  },
  {
    name: "Depends on",
    value: undefined,
    multi: true,
    activable: false,
    initialValues: services.nodes.map(
      (service) =>
        ({
          id: service.id,
          label: service.label.split("<b>")[1].replace("</b>", ""),
        } as any)
    ),
  },
  {
    name: "Log File",
    value: "/tmp/service-a.log",
    multi: false,
    activable: false,
  },
  {
    name: "Error File",
    value: "/tmp/service-a.err",
    multi: false,
    activable: false,
  },
  {
    name: "Port",
    value: "8080",
    multi: false,
    activable: false,
  },
  {
    name: "AutoRestart",
    value: true,
    multi: false,
    activable: true,
  },
];
