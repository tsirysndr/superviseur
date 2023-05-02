package superviseur

type Client struct {
}

func Connect() *Client {
	return &Client{}
}

func (c *Client) NewProject() *Project {
	return &Project{
		Name:     "",
		services: []Service{},
		client:   c,
	}
}

func (c *Client) Project(ID string) *Project {
	return &Project{
		Name:     "",
		services: []Service{},
		client:   c,
	}
}

func (c *Client) Projects() {

}
