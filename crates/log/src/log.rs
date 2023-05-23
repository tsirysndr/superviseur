use std::path::Path;

use tantivy::{
    directory::MmapDirectory,
    doc,
    schema::{self, Cardinality, Schema},
    DateOptions, Document, Index, IndexReader, ReloadPolicy,
};

use superviseur_types::log::Log;

#[derive(Clone)]
pub struct LogEngine {
    schema: Schema,
    index: Index,
    reader: IndexReader,
}

impl core::fmt::Debug for LogEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogEngine")
            .field("schema", &self.schema)
            .field("index", &self.index)
            .finish()
    }
}

impl LogEngine {
    pub fn new() -> Self {
        let index_path = format!("{}/.superviseur/logs", env!("HOME"));
        // create index_path if not exists
        if !Path::new(&index_path).exists() {
            std::fs::create_dir_all(&index_path).unwrap();
        }

        let mut schema_builder = Schema::builder();

        let date_opts = DateOptions::from(schema::INDEXED)
            .set_stored()
            .set_fast(Cardinality::SingleValue)
            .set_precision(tantivy::DatePrecision::Seconds);

        schema_builder.add_text_field("project", schema::TEXT | schema::STORED);
        schema_builder.add_text_field("service", schema::TEXT | schema::STORED);
        schema_builder.add_text_field("line", schema::TEXT | schema::STORED);
        schema_builder.add_date_field("date", date_opts);
        schema_builder.add_text_field("output", schema::TEXT | schema::STORED);

        let schema = schema_builder.build();
        let dir = MmapDirectory::open(&index_path).unwrap();
        let index = Index::open_or_create(dir, schema.clone()).unwrap();
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .expect("Tantivy reader couldn't be created");

        Self {
            schema,
            index,
            reader,
        }
    }

    pub fn insert(&self, log: &Log) -> tantivy::Result<()> {
        let mut writer = self.index.writer(50_000_000).unwrap();
        let project = self.schema.get_field("project").unwrap();
        let service = self.schema.get_field("service").unwrap();
        let line = self.schema.get_field("line").unwrap();
        let date = self.schema.get_field("date").unwrap();
        let output = self.schema.get_field("output").unwrap();

        // check if log already exists with same date
        let query_parser =
            tantivy::query::QueryParser::for_index(&self.index, vec![project, service]);
        let query = query_parser
            .parse_query(&format!(
                "project:{} AND service:{} AND date:\"{:?}\" AND output:{}",
                log.project, log.service, log.date, log.output
            ))
            .unwrap();
        let top_docs = self
            .reader
            .searcher()
            .search(&query, &tantivy::collector::TopDocs::with_limit(1))?;
        if top_docs.len() > 0 {
            return Ok(());
        }

        let doc: Document = doc!(
            project => log.project.clone(),
            service => log.service.clone(),
            line => log.line.clone(),
            date => log.date.clone(),
            output => log.output.clone()
        );

        writer.add_document(doc)?;
        writer.commit()?;
        Ok(())
    }

    pub fn search(&self, term: &str) -> tantivy::Result<Vec<Log>> {
        let searcher = self.reader.searcher();

        let line = self.schema.get_field("line").unwrap();
        let project = self.schema.get_field("project").unwrap();

        let query_parser = tantivy::query::QueryParser::for_index(&self.index, vec![project, line]);

        let query = query_parser.parse_query(term).unwrap();

        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
        Ok(top_docs
            .iter()
            .map(|(_, doc_address)| searcher.doc(*doc_address).unwrap().into())
            .collect())
    }

    pub fn search_in_service(&self, term: &str) -> tantivy::Result<Vec<Log>> {
        let searcher = self.reader.searcher();

        let line = self.schema.get_field("line").unwrap();
        let project = self.schema.get_field("project").unwrap();
        let service = self.schema.get_field("service").unwrap();

        let query_parser =
            tantivy::query::QueryParser::for_index(&self.index, vec![project, service, line]);

        let query = query_parser.parse_query(term).unwrap();

        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
        Ok(top_docs
            .iter()
            .map(|(_, doc_address)| searcher.doc(*doc_address).unwrap().into())
            .collect())
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    #[test]
    fn test_log_engine() {
        use crate::log::LogEngine;
        use superviseur_types::log::Log;

        let log_engine = LogEngine::new();
        let log = Log {
            project: "demo_project".to_string(),
            service: "demo_service".to_string(),
            line: "demo_line".to_string(),
            date: tantivy::DateTime::from_timestamp_secs(
                chrono::DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                    .unwrap()
                    .timestamp(),
            ),
            output: "demo_output".to_string(),
        };
        log_engine.insert(&log).unwrap();
        std::thread::sleep(Duration::from_secs(1));
        let logs = log_engine.search("demo_line").unwrap();
        assert_eq!(logs.len(), 1);
        let logs = log_engine.search_in_service("demo_line").unwrap();
        assert_eq!(logs.len(), 1);
    }
}
