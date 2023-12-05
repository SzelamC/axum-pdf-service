use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum PDFError {
    IOError(StatusCode, String),
}

impl IntoResponse for PDFError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            PDFError::IOError(code, message) => (code, message),
        };

        let body = Json(json!({
            "error_message": error_message
        }));
        (status_code, body).into_response()
    }
}

pub async fn handle_pdf_to_text(mut multipart: Multipart) -> Result<String, PDFError> {
    use poppler::Document;

    let mut data: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        data.append(&mut Vec::from(field.bytes().await.unwrap()));
    }

    let pdf = Document::from_data(&data, None).map_err(|_| {
        PDFError::IOError(
            StatusCode::BAD_REQUEST,
            "Cannot open pdf file, please try again".to_string(),
        )
    })?;

    let mut content = String::from("");

    let n_page = pdf.n_pages();
    for page_idx in 0..n_page {
        let page = pdf.page(page_idx);
        if let Some(page) = page {
            if let Some(text) = page.text() {
                content.push_str(text.as_str());
            }
        };
    }
    Ok(content)
}
