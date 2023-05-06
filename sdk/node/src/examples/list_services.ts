import { connect } from "../client";

async function main() {
  const project = await connect().project("obese-ants");
  const services = await project.listServices();
  console.log(services);
}

main();
