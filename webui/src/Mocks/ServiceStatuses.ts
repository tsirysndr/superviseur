import { ServiceStatus } from "../Types/ServiceStatus";

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
    name: "Stderr",
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
