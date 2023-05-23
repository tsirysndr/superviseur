import { gql } from "@apollo/client";
import { PROCESS_FRAGMENT, SERVICE_FRAGMENT } from "../Fragments";

export const GET_STATUS = gql`
  query GetStatus($id: ID!) {
    status(id: $id) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const GET_PROCESSES = gql`
  query GetProcesses {
    processes {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const GET_SERVICES = gql`
  query GetServices($projectId: ID!) {
    services(projectId: $projectId) {
      ...ServiceFragment
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const GET_SERVICE = gql`
  query GetService($id: ID!, $projectId: ID!) {
    service(id: $id, projectId: $projectId) {
      ...ServiceFragment
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const GET_ENV_VARS = gql`
  query GetEnvVars($id: ID!, $projectId: ID!) {
    service(id: $id, projectId: $projectId) {
      id
      env
    }
  }
`;

export const GET_PROJECTS = gql`
  query GetProjects {
    projects {
      id
      name
      configPath
    }
  }
`;

export const GET_PROJECT = gql`
  query GetProject($id: ID!) {
    project(id: $id) {
      id
      name
      configPath
    }
  }
`;
