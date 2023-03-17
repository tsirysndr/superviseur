import { GET_LOGS, TAIL } from "../GraphQL/Logging/Query";

export const lines = [
  "/dev/rdisk1s5s1: fsck_apfs started at Wed Feb 15 20:38:24 2023",
  "/dev/rdisk1s5s1: ** QUICKCHECK ONLY; FILESYSTEM CLEAN",
  "/dev/rdisk1s5s1: fsck_apfs completed at Wed Feb 15 20:38:24 2023",
];

export const GetLogsMocks = {
  request: {
    query: GET_LOGS,
    variables: {
      id: "1",
    },
  },
  result: {
    data: {
      logs: {
        __typename: "Log",
        lines,
      },
    },
  },
};

export const TailLogsMocks = [
  {
    request: {
      query: TAIL,
      variables: {
        id: "1",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
  {
    request: {
      query: TAIL,
      variables: {
        id: "2",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
  {
    request: {
      query: TAIL,
      variables: {
        id: "3",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
  {
    request: {
      query: TAIL,
      variables: {
        id: "4",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
  {
    request: {
      query: TAIL,
      variables: {
        id: "5",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
  {
    request: {
      query: TAIL,
      variables: {
        id: "6",
      },
    },
    result: {
      data: {
        logs: {
          __typename: "Log",
          lines,
        },
      },
    },
  },
];
