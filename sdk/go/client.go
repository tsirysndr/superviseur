package superviseur

import (
	"context"

	"github.com/machinebox/graphql"
)

type Client struct {
	client *graphql.Client
}

func Connect() *Client {
	client := graphql.NewClient("http://localhost:5478/graphql")
	return &Client{
		client,
	}
}

func (c *Client) NewProject() *Project {
	return &Project{
		ID:       nil,
		Name:     "",
		services: []Service{},
		client:   c,
	}
}

func (c *Client) Project(ID string) *Project {
	req := graphql.NewRequest(`
		query Project($id: ID!) {
			project(id: $id) {
				id
				name
			}
		}
	`)
	req.Var("id", ID)
	var responseData map[string]interface{}
	ctx := context.Background()

	if err := c.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}

	return &Project{
		Name:     "",
		services: []Service{},
		client:   c,
	}
}

func (c *Client) Projects() {
	req := graphql.NewRequest(`
		query Projects {
			projects {
				id
				name
			}
		}
	`)

	ctx := context.Background()

	var responseData map[string]interface{}

	if err := c.client.Run(ctx, req, &responseData); err != nil {
		panic(err)
	}
}
