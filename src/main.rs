use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::thread;
use tokenizers::tokenizer::Tokenizer;

//create a struct that will be used to deserialize the JSON payload
#[derive(Deserialize)]
struct Text {
    text: String,
}

//create a struct that will be used to serialize the JSON response
#[derive(serde::Serialize)]
struct TokenizedText {
    tokens: Vec<String>,
}

async fn tokenize_text(pretrained_model: String, text: String) -> Vec<String> {
    // create a thread to load the tokenizer because this is a blocking call that makes actix panic
    let handle = thread::spawn(move || {
        // create the tokenizer
        return Tokenizer::from_pretrained(pretrained_model, None);
    });

    let tokenizer = handle.join().expect("Failed to join thread");

    // encode the text using the tokenizer
    let encoded = tokenizer
        .expect("could not create the tokenizer")
        .encode(text.clone(), false)
        .expect("could not read the text");

    // get the tokens from the encoding by unwrapping the result
    let tokens = encoded.get_tokens();
    let tokenized_values = Vec::from(tokens);

    return tokenized_values;
}

#[post("tokenizers/{pretrained_model}")]
async fn tokenize(
    pretrained_model: web::Path<String>,
    text: web::Json<Text>,
) -> impl Responder {
    let pretrained_model = pretrained_model.into_inner();
    let tokenized_values = tokenize_text(pretrained_model, text.text.clone()).await;

    // return the tokenized values
    HttpResponse::Ok().json(TokenizedText {
        tokens: tokenized_values,
    })
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Summarization Service from Duke University</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(tokenize)
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
