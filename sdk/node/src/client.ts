import Project from "./project";
import { gql, GraphQLClient } from "graphql-request";

class Client {
  client: GraphQLClient;

  constructor() {
    this.client = new GraphQLClient("http://localhost:5478/graphql");
  }

  newProject(): Project {
    return new Project(this);
  }

  async project(id: string) {
    const query = gql`
      query Project($id: ID!) {
        project(id: $id) {
          id
          name
        }
      }
    `;
    const response = await this.client.request(query, { id });
    const p = new Project(this);
    p.id = id;
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
    const projects = await this.client.request<any>(query, {});
    return projects;
  }

  send = async (query: any, variables: any) => {
    const response = await this.client.request<any>(query, variables);
    return response;
  };
}

export const connect = (): Client => new Client();

export default Client;
