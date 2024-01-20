pub const VECTOR_DIMENSION: usize = 1536;
pub const MAX_INPUT_TOKENS: usize = 2046;
pub const EMBEDDING_MODEL_NAME: &str = "text-embedding-ada-002";
pub const COMPLETION_MODEL_NAME: &str = "gpt-3.5-turbo";
pub const COMPLETION_API_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const EMBEDDING_API_URL: &str = "https://api.openai.com/v1/embeddings";
pub const VECTOR_DB_PATH: &str = ".db_vector";
pub const DB_PATH: &str = ".db";
pub const REQUEST_TIMEOUT: u64 = 30;