import { gql } from "@apollo/client";

export const LOGS = gql`
  subscription Logs($id: ID!) {
    logs(id: $id) {
      line
    }
  }
`;

export const TAIL = gql`
  subscription Tail($id: ID!) {
    tail(id: $id) {
      line
    }
  }
`;
