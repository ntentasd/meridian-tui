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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::api::{Kind, Route};

    use super::*;

    const ARGOCD_FRAME: &str = r#"event: upsert
data: {"kind":"upsert","route":{"uid":"02b61775-c78a-4a32-b3c6-57aa617c1044","name":"argocd","namespace":"platform","kind":"Ingress","hostname":"argocd.platform.local"}}

"#;

    #[fixture]
    fn argocd() -> Event {
        Event {
            kind: Kind::Upsert,
            route: Route {
                uid: "02b61775-c78a-4a32-b3c6-57aa617c1044".into(),
                name: "argocd".into(),
                namespace: "platform".into(),
                kind: "Ingress".into(),
                hostname: "argocd.platform.local".into(),
            },
        }
    }

    #[rstest]
    fn parses_valid_frame(argocd: Event) {
        assert_eq!(parse_frame(ARGOCD_FRAME).unwrap(), Some(argocd));
    }

    #[rstest]
    #[case(": ping\n\n")]
    #[case("id: 7\n\n")]
    fn ignores_frame_without_data(#[case] input: &str) {
        assert_eq!(parse_frame(input).unwrap(), None);
    }

    #[rstest]
    #[case(
        r#"event: upsert
data:

"#
    )]
    #[case(
        r#"event: upsert
data: invalid

"#
    )]
    fn fails_parsing_frame(#[case] input: &str) {
        assert!(matches!(parse_frame(input), Err(ParseError::Json(_))));
    }

    #[rstest]
    fn feed_handles_any_split(argocd: Event) {
        let bytes = ARGOCD_FRAME.as_bytes();

        for i in 0..=bytes.len() {
            let mut parser = Parser::new();

            let mut events = parser.feed(&bytes[..i]).unwrap();
            events.extend(parser.feed(&bytes[i..]).unwrap());

            assert_eq!(events, vec![argocd.clone()], "split at byte {i}");
        }
    }

    #[rstest]
    fn feed_handles_two_frames(argocd: Event) {
        let frames = ARGOCD_FRAME.repeat(2);

        let mut parser = Parser::new();

        let events = parser.feed(frames.as_bytes()).unwrap();

        assert_eq!(events, vec![argocd.clone(), argocd]);
    }
}
