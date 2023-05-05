package examples

import (
	"fmt"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	sdk "github.com/tsirysndr/superviseur-go"
)

func TestCreateProject(t *testing.T) {
	deno := sdk.NewService().
		WithName("deno-fresh").
		WithCommand("./dev.ts").
		WithEnv(map[string]string{
			"PORT": "8000",
		})

	projectDir, err := filepath.Abs("../../../examples/deno-fresh")
	if err != nil {
		panic(err)
	}

	client := sdk.Connect()
	client.NewProject().
		WithName("deno-example").
		WithContext(projectDir).
		WithService(deno).
		Stdout()
	assert.Nil(nil, nil)
}

func TestListProcesses(t *testing.T) {
	client := sdk.Connect()
	processes := client.Project("obese-ants").Processes()
	fmt.Printf("%#v\n", processes)
}

func TestListProjects(t *testing.T) {
	client := sdk.Connect()
	projects := client.Projects()
	fmt.Printf("%#v\n", projects)
}

func TestListServices(t *testing.T) {
	client := sdk.Connect()
	services := client.Project("obese-ants").Services()
	fmt.Printf("%#v\n", services)
}

func TestRestartAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("obese-ants").RestartAll()
}

func TestStartAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("obese-ants").StartAll()
}

func TestStartService(t *testing.T) {
	client := sdk.Connect()
	client.Project("obese-ants").Start("happy-poison")
}

func TestStatus(t *testing.T) {
	client := sdk.Connect()
	process := client.Project("obese-ants").Status("happy-poison")
	fmt.Printf("%#v\n", process)
}

func TestStopAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("obese-ants").StopAll()
}

func TestStopService(t *testing.T) {
	client := sdk.Connect()
	client.Project("obese-ants").Stop("happy-poison")
}

func TestLogs(t *testing.T) {
	client := sdk.Connect()
	logs := client.Project("obese-ants").Logs("happy-poison")
	fmt.Printf("%#v\n", logs)
}
