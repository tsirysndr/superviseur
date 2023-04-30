package superviseur

type Service struct {
	Name            string
	Command         string
	ExecType        string
	WorkingDir      string
	Description     string
	Env             map[string]string
	AutoStart       bool
	AutoRestart     bool
	Namespace       string
	Stdout          string
	Stderr          string
	BuildCommand    string
	FloxEnvrionment *string
}

func (s *Service) New() *Service {
	return nil
}

func (s *Service) WithName(name string) *Service {
	s.Name = name
	return s
}

func (s *Service) WithCommand(command string) *Service {
	s.Command = command
	return s
}

func (s *Service) WithDescription(description string) *Service {
	s.Description = description
	return s
}

func (s *Service) WithExecType(execType string) *Service {
	s.ExecType = execType
	return s
}

func (s *Service) WithWorkingDir(workingDir string) *Service {
	s.WorkingDir = workingDir
	return s
}

func (s *Service) WithEnv(env map[string]string) *Service {
	s.Env = env
	return s
}

func (s *Service) WithAutoStart(autoStart bool) *Service {
	s.AutoStart = autoStart
	return s
}

func (s *Service) WithAutoRestart(autoRestart bool) *Service {
	s.AutoRestart = autoRestart
	return s
}

func (s *Service) WithNamespace(namespace string) *Service {
	s.Namespace = namespace
	return s
}

func (s *Service) WithStdout(stdout string) *Service {
	s.Stdout = stdout
	return s
}

func (s *Service) WithStderr(stderr string) *Service {
	s.Stderr = stderr
	return s
}

func (s *Service) WithBuildCommand(buildCommand string) *Service {
	s.BuildCommand = buildCommand
	return s
}

func (s *Service) WithFloxEnvironment(floxEnvironment *string) *Service {
	s.FloxEnvrionment = floxEnvironment
	return s
}
