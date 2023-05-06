import { connect } from "../client.ts";

if (import.meta.main) {
  const project = await connect().project("near-suit");
  const services = await project.listServices();
  console.log(services);
}
