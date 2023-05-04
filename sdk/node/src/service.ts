class Service {
  name: string;
  command: string;
  execType?: string;
  workingDir?: string;
  description?: string;
  env: { [key: string]: string };
  dependsOn?: string[];
  autostart?: boolean;
  autorestart?: boolean;
  namespace?: string;
  stdout?: string;
  stderr?: string;
  buildCommand?: string;
  floxEnvironment?: string;
  enableDocker?: boolean;
  enableNix?: boolean;

  constructor() {
    this.name = "";
    this.command = "";
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

  withDependsOn(dependsOn: string[]) {
    this.dependsOn = dependsOn;
    return this;
  }

  withWorkingDir(workingDir: string) {
    this.workingDir = workingDir;
    return this;
  }

  withEnv(env: { [key: string]: string }) {
    this.env = env;
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
