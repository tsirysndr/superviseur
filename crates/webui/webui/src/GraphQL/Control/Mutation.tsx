import { gql } from "@apollo/client";
import { PROCESS_FRAGMENT } from "../Fragments";

export const START = gql`
  mutation Start($id: ID, $projectId: ID!) {
    start(id: $id, projectId: $projectId) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const STOP = gql`
  mutation Stop($id: ID, $projectId: ID!) {
    stop(id: $id, projectId: $projectId) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const RESTART = gql`
  mutation Restart($id: ID, $projectId: ID!) {
    restart(id: $id, projectId: $projectId) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const CREATE_ENV_VAR = gql`
  mutation CreateEnvVar(
    $id: ID!
    $name: String!
    $value: String!
    $projectId: ID!
  ) {
    createEnvVar(id: $id, name: $name, value: $value, projectId: $projectId) {
      id
      env
    }
  }
`;

export const DELETE_ENV_VAR = gql`
  mutation DeleteEnvVar($id: ID!, $name: String!, $projectId: ID!) {
    deleteEnvVar(id: $id, name: $name, projectId: $projectId) {
      id
      env
    }
  }
`;

export const UPDATE_ENV_VAR = gql`
  mutation UpdateEnvVar(
    $id: ID!
    $name: String!
    $value: String!
    $projectId: ID!
  ) {
    updateEnvVar(id: $id, name: $name, value: $value, projectId: $projectId) {
      id
      env
    }
  }
`;
