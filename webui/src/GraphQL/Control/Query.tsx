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
  query GetServices {
    services {
      ...ServiceFragment
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const GET_SERVICE = gql`
  query GetService($id: ID!) {
    service(id: $id) {
      ...ServiceFragment
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const GET_ENV_VARS = gql`
  query GetEnvVars($id: ID!) {
    service(id: $id) {
      id
      env
    }
  }
`;
