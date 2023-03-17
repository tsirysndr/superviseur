import { gql } from "@apollo/client";
import { SERVICE_FRAGMENT } from "../Fragments";

export const ON_START = gql`
  subscription OnStart {
    onStart {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_STOP = gql`
  subscription OnStop {
    onStop {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_RESTART = gql`
  subscription OnRestart {
    onRestart {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_START_ALL = gql`
  subscription OnStartAll {
    onStartAll {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_STOP_ALL = gql`
  subscription OnStopAll {
    onStopAll {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_RESTART_ALL = gql`
  subscription OnRestartAll {
    onRestartAll {
      payload {
        id
        name
        status
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;
