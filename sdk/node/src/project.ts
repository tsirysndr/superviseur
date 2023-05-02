import Service from "./service";
import Client from "./client";

class Project {
  name: string;
  client: Client;
  context: string;
  services: Service[];

  constructor(client: Client) {
    this.name = "";
    this.services = [];
    this.context = "";
    this.client = client;
  }

  withName(name: string) {
    this.name = name;
    return this;
  }

  withContext(context: string) {
    this.context = context;
    return this;
  }

  withService(service: Service): Project {
    this.services.push(service);
    return this;
  }

  listServices(): Service[] {
    return [];
  }

  logs(service: string) {}

  start(service?: string) {}

  stop(service?: string) {}

  restart(service?: string) {}

  status(service: string) {}

  ps() {}

  processes() {
    this.ps();
  }

  stdout() {}
}

export default Project;
