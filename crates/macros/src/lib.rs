#[macro_export]
macro_rules! default_stderr {
    ($project: expr, $service: expr) => {
        format!("/tmp/{}-{}.err", $project, $service)
    };
}

#[macro_export]
macro_rules! default_stdout {
    ($project: expr, $service: expr) => {
        format!("/tmp/{}-{}.log", $project, $service)
    };
}

#[macro_export]
macro_rules! create_driver {
    ($driver_path:path, $project:expr, $service:expr, $processes:expr, $event_tx:expr, $childs:expr, $log_engine:expr, $superviseur_event:expr) => {
        Box::new($driver_path(
            $project.clone(),
            $service,
            $processes.clone(),
            $event_tx.clone(),
            $childs.clone(),
            $log_engine.clone(),
            $superviseur_event.clone(),
        ))
    };
}

#[macro_export]
macro_rules! check_driver {
    ($use:expr, $driver:expr) => {
        $use.clone()
            .into_iter()
            .any(|(driver, _)| driver == $driver)
    };
}

#[macro_export]
macro_rules! wait_child_process_in_background {
    ($child:expr, $event_tx:expr, $service_name:expr, $project:expr, $superviseur_event_tx:expr) => {
        tokio::spawn(async move {
            let status = $child.wait().unwrap();
            $event_tx
                .send(ProcessEvent::Stopped(
                    $service_name.clone(),
                    $project.clone(),
                ))
                .unwrap();
            match status.success() {
                true => {
                    $superviseur_event_tx
                        .send(SuperviseurEvent::Stopped($project, $service_name.clone()))
                        .unwrap();
                }
                false => {
                    $superviseur_event_tx
                        .send(SuperviseurEvent::Crashed($project, $service_name))
                        .unwrap();
                }
            }
        });
    };
}
