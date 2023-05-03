package superviseur

import (
	"context"

	"github.com/machinebox/graphql"
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

func (p *Project) Start(service string) {
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

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) Stop(service string) {
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

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) Restart(service string) {
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

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) Status(service string) {
	req := graphql.NewRequest(`
		query Status($id: ID!) {
			status(id: $id) {
				state
			}
		}
	`)
	req.Var("id", service)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) Services() {
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
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) StartAll() {
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
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) StopAll() {
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
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) RestartAll() {
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
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) Logs(service string) {
	req := graphql.NewRequest(`
		query Logs($id: ID!, $projectId: ID!) {
			logs(id: $id, projectId: $projectId) {
				lines
			}
		}
	`)
	req.Var("id", service)
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}

}

func (p *Project) Processes() {
	req := graphql.NewRequest(`
		query Processes($projectId: ID!) {
			processes(projectId: $projectId) {
				name
				pid
				serviceId
				command
				upTime
			}
		}
	`)
	req.Var("projectId", p.ID)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) AddEnvVar(serviceID, name, value string) {
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
	req.Var("projectId", p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	req.Var("value", value)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) RemoveEnvVar(serviceID, name string) {
	req := graphql.NewRequest(`
		mutation DeleteEnvVar($projectId: ID!, $id: ID!, $name: String!) {
			deleteEnvVar(projectId: $projectId, id: $id, name: $name) {
				id
				env
			}
		}
	`)
	req.Var("projectId", p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}

func (p *Project) UpdateEnvVar(serviceID, name, value string) {
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
	req.Var("projectId", p.ID)
	req.Var("id", serviceID)
	req.Var("name", name)
	req.Var("value", value)
	ctx := context.Background()

	if err := p.client.client.Run(ctx, req, nil); err != nil {
		panic(err)
	}
}
