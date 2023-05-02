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
  enableDocker?: boolean;
  enableNix?: boolean;

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

  withAutoRestart(autoRestart: boolean) {
    this.autorestart = autoRestart;
    return this;
  }

  withEnableDocker(enableDocker: boolean) {
    this.enableDocker = enableDocker;
    return this;
  }

  withEnableNix(enableNix: boolean) {
    this.enableNix = enableNix;
    return this;
  }
}

export default Service;
