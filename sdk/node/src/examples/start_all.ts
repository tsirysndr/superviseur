import { connect } from "../client";

async function main() {
  const project = await connect().project("obese-ants");
  const response = await project.start();
  console.log(response);
}

main();
