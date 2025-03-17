use anyhow::Result;
use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::buckets::list::*;
use google_cloud_storage::http::buckets::Bucket;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::{download::Range, upload::*, Object};

use crate::domain::media;

// Implements Clone + Send + Sync. In theory thread-safe. Lets test that theory.
#[derive(Clone)]
pub struct GCSClient {
    gcs_client: Client,
}

// if you cannot create a client, try rerunning 'gcloud auth application-default login' from cli

// .with_auth() looks for a json credential file in:
// 1. A JSON file whose path is specified by the GOOGLE_APPLICATION_CREDENTIALS environment variable.
// 2. A JSON file in a location known to the gcloud command-line tool. On Windows, this is
//    %APPDATA%/gcloud/application_default_credentials.json. On other systems,
//    $HOME/.config/gcloud/application_default_credentials.json.

// If using method 2, token might be stale. Run 'gcloud auth application-default login' to reauth and refresh
impl GCSClient {
    pub async fn new() -> Result<Self> {
        let client = Client::new(ClientConfig::default().with_auth().await?);

        Ok(GCSClient { gcs_client: client })
    }
}

impl GCSClient {
    // for testing connection
    pub async fn list_buckets(self: &Self, project_id: &str) -> Result<Vec<Bucket>> {
        let resp = self
            .gcs_client
            .list_buckets(&ListBucketsRequest {
                project: project_id.to_string(),
                ..Default::default()
            })
            .await?;

        Ok(resp.items)
    }

    pub async fn upload_image(
        self: &Self,
        name: &str,
        image_data: Vec<u8>,
        content_type: &str,
        bucket_name: &str,
    ) -> Result<Object> {
        let upload_req = UploadObjectRequest {
            bucket: bucket_name.to_string().clone().to_lowercase(),
            ..Default::default()
        };

        let upload_type = UploadType::Simple(Media {
            name: name.to_owned().into(),
            content_type: content_type.to_owned().into(),
            content_length: None,
        });

        Ok(self
            .gcs_client
            .upload_object(&upload_req, image_data, &upload_type)
            .await?)
    }

    pub async fn get_image(self: &Self, name: &str, bucket_name: &str) -> Result<Vec<u8>> {
        let download_req = GetObjectRequest {
            bucket: bucket_name.to_string().clone().to_lowercase(),
            object: name.to_string(),
            ..Default::default()
        };

        Ok(self
            .gcs_client
            .download_object(&download_req, &Range::default())
            .await?)
    }

    pub async fn get_image_stream(
        self: &Self,
        media: &media::Media,
        bucket_name: &str,
    ) -> Result<Response> {
        let download_req = GetObjectRequest {
            bucket: bucket_name.to_string().clone().to_lowercase(),
            object: media.file_name.to_string(),
            ..Default::default()
        };

        let download_obj = self
            .gcs_client
            .download_streamed_object(&download_req, &Range::default())
            .await?;

        let mut headers = HeaderMap::new();
        headers.append(
            CONTENT_TYPE,
            HeaderValue::from_str(&media.content_type).unwrap(),
        );

        Ok((headers, Body::from_stream(download_obj)).into_response())
    }

    // TODO:
    // Implement streaming versions of upload AND download
}
