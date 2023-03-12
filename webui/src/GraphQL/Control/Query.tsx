import { gql } from "@apollo/client";
import { PROCESS_FRAGMENT, SERVICE_FRAGMENT } from "../Fragments";

export const STATUS = gql`
  query Status($id: ID!) {
    status(id: $id) {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const PROCESSES = gql`
  query Processes {
    processes {
      ...ProcessFragment
    }
  }
  ${PROCESS_FRAGMENT}
`;

export const SERVICES = gql`
  query Services {
    services {
      ...ServiceFragment
    }
  }
  ${SERVICE_FRAGMENT}
`;
