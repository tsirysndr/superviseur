package superviseur

type Project struct {
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

}

func (p *Project) Stop(service string) {

}

func (p *Project) Restart(service string) {

}

func (p *Project) Status(service string) {

}

func (p *Project) Services() {

}

func (p *Project) StartAll() {

}

func (p *Project) StopAll() {

}

func (p *Project) RestartAll() {

}

func (p *Project) Logs(service string) {

}

func (p *Project) Processes() {

}
