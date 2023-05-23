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

pub(crate) use project_exists;
