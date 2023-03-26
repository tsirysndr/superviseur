import { gql } from "@apollo/client";

export const PROCESS_FRAGMENT = gql`
  fragment ProcessFragment on Process {
    name
    serviceId
    description
    pid
    ppid
    command
    workingDirectory
    project
    type
    logFile
    stderrFile
    autoRestart
    env
    state
    upTime
  }
`;

export const SERVICE_FRAGMENT = gql`
  fragment ServiceFragment on Service {
    id
    name
    command
    description
    namespace
    type
    status
    dependsOn
    env
    autoRestart
    workingDirectory
    logFile
    stderrFile
    port
  }
`;
