import { gql } from "@apollo/client";

export const GET_LOGS = gql`
  query GetLogs($id: ID!, $projectId: ID!) {
    logs(id: $id, projectId: $projectId) {
      lines
    }
  }
`;

export const TAIL = gql`
  query TailLogs($id: ID!, $numLines: Int, $projectId: ID!) {
    tail(id: $id, numLines: $numLines, projectId: $projectId) {
      lines
    }
  }
`;
