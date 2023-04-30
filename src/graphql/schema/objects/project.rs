use async_graphql::{Object, ID};

#[derive(Default, Clone, Debug)]
pub struct Project {
    pub id: ID,
    pub name: String,
    pub config_path: String,
}

#[Object]
impl Project {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn config_path(&self) -> &str {
        &self.config_path
    }
}
