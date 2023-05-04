package superviseur

import (
	"context"

	"github.com/machinebox/graphql"
	"github.com/mitchellh/mapstructure"
	"github.com/tsirysndr/superviseur-go/types"
)

type Project struct {
	ID       *string
	Name     string
	services []Service
	client   *Client
	context  string
}

func (p *Project) WithName(name string) *Project {
	p.Name = name
	return p
}

func (p *Project) WithContext(context string) *Project {
	p.context = context
	return p
}

func (p *Project) WithService(service *Service) *Project {
	p.services = append(p.services, *service)
	return p
}

func (p *Project) Stdout() {

}

func (p *Project) Start(service string) types.Process {
	req := graphql.NewRequest(`
		mutation StartService($id: ID, $projectId: ID!) {
			start(id: $id, projectId: $projectId) {
				pid
			}
		}
	`)
	req.Var("id", service)
	req.Var("projectId", p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["start"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) Stop(service string) types.Process {
	req := graphql.NewRequest(`
		mutation StopService($id: ID, $projectId: ID!) {
			stop(id: $id, projectId: $projectId) {
				pid
			}
		}
	`)
	req.Var("id", service)
	req.Var("projectId", p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}
	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["stop"], &process); err != nil {
		panic(err)
	}
	return process
}

func (p *Project) Restart(service string) types.Process {
	req := graphql.NewRequest(`
		mutation RestartService($id: ID, $projectId: ID!) {
			restart(id: $id, projectId: $projectId) {
				pid
			}
		}
	`)
	req.Var("id", service)
	req.Var("projectId", p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["restart"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) Status(service string) types.Process {
	req := graphql.NewRequest(`
		query Status($id: ID!) {
			status(id: $id) {
				pid
				project
				name
				serviceId
				state
				command
				upTime
			}
		}
	`)
	req.Var("id", service)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["status"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) StartAll() types.Process {
	req := graphql.NewRequest(`
		mutation StartAll($projectId: ID!) {
			start(projectId: $projectId) {
				name
				pid
				serviceId
				command
			}
		}
	`)
	req.Var("projectId", *p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}
	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["start"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) StopAll() types.Process {
	req := graphql.NewRequest(`
		mutation StopAll($projectId: ID!) {
			stop(projectId: $projectId) {
				name
				pid
				serviceId
				command
			}
		}
	`)
	req.Var("projectId", *p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["stop"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) RestartAll() types.Process {
	req := graphql.NewRequest(`
		mutation RestartAll($projectId: ID!) {
			restart(projectId: $projectId) {
				name
				pid
				serviceId
				command
			}
		}
	`)
	req.Var("projectId", *p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var process types.Process
	if err := mapstructure.Decode(responseData["restart"], &process); err != nil {
		panic(err)
	}

	return process
}

func (p *Project) Logs(service string) types.Logs {
	req := graphql.NewRequest(`
		query Logs($id: ID!, $projectId: ID!) {
			logs(id: $id, projectId: $projectId) {
				lines
			}
		}
	`)
	req.Var("id", service)
	req.Var("projectId", *p.ID)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var logs types.Logs

	if err := mapstructure.Decode(responseData["logs"], &logs); err != nil {
		panic(err)
	}

	return logs

}

func (p *Project) Processes() []types.Process {
	req := graphql.NewRequest(`
		query Processes {
			processes {
				name
				pid
				serviceId
				command
				upTime
			}
		}
	`)
	ctx := context.Background()

	var responseData map[string][]interface{}
	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var processes []types.Process
	if err := mapstructure.Decode(responseData["processes"], &processes); err != nil {
		panic(err)
	}

	return processes
}

func (p *Project) Services() []types.Service {
	req := graphql.NewRequest(`
		query Services($projectId: ID!) {
			services(projectId: $projectId) {
				id
				name
				command
				status
			}
		}
	`)
	req.Var("projectId", *p.ID)
	ctx := context.Background()

	var responseData map[string][]interface{}
	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var services []types.Service
	if err := mapstructure.Decode(responseData["services"], &services); err != nil {
		panic(err)
	}

	return services
}

func (p *Project) AddEnvVar(serviceID, name, value string) types.Service {
	req := graphql.NewRequest(`
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
	`)
	req.Var("projectId", *p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	req.Var("value", value)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var service types.Service

	if err := mapstructure.Decode(responseData["createEnvVar"], &service); err != nil {
		panic(err)
	}

	return service
}

func (p *Project) RemoveEnvVar(serviceID, name string) types.Service {
	req := graphql.NewRequest(`
		mutation DeleteEnvVar($projectId: ID!, $id: ID!, $name: String!) {
			deleteEnvVar(projectId: $projectId, id: $id, name: $name) {
				id
				env
			}
		}
	`)
	req.Var("projectId", *p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var service types.Service

	if err := mapstructure.Decode(responseData["deleteEnvVar"], &service); err != nil {
		panic(err)
	}

	return service
}

func (p *Project) UpdateEnvVar(serviceID, name, value string) types.Service {
	req := graphql.NewRequest(`
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
	`)
	req.Var("projectId", *p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	req.Var("value", value)
	ctx := context.Background()

	var responseData map[string]interface{}

	if err := p.client.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	var service types.Service

	if err := mapstructure.Decode(responseData["updateEnvVar"], &service); err != nil {
		panic(err)
	}

	return service
}
