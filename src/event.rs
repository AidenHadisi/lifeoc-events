//! Contains the event type which represents an upcoming event.

use chrono::{Datelike, Duration, Utc, Weekday};
use serde::Serialize;

/// An upcoming event.
#[derive(Debug, Clone, Serialize)]
pub struct Event {
    /// The title of the event.
    pub title: String,

    /// The start date of the event.
    /// This should always be the next Sunday.
    pub start_date: String,

    /// The image of the event.
    pub image: String,
}

impl Event {
    /// Creates a new event.
    pub fn new(image: String) -> Self {
        let today = Utc::now();

        // Find the next Sunday.
        let next_sunday = (0..7)
            .map(|i| today + Duration::days(i))
            .find(|d| d.weekday() == Weekday::Sun)
            .unwrap();

        Self {
            title: image.clone(),
            image,
            start_date: next_sunday.to_string(),
        }
    }
}

impl From<&str> for Event {
    /// Creates a new event from the given image.
    fn from(image: &str) -> Self {
        Self::new(image.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_new_event_next_sunday() {
        let image = "event_image.png".to_string();
        let event = Event::new(image.clone());

        // Check that the title is set correctly.
        assert_eq!(event.title, image);

        // Check that the image is set correctly.
        assert_eq!(event.image, image);

        // Parse the start_date and verify it is a valid date and a Sunday.
        let start_date = event.start_date.parse::<chrono::DateTime<Utc>>();
        assert!(start_date.is_ok(), "start_date should be a valid DateTime");

        let start_date = start_date.unwrap();
        assert_eq!(
            start_date.weekday(),
            Weekday::Sun,
            "start_date should be a Sunday"
        );
    }

    #[test]
    fn test_from_str() {
        let image_str = "event_image.png";
        let event = Event::from(image_str);

        // Check that the title is set correctly.
        assert_eq!(event.title, image_str);

        // Check that the image is set correctly.
        assert_eq!(event.image, image_str);

        // Parse the start_date and verify it is a valid date and a Sunday.
        let start_date = event.start_date.parse::<chrono::DateTime<Utc>>();
        assert!(start_date.is_ok(), "start_date should be a valid DateTime");

        let start_date = start_date.unwrap();
        assert_eq!(
            start_date.weekday(),
            Weekday::Sun,
            "start_date should be a Sunday"
        );
    }
}
