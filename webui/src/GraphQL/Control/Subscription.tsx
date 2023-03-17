import { gql } from "@apollo/client";
import { SERVICE_FRAGMENT } from "../Fragments";

export const ON_START = gql`
  subscription OnStart {
    onStart {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_STOP = gql`
  subscription OnStop {
    onStop {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_RESTART = gql`
  subscription OnRestart {
    onRestart {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_START_ALL = gql`
  subscription OnStartAll {
    onStartAll {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_STOP_ALL = gql`
  subscription OnStopAll {
    onStopAll {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;

export const ON_RESTART_ALL = gql`
  subscription OnRestartAll {
    onRestartAll {
      payload {
        ...ServiceFragment
      }
    }
  }
  ${SERVICE_FRAGMENT}
`;
