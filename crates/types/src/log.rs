use tantivy::{
    schema::{self, Schema, SchemaBuilder},
    DateTime, Document,
};

pub struct Log {
    pub project: String,
    pub service: String,
    pub line: String,
    pub date: DateTime,
    pub output: String,
}

impl From<Document> for Log {
    fn from(doc: Document) -> Self {
        let mut schema_builder: SchemaBuilder = Schema::builder();

        let project_field = schema_builder.add_text_field("project", schema::TEXT);
        let service_field = schema_builder.add_text_field("service", schema::TEXT);
        let line_field = schema_builder.add_text_field("line", schema::TEXT);
        let date_field = schema_builder.add_date_field("date", schema::STORED);
        let output_field = schema_builder.add_text_field("output", schema::TEXT);

        let project = doc
            .get_first(project_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let service = doc
            .get_first(service_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let line = doc
            .get_first(line_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let date = doc.get_first(date_field).unwrap().as_date().unwrap();
        let output = doc
            .get_first(output_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();

        Self {
            project,
            service,
            line,
            date,
            output,
        }
    }
}
