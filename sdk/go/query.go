package superviseur

import "fmt"

func BuildNestedQuery(service []Service) string {
	query := "id stdout"
	for _, service := range service {
		query = fmt.Sprintf(`
			withService(service: { %s }) {
				%s
			}
		`, BuildParams(service), query)
	}

	return query
}

func BuildParams(service Service) string {
	params := ""

	if service.Name != "" {
		params = fmt.Sprintf(`name: "%s", `, service.Name)
	}

	if service.Command != "" {
		params = fmt.Sprintf(`%s command: "%s", `, params, service.Command)
	}

	if len(service.Env) > 0 {
		env := ""
		for key, value := range service.Env {
			env = fmt.Sprintf(`"%s=%s", `, key, value)
		}

		env = env[:len(env)-2]
		params = fmt.Sprintf(`%s env: [%s], `, params, env)
	}

	if len(service.DependsOn) > 0 {
		dependsOn := ""
		for _, value := range service.DependsOn {
			dependsOn += fmt.Sprintf(`"%s", `, value)
		}
		dependsOn = dependsOn[:len(dependsOn)-2]
		params = fmt.Sprintf(`
		%s dependsOn: [%s],
	`, params, dependsOn)
	}
	params = params[:len(params)-2]
	return params
}
