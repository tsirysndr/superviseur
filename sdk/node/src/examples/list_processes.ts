import { connect } from "../client";

function main() {
  connect().project("deno-example").ps();
}

main();
