import { connect } from "../client.ts";

if (import.meta.main) {
  connect().projects();
}
