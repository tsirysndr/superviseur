macro_rules! return_event {
    ($tx: expr, $service_name: expr, $event: expr, $project: expr, $service: expr, $output: expr) => {{
        if $service_name != $service && !$service_name.is_empty() {
            continue;
        }

        match $tx
            .send(Ok(EventsResponse {
                event: $event.to_string(),
                project: $project,
                service: $service,
                date: chrono::Utc::now().to_rfc3339(),
                output: $output,
            }))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("Error sending event: {}", e);
            }
        }
    }};
}

macro_rules! project_exists {
    ($self:ident, $project_id:ident) => {
        if !$self.provider.project_exists(&$project_id).map_err(|e| {
            tonic::Status::internal(format!("Error while loading config: {}", e.to_string()))
        })? {
            return Err(tonic::Status::not_found("Config file not found"));
        }
    };
}

macro_rules! get_project_configuration {
    ($self:ident, $project_id:ident) => {
        $self
            .provider
            .build_configuration(&$project_id)
            .map_err(|e| tonic::Status::internal(e.to_string()))?
    };
}

macro_rules! save_project_configuration {
    ($self:ident, $project_id:ident, $config:ident) => {
        $self
            .provider
            .save_configuration(&$project_id, $config.clone())
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
    };
}

pub(crate) use get_project_configuration;
pub(crate) use project_exists;
pub(crate) use return_event;
pub(crate) use save_project_configuration;
