package examples

import (
	"testing"

	sdk "github.com/tsirysndr/superviseur-go"
)

func TestCreateProject(t *testing.T) {
	deno := sdk.NewService().
		WithName("deno-fresh").
		WithCommand("./dev.ts")

	client := sdk.Connect()
	client.NewProject().
		WithName("deno-example").
		WithContext("/Users/tsirysandratraina/Documents/GitHub/superviseur/examples/deno-fresh").
		WithService(deno).
		Stdout()
}

func TestListProcesses(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").Processes()
}

func TestListProjects(t *testing.T) {
	client := sdk.Connect()
	client.Projects()
}

func TestListServices(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").Services()
}

func TestRestartAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").RestartAll()
}

func TestStartAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").StartAll()
}

func TestStartService(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").Start("deno-fresh")
}

func TestStatus(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").Status("deno-fresh")
}

func TestStopAll(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").StopAll()
}

func TestStopService(t *testing.T) {
	client := sdk.Connect()
	client.Project("deno-example").Stop("deno-fresh")
}
