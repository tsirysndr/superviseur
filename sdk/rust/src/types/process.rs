use crate::graphql::query::processes_query::ProcessesQueryProcesses;
use crate::graphql::query::restart_all_services::RestartAllServicesRestart;
use crate::graphql::query::restart_service::RestartServiceRestart;
use crate::graphql::query::start_all_services::StartAllServicesStart;
use crate::graphql::query::start_service::StartServiceStart;
use crate::graphql::query::status_query::StatusQueryStatus;
use crate::graphql::query::stop_all_services::StopAllServicesStop;
use crate::graphql::query::stop_service::StopServiceStop;

use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct Process {
    pub name: String,
    pub pid: Option<i64>,
    pub project: String,
    pub service_id: String,
    pub command: String,
    pub up_time: DateTime<Utc>,
}

impl From<ProcessesQueryProcesses> for Process {
    fn from(process: ProcessesQueryProcesses) -> Self {
        Process {
            name: process.name,
            pid: process.pid,
            project: process.project,
            service_id: process.service_id,
            command: process.command,
            up_time: DateTime::parse_from_rfc3339(&process.up_time)
                .unwrap()
                .with_timezone(&Utc),
        }
    }
}

impl From<StartServiceStart> for Process {
    fn from(process: StartServiceStart) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<RestartServiceRestart> for Process {
    fn from(process: RestartServiceRestart) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<StopServiceStop> for Process {
    fn from(process: StopServiceStop) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<StartAllServicesStart> for Process {
    fn from(process: StartAllServicesStart) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<RestartAllServicesRestart> for Process {
    fn from(process: RestartAllServicesRestart) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<StopAllServicesStop> for Process {
    fn from(process: StopAllServicesStop) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}

impl From<StatusQueryStatus> for Process {
    fn from(process: StatusQueryStatus) -> Self {
        Process {
            pid: process.pid,
            ..Default::default()
        }
    }
}
