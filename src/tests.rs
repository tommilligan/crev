use super::*;
use common_failures::prelude::*;
use level::Level;
use proof::Content;

use std::path::PathBuf;

#[test]
fn sign_proof_review() -> Result<()> {
    let id = id::OwnId::generate("John Doe <doe@john.com>".into());

    let review = review::ReviewBuilder::default()
        .from("abcdf".into())
        .from_type("crev".into())
        .from_url("https://github.com/someone/crev-trust".into())
        .revision(Some("foobar".into()))
        .revision_type("git".into())
        .project_url("https://github.com/someone/somelib".into())
        .comment(Some("comment".into()))
        .thoroughness(Level::Low)
        .understanding(Level::Low)
        .trust(Level::Low)
        .files(vec![
            review::ReviewFile {
                path: PathBuf::from("foo.x"),
                digest: vec![1, 2, 3, 4],
                digest_type: "sha256".into(),
            },
            review::ReviewFile {
                path: PathBuf::from("foo.x"),
                digest: vec![1, 2, 3, 4],
                digest_type: "sha256".into(),
            },
        ]).build()
        .map_err(|e| format_err!("{}", e))?;

    println!("{}", review);
    let proof = review.sign(&id)?;
    println!("{}", proof);

    let parsed_review = proof.parse_content()?;
    println!("{}", parsed_review);

    Ok(())
}
