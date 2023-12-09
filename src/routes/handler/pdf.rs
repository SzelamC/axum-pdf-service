use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use poppler::PopplerDocument;
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
    let mut data: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        data.append(&mut Vec::from(field.bytes().await.unwrap()));
    }

    let pdf = PopplerDocument::new_from_data(&mut data, "").map_err(|_| {
        PDFError::IOError(
            StatusCode::BAD_REQUEST,
            "Cannot open pdf file, please try again".to_string(),
        )
    })?;

    pdf_to_text(&pdf)
}

fn pdf_to_text(pdf_file: &PopplerDocument) -> Result<String, PDFError> {
    let mut content = String::new();
    let n_page = pdf_file.get_n_pages();
    for page_idx in 0..n_page {
        let page = pdf_file.get_page(page_idx);
        if let Some(page) = page {
            if let Some(text) = page.get_text() {
                content.push_str(text);
            }
        };
    }

    Ok(content)
}

mod test {
    use std::path::Path;

    use poppler::PopplerDocument;

    use super::pdf_to_text;

    #[test]
    fn extract_text_correctly() {
        let file_path = Path::new("./files/test_1.pdf");
        let pdf = PopplerDocument::new_from_file(
            file_path,
            "",
        )
        .unwrap();
        let content = pdf_to_text(&pdf).unwrap();

        assert!(content.contains("This is a small demonstration .pdf file"));
        assert!(content.contains("And more text. And more text."));
        assert!(content.contains("...continued from page 1."));
    }
}
