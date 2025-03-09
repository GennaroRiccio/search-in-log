use std::fs::{DirEntry, File, ReadDir};
use std::io::{BufRead, BufReader, Read, Write};
use langchain_rust::{
    schemas::Document,
    vectorstore::{sqlite_vss::StoreBuilder, VecStoreOptions, VectorStore},
};
use langchain_rust::embedding::{
    embedder_trait::Embedder, ollama::ollama_embedder::OllamaEmbedder,
};

pub async fn search_analyze(p_files: &str, search_text :&str){
    let embedder  = OllamaEmbedder::default().with_model("nomic-embed-text");
    let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite::memory:".to_string());
    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .connection_url(database_url)
        .table("documents")
        .vector_dimensions(1536)
        .build()
        .await
        .unwrap();

    // Initialize the tables in the database. This is required to be done only once.
    store.initialize()
        .await
        .unwrap();
    let mut contents = String::new();

        let file = File::open(p_files).unwrap();

        let mut reader = BufReader::with_capacity(2048 * 2048, file);
        let mut line = String::new();

        loop{
            let n = reader.read_line(&mut line).unwrap();
            if n == 0 {
                break;
            }
            contents.push_str(&line);
        }


    let doc = Document::new(contents.as_str());
    // let doc1 = Document::new(
    //     "langchain-rust is a port of the langchain python library to rust and was written in 2024.",
    // );
    store
        .add_documents(&vec![doc], &VecStoreOptions::default())
        .await
        .unwrap();

    std::io::stdout().flush().unwrap();
    let mut query = String::new();
    print!("Query> ");
    std::io::stdin().read_line(&mut query).unwrap();

    let results = store
        .similarity_search(&query, 2, &VecStoreOptions::default())
        .await
        .unwrap();

    if results.is_empty() {
        println!("No results found.");
        return;
    } else {
        results.iter().for_each(|r| {
            println!("Document: {}", r.page_content);
        });
    }
}
