import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("obese-ants");
  const processes = await project.processes();
  console.log(processes);
}
