use assert_cmd::Command;

#[test]
fn help_flag() {
    let mut cmd = Command::cargo_bin("receiptgo").unwrap();
    let output = r#"Download receipts from RingGo

Usage: receiptgo --username <USERNAME> --password <PASSWORD> --client-secret <CLIENT_SECRET>

Options:
  -u, --username <USERNAME>            [env: RINGGO_USERNAME=]
  -p, --password <PASSWORD>            [env: RINGGO_PASSWORD=]
  -c, --client-secret <CLIENT_SECRET>  [env: RINGGO_CLIENT_SECRET=]
  -h, --help                           Print help
  -V, --version                        Print version
"#;

    let assert = cmd.arg("--help").assert();
    assert.success().code(0).stdout(output);

    let assert = cmd.arg("-h").assert();
    assert.success().code(0).stdout(output);
}

#[test]
fn version_flag() {
    let mut cmd = Command::cargo_bin("receiptgo").unwrap();
    let output = "receiptgo 0.1.0\n";

    let assert = cmd.arg("--version").assert();
    assert.success().code(0).stdout(output);

    let assert = cmd.arg("-V").assert();
    assert.success().code(0).stdout(output);
}
