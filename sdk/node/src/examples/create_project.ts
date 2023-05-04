import Project from "../project";
import Service from "../service";
import { connect } from "../client";

function main() {
  const deno = new Service().withName("deno-fresh").withCommand("./dev.ts");

  connect()
    .newProject()
    .withName("deno-example")
    .withContext(
      "/Users/tsirysandratraina/Documents/GitHub/superviseur/examples/deno-fresh"
    )
    .withService(deno)
    .stdout();
}

main();
