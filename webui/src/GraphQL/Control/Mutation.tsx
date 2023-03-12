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
