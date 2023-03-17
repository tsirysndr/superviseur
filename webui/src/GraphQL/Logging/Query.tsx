import { gql } from "@apollo/client";

export const GET_LOGS = gql`
  query GetLogs($id: ID!) {
    logs(id: $id) {
      lines
    }
  }
`;

export const TAIL = gql`
  query TailLogs($id: ID!, $numLines: Int) {
    tail(id: $id, numLines: $numLines) {
      lines
    }
  }
`;
