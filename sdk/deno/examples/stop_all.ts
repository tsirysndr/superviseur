import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("near-suit");
  const response = await project.stop();
  console.log(response);
}
