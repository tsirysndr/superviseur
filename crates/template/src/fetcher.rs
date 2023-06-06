use anyhow::{anyhow, Error};
use hyper::http::HeaderValue;
use hyper::Body;
use hyper::{body::HttpBody, client::HttpConnector, Uri};
use hyper_rustls::HttpsConnector;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use rustls::{OwnedTrustAnchor, RootCertStore};
use serde::Deserialize;
use std::str::FromStr;
use std::{fs::File, io::Write};
use surf::{Client, Config, Url};
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize)]
pub struct Release {
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
}

pub async fn download_template(url: &str) -> Result<(), Error> {
    let url = convert_into_github_url(url).await?;
    println!("Downloading template from {}", url.bright_green());
    let client = new_client();
    let response = client.get(Uri::from_str(&url)?).await?;

    // Ensure the response is successful
    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to download template: {}",
            response.status()
        ));
    }

    // Get the content length from the response headers
    let content_length = response
        .headers()
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    // Create a progress bar
    let progress_bar = ProgressBar::new(content_length);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // Open the destination file
    let mut tmpfile = NamedTempFile::new()?;
    let tmpfile_path = tmpfile.path().to_str().unwrap().to_string();

    // Download the asset and write it to the file
    let mut body = response.into_body();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        tmpfile.write_all(&chunk)?;

        // Increment the progress bar
        progress_bar.inc(chunk.len() as u64);
    }

    // Mark the progress bar as finished
    progress_bar.finish_with_message("Download complete");

    extract_zip(&tmpfile_path)?;
    Ok(())
}

async fn convert_into_github_url(url: &str) -> Result<String, Error> {
    let url = url.to_string();
    if url.contains("https://") {
        return Ok(url);
    }
    let url = get_latest_release(&url).await?;
    get_location(&url).await
}

async fn get_latest_release(repo: &str) -> Result<String, Error> {
    let base_url = "https://api.github.com";
    let client: Client = Config::new()
        .set_base_url(Url::parse(&base_url)?)
        .try_into()?;
    let latest = format!("/repos/{}/releases/latest", repo);
    let release = client
        .get(latest)
        .recv_json::<Release>()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get latest release: {}", e))?;
    Ok(release.zipball_url)
}

async fn get_location(url: &str) -> Result<String, Error> {
    let client = new_client();
    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri(url)
        .header("User-Agent", HeaderValue::from_static("Mozilla/5.0"))
        .body(Body::empty())?;
    let response = client.request(req).await.unwrap();

    // Ensure the response is successful
    if !response.status().is_redirection() {
        return Err(anyhow!(
            "Failed to download template: {}",
            response.status()
        ));
    }

    // Get the content length from the response headers
    let location = response
        .headers()
        .get(hyper::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    Ok(location.to_string())
}

fn new_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let tls = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Prepare the HTTPS connector
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();
    let client = hyper::client::Client::builder().build(https);
    client
}

fn extract_zip(path: &str) -> Result<(), Error> {
    let file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.file_names().for_each(|f| println!("{}", f));
    archive.extract(".")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_release() {
        let url = "tsirysndr/tunein";
        let zipball_url = get_latest_release(url).await.unwrap();
        assert!(zipball_url.starts_with("https://api.github.com/repos/tsirysndr/tunein/zipball/"));
    }

    #[tokio::test]
    async fn test_download_template() {
        let url = "tsirysndr/tunein";
        download_template(url).await.unwrap();
        assert!(true)
    }
}
