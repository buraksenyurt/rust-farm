#[cfg(test)]
mod test {
    use crate::utility::Utility;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn should_ping_convert_to_valid_strings() {
        let mut buffer = BytesMut::with_capacity(64);
        buffer.put(&b"ping"[..]);
        let actual = Utility::convert_to_vec(&mut buffer);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], "ping".to_string());
    }

    #[test]
    fn should_get_constr_convert_to_valid_strings() {
        let mut buffer = BytesMut::with_capacity(64);
        buffer.put(&b"get constr"[..]);
        let actual = Utility::convert_to_vec(&mut buffer);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], "get".to_string());
        assert_eq!(actual[1], "constr".to_string());
    }

    #[test]
    fn should_set_apipath_localhost_convert_to_valid_strings() {
        let mut buffer = BytesMut::with_capacity(64);
        buffer.put(&b"set apipath localhost:1234/api/common"[..]);
        let actual = Utility::convert_to_vec(&mut buffer);
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], "set".to_string());
        assert_eq!(actual[1], "apipath".to_string());
        assert_eq!(actual[2], "localhost:1234/api/common".to_string());
    }
}
