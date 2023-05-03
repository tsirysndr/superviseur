import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("obese-ants");
  const status = await project.status("happy-poison");
  console.log(status);
}
