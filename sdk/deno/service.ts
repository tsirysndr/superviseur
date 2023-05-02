class Service {
  name: string;
  command: string;
  execType: string;
  dependsOn: string[];
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
    this.execType = "";
    this.dependsOn = [];
    this.workingDir = "";
    this.description = "";
    this.env = new Map();
    this.autostart = false;
    this.autorestart = false;
    this.namespace = "";
    this.stdout = "";
    this.stderr = "";
    this.buildCommand = "";
    this.floxEnvironment = "";
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

  withEnv(env: Map<string, string>): Service {
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
}

export default Service;
