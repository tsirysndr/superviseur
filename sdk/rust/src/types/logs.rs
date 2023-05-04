use crate::graphql::query::logs_query::LogsQueryLogs;

#[derive(Debug)]
pub struct Logs {
    pub lines: Vec<String>,
}

impl From<LogsQueryLogs> for Logs {
    fn from(logs: LogsQueryLogs) -> Self {
        Logs { lines: logs.lines }
    }
}
