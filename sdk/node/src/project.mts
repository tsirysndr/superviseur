import Service from "./service.mjs";

class Project {
  name: string;
  services: Service[];

  constructor() {
    this.name = "";
    this.services = [];
  }

  withName(name: string) {
    this.name = name;
    return this;
  }

  addService(service: Service): Project {
    this.services.push(service);
    return this;
  }

  list(): Service[] {
    return [];
  }

  logs(service: string) {}

  start(service?: string) {}

  stop(service?: string) {}

  restart(service?: string) {}

  status(service: string) {}
}

export default Project;
