import Project from "./project.ts";
import {
  gql,
  GraphQLClient,
} from "https://raw.githubusercontent.com/ArtCodeStudio/graphql-request/main/mod.ts";

class Client {
  client: GraphQLClient;

  constructor() {
    this.client = new GraphQLClient("http://localhost:5478/graphql");
  }

  newProject(): Project {
    return new Project(this);
  }

  async project(id: string): Promise<Project> {
    const query = gql`
      query Project($id: ID!) {
        project(id: $id) {
          id
          name
        }
      }
    `;
    const { project } = await this.client.request(query, { id });

    const p = new Project(this);
    p.id = project.id;
    return p;
  }

  async projects() {
    const query = gql`
      query Projects {
        projects {
          id
          name
        }
      }
    `;
    const { projects } = await this.client.request(query, {});
    return projects;
  }

  send(query: string, variables: any) {
    return this.client.request(query, variables);
  }
}

export const connect = (): Client => new Client();

export default Client;
