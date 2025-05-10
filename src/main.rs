use http::{Method};
use tokio::{fs};
use axum::{extract::Path, routing::{get}, Router, http::StatusCode, response::Response, body::Body};
use tower_http::cors::{CorsLayer, Any};


async fn html_file() -> axum::body::Body {
    let filename = "content/page.html";
    let content = fs::read_to_string(filename)
        .await
        .expect(format!("Eroare la deschiderea filei {}\n", filename).as_str());
    content.into()
}

async fn handler(Path(cuvant): Path<String> ) -> axum::body::Body {
    let filename = format!("./content/{}", cuvant);
    let content = fs::read_to_string(filename.as_str())
        .await
        .unwrap_or(
            fs::read_to_string("content/error404.html").await.unwrap()
        );
        //.expect(format!("Eroare la deschiderea filei {}", filename.as_str()).as_str());
    println!("[GET] <= PATH: {}", cuvant);
    println!("[POST] => FILE: {}", cuvant);
    content.into()
}


async fn image_handler(Path(filename): Path<String>) -> Response<Body> {
    let file_location = format!("./content/images/{}", filename);
    let bytes = match tokio::fs::read(&file_location).await {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("[ERROR] => STATUS_CODE: 404, file: {}", filename);
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
        }
    };
    let image_type = filename.chars().rev().take(3).collect::<String>().chars().rev().collect::<String>();
    println!("[GET] <= PATH: {}", filename);
    println!("[POST] => TYPE: {}, FILE: {}", image_type, filename);
    Response::builder()
        .header("Content-Type", format!("image/{}", image_type.as_str()).as_str())
        .body(axum::body::Body::from(bytes))
        .unwrap()
}


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(html_file))
        .route("/:cuvant", get(handler))
        .route("/images/:filename", get(image_handler))
        .layer(cors);
        
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("[START SERVER] => PORT: 7878, ADRESS:  http://127.0.0.1:7878/");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}