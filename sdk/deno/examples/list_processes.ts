import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("near-suit");
  const processes = await project.processes();
  console.log(processes);
}
