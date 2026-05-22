use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsomics-bam-to-fastq"))
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/small.bam")
}

fn samtools_available() -> bool {
    Command::new("samtools")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// Default extraction must be byte-identical to `samtools fastq`: same reads
// (secondary/supplementary excluded), file order, /1 /2 mate suffixes.
#[test]
fn matches_samtools_fastq() {
    if !samtools_available() {
        eprintln!("skipping: samtools not found");
        return;
    }
    let ours = bin().arg(fixture()).output().unwrap();
    assert!(
        ours.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&ours.stderr)
    );
    let theirs = Command::new("samtools")
        .arg("fastq")
        .arg(fixture())
        .output()
        .unwrap();
    assert!(theirs.status.success());
    assert_eq!(
        String::from_utf8_lossy(&ours.stdout),
        String::from_utf8_lossy(&theirs.stdout)
    );
}
