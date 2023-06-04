import { atom } from "recoil";
import PostgreSQL from "../../../Images/postgresql.svg";
import Redis from "../../../Images/redis.svg";
import MongoDB from "../../../Images/mongodb.svg";
import MySQL from "../../../Images/mysql.svg";
import Fresh from "../../../Images/fresh.svg";
import NodeJS from "../../../Images/nodejs.svg";
import Go from "../../../Images/go.svg";
import Hono from "../../../Images/hono.png";
import Bun from "../../../Images/bun.svg";
import Deno from "../../../Images/deno.svg";
import Wasm from "../../../Images/wasm.svg";

export const newServiceModalState = atom({
  key: "newServiceState",
  default: {
    templates: [
      {
        id: "4c02014d-7c42-40b0-b5ac-466e84535e13",
        name: "Deno Fresh",
        tags: ["flox"],
        icon: Fresh,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0a",
        name: "Hono",
        tags: ["flox"],
        icon: Hono,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0b",
        name: "Bun Server",
        tags: ["nix"],
        icon: Bun,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0c",
        name: "NodeJS, Redis",
        tags: ["devenv"],
        icon: NodeJS,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0d",
        name: "Go, MySQL",
        tags: ["devbox"],
        icon: Go,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0e",
        name: "Spin HTTP Server",
        tags: ["nix"],
        icon: Wasm,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0f",
        name: "Deno",
        tags: ["devbox"],
        icon: Deno,
      },
    ],
    databases: [
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0g",
        name: "Add PostgreSQL",
        tags: ["docker"],
        icon: PostgreSQL,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0h",
        name: "Add Redis",
        tags: ["docker"],
        icon: Redis,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0i",
        name: "Add MongoDB",
        tags: ["docker"],
        icon: MongoDB,
      },
      {
        id: "b4f5b0a0-0b0a-4b0a-8b0a-0b0a0b0a0b0j",
        name: "Add MySQL",
        tags: ["docker"],
        icon: MySQL,
      },
    ],
    selectedTemplate: null,
    selectedDatabase: null,
  },
});
