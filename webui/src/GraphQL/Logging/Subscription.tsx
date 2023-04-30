import { gql } from "@apollo/client";

export const LOGS = gql`
  subscription Logs($id: ID!, $projectId: ID!) {
    logs(id: $id, projectId: $projectId) {
      line
    }
  }
`;

export const TAIL = gql`
  subscription Tail($id: ID!, $projectId: ID!) {
    tail(id: $id, projectId: $projectId) {
      line
    }
  }
`;
