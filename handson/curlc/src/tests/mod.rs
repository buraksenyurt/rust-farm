#[cfg(test)]
mod tests {
    use crate::io::parser::parse_line;
    use crate::io::reader::read_file;
    use crate::transport::http::Method::{Get, Post};
    use crate::transport::http::{HttpParseError, HttpRequest};

    #[test]
    fn should_http_get_works() {
        let response = parse_line("GET google.com");
        let expected = Ok(HttpRequest {
            method: Get,
            url: "google.com".to_string(),
        });
        assert_eq!(response, expected);
    }

    #[test]
    fn should_http_post_works() {
        let response = parse_line("POST movies.com/api {title : 'interstellar'}");
        let expected = Ok(HttpRequest {
            method: Post {
                body: "{title : 'interstellar'}".to_string(),
            },
            url: "movies.com/api".to_string(),
        });
        assert_eq!(response, expected);
    }

    #[test]
    fn should_uncrecognize_error() {
        let response = parse_line("houston we have a problem");
        let expected = Err(HttpParseError::UnrecognizedFormat);
        assert_eq!(response, expected);
    }

    #[test]
    fn should_reader_works() {
        let request = read_file("initial.req");
        let expected = vec![
            Ok(HttpRequest {
                method: Get,
                url: "google.com".to_string(),
            }),
            Ok(HttpRequest {
                method: Post {
                    body: "{\"title\" : \"Interstellar\"}".to_string(),
                },
                url: "movies/api".to_string(),
            }),
            Err(HttpParseError::UnrecognizedFormat),
            Ok(HttpRequest {
                method: Get,
                url: "buraksenyurt.com".to_string(),
            }),
        ];
        assert_eq!(request, expected);
    }
}
