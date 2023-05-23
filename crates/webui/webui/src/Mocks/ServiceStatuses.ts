import { Process } from "../Hooks/GraphQL";
import { ServiceStatus } from "../Types/ServiceStatus";
import dayjs from "dayjs";
import LocalizedFormat from "dayjs/plugin/localizedFormat";
import RelativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(LocalizedFormat);
dayjs.extend(RelativeTime);

const parseActiveStatus = (process: Process): string => {
  // ${process.uptime} seconds ago
  if (process.state === "Running") {
    const date = dayjs(process.upTime);
    return `Running since ${date.format("llll")}; ${date.fromNow()}`;
  }
  return process.state;
};

export const parseIntoStatuses = (process: Process): ServiceStatus[] => {
  return [
    {
      name: "Active",
      status: parseActiveStatus(process),
    },
    {
      name: "PID",
      status: process.state === "Stopped" ? "-" : process.pid || "-",
    },
    {
      name: "Command",
      status: process.command,
    },
    {
      name: "Directory",
      status: process.workingDirectory || "-",
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
