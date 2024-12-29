/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// Importing the entitiy to store 
/// metadata about files uploaded.
use actix_multipart::form::json::Json;

/// Importing the trait to make
/// multipart file uploads.
use actix_multipart::form::MultipartForm;

/// Importing the structure to store
/// temporary files to make
/// multipart file uploads.
use actix_multipart::form::tempfile::TempFile;

/// A structure that holds
/// the metadata on an uploaded
/// file.
#[derive(Deserialize, Debug)]
pub struct MetaData {
    pub name: String,
    pub api_token: String,
}

/// A structure to assist with
/// file upload via "actix-multipart".
#[derive(MultipartForm, Debug)]
pub struct FileUploadForm {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub metadata: Json<MetaData>
}