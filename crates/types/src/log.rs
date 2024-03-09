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

#[cfg(test)]
pub mod tests {
    use tantivy::{schema::Cardinality, DateOptions};

    #[test]
    fn test_from_document() {
        use super::*;
        use tantivy::schema::Schema;
        use tantivy::Document;

        let date_opts = DateOptions::from(schema::INDEXED)
            .set_stored()
            .set_fast(Cardinality::SingleValue)
            .set_precision(tantivy::DatePrecision::Seconds);

        let mut schema = Schema::builder();
        let project_field = schema.add_text_field("project", schema::TEXT);
        let service_field = schema.add_text_field("service", schema::TEXT);
        let line_field = schema.add_text_field("line", schema::TEXT);
        let date_field = schema.add_date_field("date", date_opts);
        let output_field = schema.add_text_field("output", schema::TEXT);

        schema.build();

        let mut doc = Document::default();
        doc.add_text(project_field, "project");
        doc.add_text(service_field, "service");
        doc.add_text(line_field, "line");
        doc.add_date(
            date_field,
            tantivy::DateTime::from_timestamp_secs(
                chrono::DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                    .unwrap()
                    .timestamp(),
            ),
        );
        doc.add_text(output_field, "output");

        let log = Log::from(doc);
        assert_eq!(log.project, "project");
        assert_eq!(log.service, "service");
        assert_eq!(log.line, "line");
        assert_eq!(log.output, "output");
        assert_eq!(
            log.date,
            tantivy::DateTime::from_timestamp_secs(
                chrono::DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                    .unwrap()
                    .timestamp()
            )
        );
    }
}
