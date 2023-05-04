package types

type Service struct {
	ID      string   `json:"id"`
	Name    string   `json:"name"`
	Command string   `json:"command"`
	Status  string   `json:"status"`
	Env     []string `json:"env"`
}
