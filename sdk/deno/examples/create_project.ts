import { connect } from "../client.ts";
import Service from "../service.ts";

if (import.meta.main) {
  const deno = new Service().withName("deno-fresh").withCommand("./dev.ts");

  const response = await connect()
    .newProject()
    .withName("deno-example")
    .withContext(
      "/Users/tsirysandratraina/Documents/GitHub/superviseur/examples/deno-fresh"
    )
    .withService(deno)
    .stdout();
  console.log(response);
}
