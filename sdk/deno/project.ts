import Client from "./client.ts";
import Service from "./service.ts";

class Project {
  client: Client;
  name: string;
  context: string;
  services: Service[];

  constructor(client: Client) {
    this.client = client;
    this.name = "";
    this.services = [];
    this.context = "";
  }

  withContext(context: string): Project {
    this.context = context;
    return this;
  }

  withName(name: string): Project {
    this.name = name;
    return this;
  }

  withService(service: Service): Project {
    this.services.push(service);
    return this;
  }

  start(serviceId?: string): void {}

  stop(serviceId?: string): void {}

  restart(serviceId?: string): void {}

  listServices(): Service[] {
    return [];
  }

  logs(serviceId: string): void {}

  status(serviceId: string): void {}

  ps() {}

  stdout() {}
}

export default Project;
