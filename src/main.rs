use actix_web::{middleware::Logger, post, get, web, App, HttpResponse, HttpServer, Responder};
use tokenizers::tokenizer::{ Tokenizer};
use serde::Deserialize;


//create a struct that will be used to deserialize the JSON payload
#[derive(Deserialize)]
struct Text {
    text: String,
}

//create a struct that will be used to serialize the JSON response
#[derive(serde::Serialize)]
struct SummarizedText {
    text: String,
}

fn bert_base_cased(text: String) -> String {
        let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
        let encoding = tokenizer.encode(text, false)?;
        return encoding.get_tokens().as_string()
}

#[post("/tokenizers/bert-base-cased")]
async fn tokenize_bert(text: web::Json<Text>) -> impl Responder {
    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None);

    let encoding = tokenizer.expect("could not read input text").encode(text.text, false);
    // return the hash as JSON using the SummarizedText struct
    HttpResponse::Ok().json(SummarizedText { text: encoding.get_tokens() })
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Summarization Service</h1>")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(tokenize_bert)
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
