import { Process } from "../Hooks/GraphQL";
import { ServiceStatus } from "../Types/ServiceStatus";

export const parseIntoStatuses = (process: Process): ServiceStatus[] => {
  return [
    {
      name: "Active",
      status: "Running since 2023-03-05 19:17:56.512455 UTC; 17 seconds ago",
    },
    {
      name: "PID",
      status: process.pid!,
    },
    {
      name: "Command",
      status: process.command,
    },
    {
      name: "Directory",
      status: process.workingDirectory,
    },
    {
      name: "Log",
      status: process.logFile,
    },
    {
      name: "Errors",
      status: process.stderrFile,
    },
    {
      name: "AutoRestart",
      status: process.autoRestart,
    },
    {
      name: "Type",
      status: process.type,
    },
  ];
};

export const statuses: ServiceStatus[] = [
  {
    name: "Active",
    status: "Running since 2023-03-05 19:17:56.512455 UTC; 17 seconds ago",
  },
  {
    name: "PID",
    status: "1234",
  },
  {
    name: "Command",
    status: "npm start",
  },
  {
    name: "Directory",
    status: "/home/username/website",
  },
  {
    name: "Log",
    status: "/tmp/demo-stdout.log",
  },
  {
    name: "Errors",
    status: "/tmp/demo-stderr.log",
  },
  {
    name: "AutoRestart",
    status: "true",
  },
  {
    name: "Type",
    status: "exec",
  },
];
