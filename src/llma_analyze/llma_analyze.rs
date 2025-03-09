use langchain_rust::embedding::{
     ollama::ollama_embedder::OllamaEmbedder,
};
use langchain_rust::vectorstore::VecStoreOptions;
use langchain_rust::{
    schemas::Document,
    vectorstore::qdrant::{Qdrant, StoreBuilder},
    vectorstore::VectorStore,
};
use std::fs::{File};
use std::io::{BufRead, BufReader, Write};

pub async fn search_analyze(p_files: &str, _search_text: &str) {
    let embedder = OllamaEmbedder::default().with_model("nomic-embed-text");
    let client = Qdrant::from_url("http://localhost:6334").build().unwrap();

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .client(client)
        .collection_name("langchain-rs")
        .build()
        .await
        .unwrap();

    // let mut contents = String::new();
    let file = File::open(p_files).unwrap();
    let mut reader = BufReader::with_capacity(2048 * 2048, file);

    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).unwrap();
        if n == 0 {
            break;
        }
        println!("Aggiungo linea :{}", line);
        // contents.push_str(&line);
        let doc = Document::new(line.as_str());
        // let doc1 = Document::new(
        //     "langchain-rust is a port of the langchain python library to rust and was written in 2024.",
        // );
        store
            .add_documents(&vec![doc], &VecStoreOptions::default())
            .await
            .unwrap();
    }

    std::io::stdout().flush().unwrap();
    let mut query = String::new();
    println!("Query> ");
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
