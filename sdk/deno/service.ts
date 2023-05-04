class Service {
  name: string;
  command: string;
  execType?: string;
  dependsOn: string[];
  workingDir?: string;
  description?: string;
  env: { [key: string]: string };
  autostart: boolean;
  autorestart: boolean;
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
    this.dependsOn = [];
    this.env = {};
    this.autostart = false;
    this.autorestart = false;
  }

  withName(name: string): Service {
    this.name = name;
    return this;
  }

  withCommand(command: string): Service {
    this.command = command;
    return this;
  }

  withExecType(execType: string): Service {
    this.execType = execType;
    return this;
  }

  withDependsOn(dependsOn: string[]): Service {
    this.dependsOn = dependsOn;
    return this;
  }

  withWorkingDir(workingDir: string): Service {
    this.workingDir = workingDir;
    return this;
  }

  withDescription(description: string): Service {
    this.description = description;
    return this;
  }

  withEnv(env: { [key: string]: string }): Service {
    this.env = env;
    return this;
  }

  withAutostart(autostart: boolean): Service {
    this.autostart = autostart;
    return this;
  }

  withAutorestart(autorestart: boolean): Service {
    this.autorestart = autorestart;
    return this;
  }

  withNamespace(namespace: string): Service {
    this.namespace = namespace;
    return this;
  }

  withStdout(stdout: string): Service {
    this.stdout = stdout;
    return this;
  }

  withStderr(stderr: string): Service {
    this.stderr = stderr;
    return this;
  }

  withBuildCommand(buildCommand: string): Service {
    this.buildCommand = buildCommand;
    return this;
  }

  withFloxEnvironment(floxEnvironment: string): Service {
    this.floxEnvironment = floxEnvironment;
    return this;
  }

  withEnableDocker(enableDocker: boolean): Service {
    this.enableDocker = enableDocker;
    return this;
  }

  withEnableNix(enableNix: boolean): Service {
    this.enableNix = enableNix;
    return this;
  }
}

export default Service;
