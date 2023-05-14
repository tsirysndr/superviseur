macro_rules! create_driver {
    ($driver_path:path, $project:expr, $service:expr, $processes:expr, $event_tx:expr, $childs:expr, $log_engine:expr) => {
        Box::new($driver_path(
            $project.clone(),
            $service,
            $processes.clone(),
            $event_tx.clone(),
            $childs.clone(),
            $log_engine.clone(),
        ))
    };
}

macro_rules! check_driver {
    ($use:expr, $driver:expr) => {
        $use.clone()
            .into_iter()
            .any(|(driver, _)| driver == $driver)
    };
}

pub(crate) use check_driver;
pub(crate) use create_driver;
