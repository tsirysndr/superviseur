use std::path::Path;

use tantivy::{
    directory::MmapDirectory,
    doc,
    schema::{self, Schema, SchemaBuilder},
    DateTime, Document, Index, IndexReader, ReloadPolicy,
};

pub struct Log {
    pub project: String,
    pub service: String,
    pub line: String,
    pub date: DateTime,
    pub output: String,
}

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

        schema_builder.add_text_field("project", schema::TEXT | schema::STORED);
        schema_builder.add_text_field("service", schema::TEXT | schema::STORED);
        schema_builder.add_text_field("line", schema::TEXT | schema::STORED);
        schema_builder.add_date_field("date", schema::STORED);
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
