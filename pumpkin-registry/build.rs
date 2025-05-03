use std::{
    fs::{File, create_dir_all},
    io::copy,
    path::Path,
};

use eyre::{Result, WrapErr, eyre};
use reqwest::blocking::Client;
use serde::Deserialize;

const VERSION: &str = "1.21.5";
const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const SERVER_JAR_CACHE_DIR: &str = "cache/server_jars";

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<VersionEntry>,
}

#[derive(Deserialize)]
struct VersionEntry {
    id: String,
    url: String,
}

#[derive(Deserialize)]
struct VersionDetails {
    downloads: VersionDownloads,
}

#[derive(Deserialize)]
struct VersionDownloads {
    server: DownloadInfo,
}

#[derive(Deserialize)]
struct DownloadInfo {
    url: String,
}

fn main() -> Result<()> {
    let reqwest_client = Client::new();

    let version_manifest_json = reqwest_client
        .get(VERSION_MANIFEST_URL)
        .send()
        .wrap_err("Failed to fetch version manifest")?
        .text()
        .wrap_err("Failed to read version manifest response")?;

    let version_manifest: VersionManifest = serde_json::from_str(&version_manifest_json)
        .wrap_err("Failed to parse version manifest")?;

    let version_entry = version_manifest
        .versions
        .iter()
        .find(|v| v.id == VERSION)
        .ok_or_else(|| eyre!("Specified Minecraft version ({VERSION}) not found"))?;

    let version_details_json = reqwest_client
        .get(&version_entry.url)
        .send()
        .wrap_err("Failed to fetch version details")?
        .text()
        .wrap_err("Failed to read version details response")?;

    let version_details: VersionDetails =
        serde_json::from_str(&version_details_json).wrap_err("Failed to parse version details")?;

    let server_jar_cache_path = Path::new(SERVER_JAR_CACHE_DIR).join(format!("{VERSION}.jar"));
    if !server_jar_cache_path.exists() {
        create_dir_all(SERVER_JAR_CACHE_DIR)
            .wrap_err_with(|| format!("Failed to create directory {SERVER_JAR_CACHE_DIR}"))?;

        let mut response = reqwest_client
            .get(&version_details.downloads.server.url)
            .send()
            .wrap_err("Failed to fetch server jar")?;
        let mut file =
            File::create(&server_jar_cache_path).wrap_err("Failed to create server jar file")?;
        copy(&mut response, &mut file).wrap_err("Failed to write to server jar file")?;
    }

    Ok(())
}
