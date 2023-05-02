import { connect } from "../Client";

function main() {
  connect().project("deno-example").listServices();
}

main();
