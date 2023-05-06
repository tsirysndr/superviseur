import Client from "./client.ts";
import { buildNestedWithServiceQuery } from "./query.ts";
import Service from "./service.ts";
import { gql } from "https://raw.githubusercontent.com/ArtCodeStudio/graphql-request/main/mod.ts";

class Project {
  client: Client;
  id?: string;
  name: string;
  context: string;
  services: Service[];

  constructor(client: Client) {
    this.client = client;
    this.name = "";
    this.services = [];
    this.context = "";
  }

  withContext(context: string): Project {
    this.context = context;
    return this;
  }

  withName(name: string): Project {
    this.name = name;
    return this;
  }

  withService(service: Service): Project {
    this.services.push(service);
    return this;
  }

  async start(serviceId?: string) {
    const query = gql`
      mutation StartService($id: ID, $projectId: ID!) {
        start(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: serviceId,
      projectId: this.id,
    });
    return response.start;
  }

  async stop(serviceId?: string) {
    const query = gql`
      mutation StopService($id: ID, $projectId: ID!) {
        stop(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: serviceId,
      projectId: this.id,
    });
    return response.stop;
  }

  async restart(serviceId?: string) {
    const query = gql`
      mutation RestartService($id: ID, $projectId: ID!) {
        restart(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: serviceId,
      projectId: this.id,
    });
    return response.restart;
  }

  async listServices() {
    const query = gql`
      query Services($projectId: ID!) {
        services(projectId: $projectId) {
          id
          name
          command
          status
        }
      }
    `;
    const response = await this.client.send(query, {
      projectId: this.id,
    });
    return response.services;
  }

  async logs(serviceId: string) {
    const query = gql`
      query Logs($id: ID!, $projectId: ID!) {
        logs(id: $id, projectId: $projectId) {
          lines
        }
      }
    `;
    const response = await this.client.send(query, {
      id: serviceId,
      projectId: this.id,
    });
    return response.logs;
  }

  async status(serviceId: string) {
    const query = gql`
      query Status($id: ID!) {
        status(id: $id) {
          name
          serviceId
          pid
          project
          state
          upTime
        }
      }
    `;
    const response = await this.client.send(query, {
      id: serviceId,
    });
    return response.status;
  }

  async ps() {
    const query = gql`
      query Processes {
        processes {
          name
          pid
          project
          serviceId
          command
          upTime
        }
      }
    `;
    const response = await this.client.send(query, {});
    return response.processes;
  }

  processes = this.ps;

  async stdout() {
    if (this.services.length === 0) {
      throw new Error("Project must have at least one service");
    }
    const nestedQuery = buildNestedWithServiceQuery(this.services);
    const query = gql`
      mutation NewProject($name: String!, $context: String!) {
        newProject(name: $name, context: $context) {
         ${nestedQuery}
        }
      }
    `;
    const response = await this.client.send(query, {
      name: this.name,
      context: this.context,
    });
    return response.newProject;
  }

  async addEnvVariable(serviceId: string, name: string, value: string) {
    const query = gql`
      mutation CreateEnvVar(
        $projectId: ID!
        $id: ID!
        $name: String!
        $value: String!
      ) {
        createEnvVar(
          projectId: $projectId
          id: $id
          name: $name
          value: $value
        ) {
          id
          env
        }
      }
    `;
    const response = await this.client.send(query, {
      projectId: this.id,
      id: serviceId,
      name,
      value,
    });
    return response.createEnvVar;
  }

  async removeEnvVariable(serviceId: string, name: string) {
    const query = gql`
      mutation DeleteEnvVar($projectId: ID!, $id: ID!, $name: String!) {
        deleteEnvVar(projectId: $projectId, id: $id, name: $name) {
          id
          env
        }
      }
    `;
    const response = await this.client.send(query, {
      projectId: this.id,
      id: serviceId,
      name,
    });
    return response.deleteEnvVar;
  }

  async updateEnvVariable(serviceId: string, name: string, value: string) {
    const query = gql`
      mutation UpdateEnvVar(
        $projectId: ID!
        $id: ID!
        $name: String!
        $value: String!
      ) {
        updateEnvVar(
          projectId: $projectId
          id: $id
          name: $name
          value: $value
        ) {
          id
          env
        }
      }
    `;
    const response = await this.client.send(query, {
      projectId: this.id,
      id: serviceId,
      name,
      value,
    });
    return response.updateEnvVar;
  }
}

export default Project;
