//! kitty: a Herdr client's container is the kitty window in `KITTY_WINDOW_ID`.
//!
//! `kitten @ focus-window` switches to that window's tab and brings its OS
//! window forward. It needs kitty remote control over a socket
//! (`allow_remote_control` and `listen_on` in kitty.conf), which kitty exposes
//! to the client as `KITTY_LISTEN_ON`.

use std::path::Path;

use super::{FocusCommand, HerdrClient, TerminalAdapter};

pub(super) struct Kitty;

impl TerminalAdapter for Kitty {
    fn bundle_id(&self) -> &'static str {
        "net.kovidgoyal.kitty"
    }

    fn focus_command(&self, client: &HerdrClient) -> Option<FocusCommand> {
        let window_id = client
            .env("KITTY_WINDOW_ID")
            .filter(|id| id.bytes().all(|b| b.is_ascii_digit()))?;
        let listen_on = client.env("KITTY_LISTEN_ON")?;
        // KITTY_INSTALLATION_DIR is `kitty.app/Contents/Resources/kitty`; the
        // kitten binary lives in `kitty.app/Contents/MacOS`.
        let kitten = Path::new(client.env("KITTY_INSTALLATION_DIR")?)
            .parent()?
            .parent()?
            .join("MacOS/kitten");
        Some(FocusCommand::new(
            kitten,
            [
                "@".to_string(),
                "--to".to_string(),
                listen_on.to_string(),
                "focus-window".to_string(),
                "--match".to_string(),
                format!("id:{window_id}"),
            ],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTALL: (&str, &str) = (
        "KITTY_INSTALLATION_DIR",
        "/Applications/kitty.app/Contents/Resources/kitty",
    );

    fn client(pairs: &[(&str, &str)]) -> HerdrClient {
        HerdrClient {
            environment: pairs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }

    #[test]
    fn focuses_the_client_kitty_window() {
        let command = Kitty.focus_command(&client(&[
            ("KITTY_WINDOW_ID", "3"),
            ("KITTY_LISTEN_ON", "unix:/tmp/my kitty-42"),
            INSTALL,
        ]));
        assert_eq!(
            command,
            Some(FocusCommand::new(
                "/Applications/kitty.app/Contents/MacOS/kitten",
                [
                    "@",
                    "--to",
                    "unix:/tmp/my kitty-42",
                    "focus-window",
                    "--match",
                    "id:3"
                ],
            ))
        );
    }

    #[test]
    fn requires_remote_control_and_a_numeric_window_id() {
        let listen = ("KITTY_LISTEN_ON", "unix:/tmp/kitty");
        assert_eq!(
            Kitty.focus_command(&client(&[("KITTY_WINDOW_ID", "3"), INSTALL])),
            None
        );
        assert_eq!(
            Kitty.focus_command(&client(&[("KITTY_WINDOW_ID", "3; rm"), listen, INSTALL])),
            None
        );
        assert_eq!(Kitty.focus_command(&client(&[listen, INSTALL])), None);
        assert_eq!(
            Kitty.focus_command(&client(&[("KITTY_WINDOW_ID", "3"), listen])),
            None
        );
    }
}
