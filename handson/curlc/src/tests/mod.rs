#[cfg(test)]
mod tests {
    use crate::io::parser::{parse_line, LineOutput};
    use crate::io::reader::read_file;
    use crate::transport::http::HttpRequest;
    use crate::transport::http::Method::{Get, Post};

    #[test]
    fn should_http_get_works() {
        let response = parse_line("GET google.com");
        let expected = LineOutput::Valid(HttpRequest {
            method: Get,
            url: "google.com".to_string(),
        });
        assert_eq!(response, expected);
    }

    #[test]
    fn should_http_post_works() {
        let response = parse_line("POST movies.com/api {title : 'interstellar'}");
        let expected = LineOutput::Valid(HttpRequest {
            method: Post {
                body: "{title : 'interstellar'}".to_string(),
            },
            url: "movies.com/api".to_string(),
        });
        assert_eq!(response, expected);
    }

    #[test]
    fn should_invalid_request_raise_none() {
        let response = parse_line("houston we have a problem");
        let expected = LineOutput::None;
        assert_eq!(response, expected);
    }

    #[test]
    fn should_hashtag_raise_comment() {
        let response = parse_line("# It's comment line");
        let expected = LineOutput::Comment;
        assert_eq!(response, expected);
    }

    #[test]
    fn should_reader_works() {
        let request = read_file("initial.req");
        let expected = vec![
            LineOutput::Valid(HttpRequest {
                method: Get,
                url: "google.com".to_string(),
            }),
            LineOutput::Valid(HttpRequest {
                method: Post {
                    body: "{\"title\" : \"Interstellar\"}".to_string(),
                },
                url: "movies/api".to_string(),
            }),
            LineOutput::None,
            LineOutput::Valid(HttpRequest {
                method: Get,
                url: "buraksenyurt.com".to_string(),
            }),
            LineOutput::Comment,
        ];
        assert_eq!(request, expected);
    }
}
