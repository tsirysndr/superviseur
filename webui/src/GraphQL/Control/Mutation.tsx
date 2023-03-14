import { gql } from "@apollo/client";
import { PROCESS_FRAGMENT } from "../Fragments";

export const START = gql`
  mutation Start($id: ID!) {
    start(id: $id) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const STOP = gql`
  mutation Stop($id: ID!) {
    stop(id: $id) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const RESTART = gql`
  mutation Restart($id: ID!) {
    restart(id: $id) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const CREATE_ENV_VAR = gql`
  mutation CreateEnvVar($id: ID!, $name: String!, $value: String!) {
    createEnvVar(id: $id, name: $name, value: $value) {
      id
      env
    }
  }
`;

export const DELETE_ENV_VAR = gql`
  mutation DeleteEnvVar($id: ID!, $name: String!) {
    deleteEnvVar(id: $id, name: $name) {
      id
      env
    }
  }
`;

export const UPDATE_ENV_VAR = gql`
  mutation UpdateEnvVar($id: ID!, $name: String!, $value: String!) {
    updateEnvVar(id: $id, name: $name, value: $value) {
      id
      env
    }
  }
`;
