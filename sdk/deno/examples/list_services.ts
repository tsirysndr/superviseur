import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("obese-ants");
  const services = await project.listServices();
  console.log(services);
}
