import { connect } from "../client";

async function main() {
  const project = await connect().project("obese-ants");
  const response = await project.restart();
  console.log(response);
}

main();
