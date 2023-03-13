import { GET_LOGS } from "../GraphQL/Logging/Query";

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
