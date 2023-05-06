import * as path from "https://deno.land/std/path/mod.ts";
import { connect } from "../client.ts";
import Service from "../service.ts";

if (import.meta.main) {
  const deno = new Service()
    .withName("deno-fresh")
    .withCommand("./dev.ts")
    .withEnv({
      PORT: "8000",
    });

  const projectDir = path.resolve("../../examples/deno-fresh");

  const response = await connect()
    .newProject()
    .withName("deno-example")
    .withContext(projectDir)
    .withService(deno)
    .stdout();
  console.log(response);
}
