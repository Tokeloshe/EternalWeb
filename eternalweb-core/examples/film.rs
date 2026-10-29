// SPDX-License-Identifier: AGPL-3.0-or-later
//! The scenario behind docs/media/eternalweb.svg.
//!
//! Runs the working parts of eternalweb-core through one story and prints
//! every number the animation shows, as JSON:
//!
//!   cargo run -p eternalweb-core --release --example film > /tmp/film.json
//!
//! Archiving, content addressing, integrity checks, Ed25519 signatures,
//! reputation and curation are real library calls. Peer replication and
//! the network are not implemented yet, and the animation says so.

use eternalweb_core::archiving::archive_offline;
use eternalweb_core::incentives::{calculate_reputation, get_reputation_level, ContribType};
use eternalweb_core::search::{CurationManager, VoteType};
use eternalweb_core::security::{
    sign_data, verify_integrity, verify_signature, KeyType, PrivateKey, PublicKey,
};
use serde_json::json;

const V1: &str = "<!doctype html><html><head><title>Riverside Flood Gauge, Station 14</title></head>\
<body><h1>Riverside Flood Gauge, Station 14</h1><p>Peak level 4.82 m on 12 March. Readings every \
15 minutes since 1998, published by the county for anyone to download.</p></body></html>";
const V2: &str = "<!doctype html><html><head><title>Riverside Flood Gauge, Station 14</title></head>\
<body><h1>Riverside Flood Gauge, Station 14</h1><p>Peak level 4.82 m on 12 March. Readings every \
15 minutes since 1998. Historical data is no longer available.</p></body></html>";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    std::env::set_var("ETERNALWEB_DB_PATH", dir.path().join("db"));
    let write = |name: &str, body: &str| -> anyhow::Result<String> {
        let p = dir.path().join(name);
        std::fs::write(&p, body)?;
        Ok(p.to_string_lossy().into_owned())
    };

    // the archive and its address
    let cid1 = archive_offline(&write("station14-2026-03.html", V1)?).await?;
    let cid2 = archive_offline(&write("station14-2026-09.html", V2)?).await?;
    // one character different: 4.82 m becomes 4.83 m
    let one_char = V1.replacen("4.82 m", "4.83 m", 1);
    let cid_one = archive_offline(&write("station14-edit.html", &one_char)?).await?;

    let ok_original = verify_integrity(&cid1, V1.as_bytes())?;
    let ok_tampered = verify_integrity(&cid1, one_char.as_bytes())?;

    // signatures: a fixed demo key so the film is reproducible
    let sk = ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]);
    let pk = PublicKey { bytes: sk.verifying_key().to_bytes().to_vec(), key_type: KeyType::Ed25519 };
    let priv_key = PrivateKey { bytes: sk.to_bytes().to_vec(), key_type: KeyType::Ed25519 };
    let sig = sign_data(cid1.to_bytes().as_slice(), &priv_key)?;
    let sig_ok = verify_signature(cid1.to_bytes().as_slice(), &sig, &pk)?;
    let sig_forged = verify_signature(cid_one.to_bytes().as_slice(), &sig, &pk)?;

    // the community around the archive
    let people = [
        ("Amara", vec![(ContribType::Archive, 41), (ContribType::Hosting, 120), (ContribType::Curation, 9)]),
        ("Tomás", vec![(ContribType::Archive, 6), (ContribType::Curation, 22)]),
        ("Kenji", vec![(ContribType::Hosting, 38), (ContribType::Development, 4)]),
        ("Lindiwe", vec![(ContribType::Archive, 3)]),
    ];
    let reputation: Vec<_> = people.iter().map(|(name, c)| {
        let r = calculate_reputation(c.clone());
        json!({ "name": name, "points": r, "level": get_reputation_level(r) })
    }).collect();

    let curation = CurationManager::new();
    let key = cid1.to_string();
    let votes = [VoteType::Upvote, VoteType::Upvote, VoteType::Upvote, VoteType::Downvote,
                 VoteType::Upvote, VoteType::Upvote, VoteType::Upvote, VoteType::Flag];
    let mut trail = vec![];
    for v in votes {
        let label = match v { VoteType::Upvote => "up", VoteType::Downvote => "down", VoteType::Flag => "flag" };
        curation.record_vote(&key, v);
        trail.push(json!({ "vote": label, "score": curation.get_quality_score(&key) }));
    }

    let out = json!({
        "page": { "title": "Riverside Flood Gauge, Station 14", "bytes_v1": V1.len(), "bytes_v2": V2.len() },
        "cid_v1": cid1.to_string(),
        "cid_v2": cid2.to_string(),
        "cid_one_char": cid_one.to_string(),
        "integrity": { "original": ok_original, "one_char_changed": ok_tampered },
        "signature": { "public_key": hex::encode(&pk.bytes), "sig": hex::encode(&sig),
                        "valid": sig_ok, "on_other_cid": sig_forged },
        "reputation": reputation,
        "curation": trail,
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
