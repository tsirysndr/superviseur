import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("obese-ants");
  await project.stop("deno");
}
