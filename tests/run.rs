use anyhow::Result;
use assert_cmd::prelude::*;
use std::{collections::HashSet, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

// Mock structure
struct Mock {
    // Socket
    sock: PathBuf,
    // Temporary directory
    _dir: TempDir,
}

impl Mock {
    // Create mock
    fn new() -> Result<Mock> {
        let dir = tempdir()?;
        let sock = dir.path().join("sifis.sock");

        // Run server
        let _server = Command::new("sifis-runtime-mock")
            .env("SIFIS_SERVER", &sock)
            .spawn()?;

        Ok(Mock { sock, _dir: dir })
    }
}

// Create test function
#[test]
fn default_mock() {
    let mock = Mock::new().unwrap();

    // Run third-party-lamp device
    let mut cmd = Command::cargo_bin("third-party-lamp-controller").unwrap();

    // Get output
    let out = cmd.env("SIFIS_SERVER", &mock.sock).output().unwrap();

    let s = String::from_utf8(out.stdout).unwrap();

    // Get state of lamps as lines
    let lines = s.lines().collect::<HashSet<_>>();

    let expected: HashSet<&str> = [
        "lamp1           Off     0     ",
        "lamp2           Off     0     ",
    ]
    .into();
    assert_eq!(expected, lines);
}
