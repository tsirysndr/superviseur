import {
  CREATE_ENV_VAR,
  DELETE_ENV_VAR,
  UPDATE_ENV_VAR,
} from "../GraphQL/Control/Mutation";

export const variables = [
  {
    name: "NODE_ENV",
    value: "production",
  },
  {
    name: "PORT",
    value: "3000",
  },
];

export const CreateEnvVarMock = {
  request: {
    query: CREATE_ENV_VAR,
    variables: {
      id: "1",
      name: "LOG_LEVEL",
      value: "debug",
    },
  },
  result: {
    data: {
      createEnvVar: {
        __typename: "Service",
        id: "1",
        env: ["NODE_ENV=dev", "PORT=3000", "LOG_LEVEL=debug"],
      },
    },
  },
};

export const UpdateEnvVarMock = [
  {
    request: {
      query: UPDATE_ENV_VAR,
      variables: {
        id: "1",
        name: "NODE_ENV",
        value: "production",
      },
    },
    result: {
      data: {
        updateEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=production", "PORT=3000"],
        },
      },
    },
  },
  {
    request: {
      query: UPDATE_ENV_VAR,
      variables: {
        id: "1",
        name: "NODE_ENV",
        value: "dev",
      },
    },
    result: {
      data: {
        updateEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=dev", "PORT=3000"],
        },
      },
    },
  },
  {
    request: {
      query: UPDATE_ENV_VAR,
      variables: {
        id: "1",
        name: "PORT",
        value: "3000",
      },
    },
    result: {
      data: {
        updateEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=production", "PORT=3000"],
        },
      },
    },
  },
  {
    request: {
      query: UPDATE_ENV_VAR,
      variables: {
        id: "1",
        name: "LOG_LEVEL",
        value: "info",
      },
    },
    result: {
      data: {
        updateEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=production", "PORT=3000", "LOG_LEVEL=info"],
        },
      },
    },
  },
];

export const DeleteEnvVarMock = [
  {
    request: {
      query: DELETE_ENV_VAR,
      variables: {
        id: "1",
        name: "NODE_ENV",
      },
    },
    result: {
      data: {
        deleteEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["PORT=3000"],
        },
      },
    },
  },
  {
    request: {
      query: DELETE_ENV_VAR,
      variables: {
        id: "1",
        name: "PORT",
      },
    },
    result: {
      data: {
        deleteEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=production"],
        },
      },
    },
  },
  {
    request: {
      query: DELETE_ENV_VAR,
      variables: {
        id: "1",
        name: "LOG_LEVEL",
      },
    },
    result: {
      data: {
        deleteEnvVar: {
          __typename: "Service",
          id: "1",
          env: ["NODE_ENV=production", "PORT=3000"],
        },
      },
    },
  },
];
