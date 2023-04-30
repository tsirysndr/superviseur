class Service {
  name: string;
  command: string;
  execType: string;
  workingDir: string;
  description: string;
  env: Map<string, string>;
  autostart: boolean;
  autorestart: boolean;
  namespace: string;
  stdout: string;
  stderr: string;
  buildCommand: string;
  floxEnvironment: string;

  constructor() {
    this.name = "";
    this.command = "";
    this.description = "";
  }

  withName(name: string) {
    this.name = name;
    return this;
  }

  withCommand(command: string) {
    this.command = command;
    return this;
  }

  withDescription(description: string) {
    this.description = description;
    return this;
  }
}

export default Service;
