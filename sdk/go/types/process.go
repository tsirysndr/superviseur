package types

type Process struct {
	PID       *int64 `json:"pid"`
	Name      string `json:"name"`
	Project   string `json:"project"`
	ServiceID string `json:"serviceId"`
	Command   string `json:"command"`
	UpTime    string `json:"upTime"`
	State     string `json:"state"`
}
