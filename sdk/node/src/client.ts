import Project from "./project";

class Client {
  newProject(): Project {
    return new Project(this);
  }

  project(id: string): Project {
    return new Project(this);
  }

  projects(): Project[] {
    return [];
  }
}

export const connect = (): Client => new Client();

export default Client;
