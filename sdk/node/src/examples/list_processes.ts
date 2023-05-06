import { connect } from "../client";

async function main() {
  const project = await connect().project("obese-ants");
  await project.processes();
}

main();
