macro_rules! project_exists {
    ($provider:ident, $project_id:ident) => {
        if !$provider
            .project_exists(&$project_id)
            .map_err(|e| Error::new(e.to_string()))?
        {
            return Err(Error::new("Configuration file not found"));
        }
    };
}

macro_rules! send_event {
    ($project:expr, $services:expr,$cmd_tx:ident, $command:ident, $event:ident) => {
        for (_, service) in &$services {
            $cmd_tx
                .send(SuperviseurCommand::$command(
                    service.clone(),
                    $project.clone(),
                ))
                .unwrap();
        }
        let services = $services.clone();
        let services = services
            .iter()
            .map(|(_, x)| Service::from(x))
            .collect::<Vec<Service>>();
        SimpleBroker::publish($event { payload: services });
    };
}

macro_rules! send_event_alt {
    ($project:expr, $services:expr,$cmd_tx:ident, $command:ident, $event:ident) => {
        for (_, service) in &$services {
            $cmd_tx
                .send(SuperviseurCommand::$command(
                    service.clone(),
                    $project.clone(),
                    true,
                ))
                .unwrap();
        }
        let services = $services.clone();
        let services = services
            .iter()
            .map(|(_, x)| Service::from(x))
            .collect::<Vec<Service>>();
        SimpleBroker::publish($event { payload: services });
    };
}

pub(crate) use project_exists;
pub(crate) use send_event;
pub(crate) use send_event_alt;
