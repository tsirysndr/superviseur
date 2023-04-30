import Client from "./client.ts";
import Service from "./service.ts";

class Project {
  client: Client;
  name: string;
  services: Service[];

  constructor(client: Client) {
    this.client = client;
    this.name = "";
    this.services = [];
  }

  withName(name: string): Project {
    this.name = name;
    return this;
  }

  addService(service: Service): Project {
    this.services.push(service);
    return this;
  }

  start(serviceId?: string): void {}

  stop(serviceId?: string): void {}

  restart(serviceId?: string): void {}

  list(): Service[] {
    return [];
  }

  logs(serviceId: string): void {}

  status(serviceId: string): void {}
}

export default Project;
