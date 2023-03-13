import {
  GET_PROCESSES,
  GET_SERVICE,
  GET_SERVICES,
  GET_STATUS,
} from "../GraphQL/Control/Query";

export const services = [
  {
    __typename: "Service",
    id: "1",
    name: "Service A",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: ["2", "3", "5"],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
  {
    __typename: "Service",
    id: "2",
    name: "Service B",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: ["6"],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
  {
    __typename: "Service",
    id: "3",
    name: "Service C",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: [],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
  {
    __typename: "Service",
    id: "4",
    name: "Service D",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: [],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
  {
    __typename: "Service",
    id: "5",
    name: "Service E",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: [],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
  {
    __typename: "Service",
    id: "6",
    name: "Service F",
    command: "echo 'Hello World'",
    description: "A simple service",
    namespace: "default",
    type: "exec",
    status: "running",
    dependsOn: [],
    env: ["NODE_ENV=dev", "PORT=3000"],
    autoRestart: true,
    workingDirectory: "/tmp",
    logFile: "/tmp/demo-stdout.log",
    stderrFile: "/tmp/demo-stderr.log",
    port: "3000",
  },
];

export const GetServicesMock = {
  request: {
    query: GET_SERVICES,
  },
  result: {
    data: {
      services,
    },
  },
};

export const GetServiceMock = {
  request: {
    query: GET_SERVICE,
    variables: {
      id: "1",
    },
  },
  result: {
    data: {
      service: {
        __typename: "Service",
        id: "1",
        name: "Service A",
        command: "npm start",
        description: "A simple service",
        namespace: "default",
        type: "exec",
        status: "running",
        dependsOn: ["Service B"],
        env: ["NODE_ENV=dev", "PORT=3000"],
        autoRestart: true,
        workingDirectory: "/tmp",
        logFile: "/tmp/demo-stdout.log",
        stderrFile: "/tmp/demo-stderr.log",
        port: "3000",
      },
    },
  },
};

export const GetProcessesMock = {
  request: {
    query: GET_PROCESSES,
  },
  result: {
    data: {
      processes: [
        {
          __typename: "Process",
          name: "Service A",
          description: "A simple service",
          pid: 123,
          ppid: 1,
          command: "echo 'Hello World'",
          workingDirectory: "/home/user",
          project: "project",
          type: "exec",
          logFile: "/tmp/logs/service-a-out.log",
          stderrFile: "/tmp/logs/service-a-err.log",
          autoRestart: true,
          env: ["ENV=dev"],
        },
      ],
    },
  },
};

export const GetStatusMock = {
  request: {
    query: GET_STATUS,
    variables: {
      id: "1",
    },
  },
  result: {
    data: {
      status: {
        __typename: "Process",
        name: "Service A",
        description: "A simple service",
        pid: 123,
        ppid: 1,
        command: "echo 'Hello World'",
        workingDirectory: "/home/user",
        project: "project",
        type: "exec",
        logFile: "/tmp/logs/service-a-out.log",
        stderrFile: "/tmp/logs/service-a-err.log",
        autoRestart: true,
        env: ["ENV=dev"],
      },
    },
  },
};
