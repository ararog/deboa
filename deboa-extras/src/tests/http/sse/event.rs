use crate::http::sse::event::ServerEvent;
use bytes::Bytes;

#[test]
fn test_parse_event() {
    let event = ServerEvent::parse(&Bytes::from("id: 1\n\ndata: 2\n\n")).unwrap();
    assert_eq!(event.id(), &Some("1".to_string()));
    assert_eq!(event.data(), &vec!["2".to_string()]);
}
