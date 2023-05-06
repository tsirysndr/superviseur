import Project from "../project";
import Service from "../service";
import path from "path";
import { connect } from "../client";

function main() {
  const deno = new Service()
    .withName("deno-fresh")
    .withCommand("./dev.ts")
    .withEnv({
      PORT: "8000",
    });

  const projectDir = path.resolve("../../examples/deno-fresh");

  connect()
    .newProject()
    .withName("deno-example")
    .withContext(projectDir)
    .withService(deno)
    .stdout();
}

main();
