import { gql } from "@apollo/client";

export const GET_LOGS = gql`
  query GetLogs($id: ID!) {
    logs(id: $id) {
      lines
    }
  }
`;

export const TAIL = gql`
  query TailLogs($id: ID!) {
    tail(id: $id) {
      lines
    }
  }
`;
