import { gql } from "@apollo/client";

export const PROCESS_FRAGMENT = gql`
  fragment ProcessFragment on Process {
    name
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
  }
`;

export const SERVICE_FRAGMENT = gql`
  fragment ServiceFragment on Service {
    name
    command
    description
    namespace
    type
    status
    dependsOn
    env
    autoRestart
  }
`;
