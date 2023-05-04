package types

type Project struct {
	ID       string    `json:"id"`
	Name     string    `json:"name"`
	Services []Service `json:"services"`
	Context  string    `json:"context"`
}
