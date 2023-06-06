import { atom } from "recoil";

export const currentProjectState = atom({
  key: "currentProjectState",
  default: {
    id: "",
    name: "",
  },
});
