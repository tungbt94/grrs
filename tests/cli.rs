use std::process::Command;
use predicates::prelude::*;
use assert_cmd::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
  let mut cmd = Command::main_binary()?;
  cmd.arg("foo bar")
      .arg("test/file/doesnt/exist");
  cmd.assert()
      .failure()
      .stderr(predicate::str::contains("No such file or directory"));
  Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
  let mut file = NamedTempFile::new()?;
  writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

  let mut cmd = Command::main_binary()?;
  cmd.arg("test")
      .arg(file.path());
  cmd.assert()
      .success()
      .stdout(predicate::str::contains("test\nAnother test"));
  Ok(())
}