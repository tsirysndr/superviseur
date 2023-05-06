use crate::service::Service;

pub fn build_nested_with_service_query(services: Vec<Service>) -> String {
    let mut query = "id stdout".to_string();
    for service in services {
        query = format!(
            r#"
            withService(service: {{ {} }}) {{
                {}
            }}
            "#,
            build_params(service),
            query
        );
    }
    query
}

fn build_params(service: Service) -> String {
    let mut params = String::new();

    if !service.name.is_empty() {
        params = format!(r#"name: "{}", "#, service.name);
    }

    if !service.command.is_empty() {
        params = format!(r#"{} command: "{}", "#, params, service.command);
    }

    if service.port.is_some() {
        params = format!(r#"{} port: {}, "#, params, service.port.unwrap());
    }

    if !service.r#type.is_empty() {
        params = format!(r#"{} type: "{}", "#, params, service.r#type);
    }

    if !service.working_dir.is_empty() {
        params = format!(r#"{} workingDir: "{}", "#, params, service.working_dir);
    }

    if !service.description.is_empty() {
        params = format!(r#"{} description: "{}", "#, params, service.description);
    }

    if !service.depends_on.is_empty() {
        params = format!(
            "{} dependsOn: [{}], ",
            params,
            service
                .depends_on
                .iter()
                .map(|s| format!(r#""{}""#, s))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    if !service.env.is_empty() {
        params = format!(
            "{} env: [{}], ",
            params,
            service
                .env
                .iter()
                .map(|(k, v)| format!(r#""{}={}""#, k, v))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    params.truncate(params.len() - 2);
    params
}
