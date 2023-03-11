use async_graphql::Object;

#[derive(Default, Clone)]
pub struct Log {
    pub lines: Vec<String>,
}

#[Object]
impl Log {
    async fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
