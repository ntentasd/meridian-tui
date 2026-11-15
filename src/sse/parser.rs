use std::{error, fmt};

use crate::api::Event;

#[derive(Default)]
pub struct Parser {
    buf: Vec<u8>,
}

#[derive(Debug)]
pub enum ParseError {
    Utf8(std::str::Utf8Error),
    Json(serde_json::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utf8(_) => write!(f, "frame is not valid UTF-8"),
            Self::Json(_) => write!(f, "event payload is not valid JSON"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Utf8(e) => Some(e),
            Self::Json(e) => Some(e),
        }
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<Vec<Event>, ParseError> {
        self.buf.extend_from_slice(bytes);

        let mut events = Vec::new();

        // Check for complete frames
        while let Some(i) = self.buf.windows(2).position(|w| w == b"\n\n") {
            let result = std::str::from_utf8(&self.buf[..i + 2])
                .map_err(ParseError::from)
                .and_then(parse_frame);
            // Drain before calling '?'. Erroneous frames get dropped before the error is thrown.
            self.buf.drain(..i + 2);

            if let Some(event) = result? {
                events.push(event);
            }
        }

        Ok(events)
    }
}

fn parse_frame(frame: &str) -> Result<Option<Event>, ParseError> {
    for line in frame.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let value = value.strip_prefix(' ').unwrap_or(value);
            if key == "data" {
                let event = serde_json::from_str(value)?;
                return Ok(Some(event));
            }
        }
    }

    Ok(None)
}
