//! This module contains the tool that parses the emails and extracts the event data.

use crate::event::Event;
use select::{document::Document, predicate::Name};

/// Parses the email and returns the event data.
pub fn parse_email(email: &str) -> Vec<crate::event::Event> {
    let document = Document::from(email);

    document
        .find(Name("img"))
        .filter_map(|node| node.attr("src"))
        .map(Event::from)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_empty_email() {
        let email = "";
        assert!(parse_email(email).is_empty());
    }

    #[test]
    fn it_parses_single_event() {
        let email = r#"<html><body><img src="event1.jpg"></body></html>"#;
        let events = parse_email(email);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].image, "event1.jpg");
    }

    #[test]
    fn it_parses_multiple_events() {
        let email =
            r#"<html><body><img src="event1.jpg"><img src="event2.jpg"></img></body></html>"#;
        let events = parse_email(email);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].image, "event1.jpg");
        assert_eq!(events[1].image, "event2.jpg");
    }

    #[test]
    fn it_ignores_non_img_tags() {
        let email = r#"<html><body><p>Some text</p><img src="event.jpg"></body></html>"#;
        let events = parse_email(email);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].image, "event.jpg");
    }
}
