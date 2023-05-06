import Service from "./service";
import Client from "./client";
import { buildNestedWithServiceQuery } from "./query";
import { gql } from "graphql-request";

class Project {
  id?: string;
  name: string;
  client: Client;
  context: string;
  services: Service[];

  constructor(client: Client) {
    this.name = "";
    this.services = [];
    this.context = "";
    this.client = client;
  }

  withName(name: string) {
    this.name = name;
    return this;
  }

  withContext(context: string) {
    this.context = context;
    return this;
  }

  withService(service: Service): Project {
    this.services.push(service);
    return this;
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
    const response = await this.client.send(query, { projectId: this.id });
    return response.services;
  }

  async logs(service: string) {
    const query = gql`
      query Logs($id: ID!, $projectId: ID!) {
        logs(id: $id, projectId: $projectId) {
          lines
        }
      }
    `;
    const response = await this.client.send(query, {
      id: service,
      projectId: this.id,
    });
    return response.logs;
  }

  async start(service?: string) {
    const query = gql`
      mutation StartService($id: ID, $projectId: ID!) {
        start(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: service,
      projectId: this.id,
    });
    return response.start;
  }

  async stop(service?: string) {
    const query = gql`
      mutation StopService($id: ID, $projectId: ID!) {
        stop(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: service,
      projectId: this.id,
    });
    return response.stop;
  }

  async restart(service?: string) {
    const query = gql`
      mutation RestartService($id: ID, $projectId: ID!) {
        restart(id: $id, projectId: $projectId) {
          pid
        }
      }
    `;
    const response = await this.client.send(query, {
      id: service,
      projectId: this.id,
    });
    return response.restart;
  }

  async status(service: string) {
    const query = gql`
      query Status($id: ID!) {
        status(id: $id) {
          state
        }
      }
    `;
    const response = await this.client.send(query, { id: service });
    return response.status;
  }

  async ps() {
    const query = gql`
      query Processes {
        processes {
          name
          pid
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
