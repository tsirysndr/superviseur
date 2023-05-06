import { connect } from "../client";

async function main() {
  const projects = await connect().projects();
  console.log(projects);
}

main();
