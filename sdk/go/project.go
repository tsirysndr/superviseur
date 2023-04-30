package superviseur

type Project struct {
	Name     string
	Services []Service
}

func (p *Project) New() *Project {
	return nil
}

func (p *Project) WithName(name string) *Project {
	p.Name = name
	return p
}

func (p *Project) WithService(service Service) *Project {
	p.Services = append(p.Services, service)
	return p
}

func (p *Project) Start(service string) {

}

func (p *Project) Stop(service string) {

}

func (p *Project) Restart(service string) {

}

func (p *Project) Status(service string) {

}

func (p *Project) ListServices() {

}

func (p *Project) StartAll() {

}

func (p *Project) StopAll() {

}

func (p *Project) RestartAll() {

}
