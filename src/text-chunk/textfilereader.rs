use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub struct TextFileChunkReader {
    reader: BufReader<File>,
    chunk_size: usize,
    buffer: Vec<u8>,
    reached_eof: bool,
}

impl TextFileChunkReader {
    /// Crea un nuovo TextFileChunkReader per il file specificato
    ///
    /// # Argomenti
    /// * `file_path` - Percorso del file da leggere
    /// * `chunk_size` - Dimensione approssimativa di ogni blocco in bytes
    ///
    /// # Esempio
    /// ```
    /// let mut reader = TextFileChunkReader::new("example.txt", 8192).unwrap();
    /// while let Some(chunk) = reader.next_chunk().unwrap() {
    ///     // Processa il blocco
    /// }
    /// ```
    pub fn new<P: AsRef<Path>>(file_path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        Ok(Self {
            reader,
            chunk_size,
            buffer: Vec::with_capacity(chunk_size),
            reached_eof: false,
        })
    }

    /// Legge il prossimo blocco di testo dal file
    ///
    /// Restituisce:
    /// - Ok(Some(String)) se c'è un nuovo blocco da leggere
    /// - Ok(None) se è stata raggiunta la fine del file
    /// - Err(io::Error) in caso di errore di lettura
    pub fn next_chunk(&mut self) -> io::Result<Option<String>> {
        if self.reached_eof {
            return Ok(None);
        }

        self.buffer.clear();

        // Leggi fino a chunk_size bytes o fino al prossimo newline
        let mut total_bytes_read = 0;
        let mut last_newline_pos = 0;

        loop {
            // Usa fill_buf per evitare copie non necessarie
            let buf = self.reader.fill_buf()?;
            if buf.is_empty() {
                self.reached_eof = true;
                break;
            }

            // Cerca l'ultimo newline in questo buffer
            let bytes_to_take = if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                // Trovato un newline, prendi fino a qui +1
                let take = pos + 1;
                last_newline_pos = self.buffer.len() + take;
                take
            } else {
                // Nessun newline, prendi tutto
                buf.len()
            };

            // Aggiungi al nostro buffer
            self.buffer.extend_from_slice(&buf[..bytes_to_take]);
            self.reader.consume(bytes_to_take);

            total_bytes_read += bytes_to_take;

            // Se abbiamo superato la dimensione del chunk o trovato un newline
            if total_bytes_read >= self.chunk_size || bytes_to_take < buf.len() {
                break;
            }
        }

        if self.buffer.is_empty() {
            return Ok(None);
        }

        // Se non abbiamo trovato un newline nell'ultimo pezzo, prendiamo tutto
        let chunk = if last_newline_pos == 0 {
            String::from_utf8(self.buffer.clone())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            String::from_utf8(self.buffer[..last_newline_pos].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        };

        Ok(Some(chunk))
    }

    /// Resetta il reader all'inizio del file
    pub fn reset(&mut self) -> io::Result<()> {
        self.reader.seek(SeekFrom::Start(0))?;
        self.buffer.clear();
        self.reached_eof = false;
        Ok(())
    }
}

/*fn main() -> io::Result<()> {
    let mut reader = TextFileChunkReader::new("large_file.txt", 8192)?;

    while let Some(chunk) = reader.next_chunk()? {
        // Fai qualcosa con il blocco di testo
        println!("Blocco letto ({} caratteri):\n{}", chunk.len(), &chunk[..chunk.len().min(50)]);
    }

    Ok(())
}*/