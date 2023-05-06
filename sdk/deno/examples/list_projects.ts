import { connect } from "../client.ts";

if (import.meta.main) {
  const projects = await connect().projects();
  console.log(projects);
}
