// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Structure containing the estimated age range, in years, for a face.</p> <p>Rekognition estimates an age-range for faces detected in the input image. Estimated age ranges can overlap; a face of a 5 year old may have an estimated range of 4-6 whilst the face of a 6 year old may have an estimated range of 4-8.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AgeRange {
    /// <p>The highest estimated age.</p>
    #[serde(rename = "High")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    /// <p>The lowest estimated age.</p>
    #[serde(rename = "Low")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
}

/// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Beard {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has beard or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p><p>Identifies the bounding box around the object, face or text. The <code>left</code> (x-coordinate) and <code>top</code> (y-coordinate) are coordinates representing the top and left sides of the bounding box. Note that the upper-left corner of the image is the origin (0,0). </p> <p>The <code>top</code> and <code>left</code> values returned are ratios of the overall image size. For example, if the input image is 700x200 pixels, and the top-left coordinate of the bounding box is 350x50 pixels, the API returns a <code>left</code> value of 0.5 (350/700) and a <code>top</code> value of 0.25 (50/200).</p> <p>The <code>width</code> and <code>height</code> values represent the dimensions of the bounding box as a ratio of the overall image dimension. For example, if the input image is 700x200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. </p> <note> <p> The bounding box coordinates can have negative values. For example, if Amazon Rekognition is able to detect a face that is at the image edge and is only partially visible, the service can return coordinates that are outside the image bounds and, depending on the image edge, you might get negative values or values greater than 1 for the <code>left</code> or <code>top</code> values. </p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BoundingBox {
    /// <p>Height of the bounding box as a ratio of the overall image height.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    /// <p>Left coordinate of the bounding box as a ratio of overall image width.</p>
    #[serde(rename = "Left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    /// <p>Top coordinate of the bounding box as a ratio of overall image height.</p>
    #[serde(rename = "Top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    /// <p>Width of the bounding box as a ratio of the overall image width.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

/// <p>Provides information about a celebrity recognized by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Celebrity {
    /// <p>Provides information about the celebrity's face, such as its location on the image.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>A unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The confidence, in percentage, that Rekognition has that the recognized face is the celebrity.</p>
    #[serde(rename = "MatchConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_confidence: Option<f32>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional information about the celebrity. If there is no additional information about the celebrity, this list is empty.</p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a recognized celebrity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CelebrityDetail {
    /// <p>Bounding box around the body of a celebrity.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>The confidence, in percentage, that Amazon Rekognition has that the recognized face is the celebrity. </p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Face details for the recognized celebrity.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>The unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a detected celebrity and the time the celebrity was detected in a stored video. For more information, see GetCelebrityRecognition in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CelebrityRecognition {
    /// <p>Information about a recognized celebrity.</p>
    #[serde(rename = "Celebrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity: Option<CelebrityDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the celebrity was recognized.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides information about a face in a target image that matches the source image face analysed by <code>CompareFaces</code>. The <code>Face</code> property contains the bounding box of the face in the target image. The <code>Similarity</code> property is the confidence that the source image face matches the face in the bounding box.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CompareFacesMatch {
    /// <p>Provides face metadata (bounding box and confidence that the bounding box actually contains a face).</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>Level of confidence that the faces match.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CompareFacesRequest {
    /// <p>The minimum level of confidence in the face matches that a match must meet to be included in the <code>FaceMatches</code> array.</p>
    #[serde(rename = "SimilarityThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "SourceImage")]
    pub source_image: Image,
    /// <p>The target image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "TargetImage")]
    pub target_image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CompareFacesResponse {
    /// <p>An array of faces in the target image that match the source image face. Each <code>CompareFacesMatch</code> object provides the bounding box, the confidence level that the bounding box contains a face, and the similarity score for the face in the bounding box and the face in the source image.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<CompareFacesMatch>>,
    /// <p>The face in the source image that was used for comparison.</p>
    #[serde(rename = "SourceImageFace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_face: Option<ComparedSourceImageFace>,
    /// <p><p> The orientation of the source image (counterclockwise direction). If your application displays the source image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>SourceImageFace</code> represent the location of the face before the image orientation is corrected. </p> <note> <p>If the source image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image&#39;s orientation. If the Exif metadata for the source image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>SourceImageFace</code> bounding box coordinates represent the location of the face after Exif metadata is used to correct the orientation. Images in .png format don&#39;t contain Exif metadata.</p> </note></p>
    #[serde(rename = "SourceImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_orientation_correction: Option<String>,
    /// <p><p> The orientation of the target image (in counterclockwise direction). If your application displays the target image, you can use this value to correct the orientation of the image. The bounding box coordinates returned in <code>FaceMatches</code> and <code>UnmatchedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the target image is in .jpg format, it might contain Exif metadata that includes the orientation of the image. If the Exif metadata for the target image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the bounding box coordinates in <code>FaceMatches</code> and <code>UnmatchedFaces</code> represent the location of the face after Exif metadata is used to correct the orientation. Images in .png format don&#39;t contain Exif metadata.</p> </note></p>
    #[serde(rename = "TargetImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_image_orientation_correction: Option<String>,
    /// <p>An array of faces in the target image that did not match the source image face.</p>
    #[serde(rename = "UnmatchedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides face metadata for target image faces that are analysed by <code>CompareFaces</code> and <code>RecognizeCelebrities</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComparedFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Level of confidence that what the bounding box contains is a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>An array of facial landmarks.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies face image brightness and sharpness. </p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
}

/// <p>Type that describes the face Amazon Rekognition chose to compare with the faces in the target. This contains a bounding box for the selected face and confidence level that the bounding box contains a face. Note that Amazon Rekognition selects the largest face in the source image for this comparison. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComparedSourceImageFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the selected bounding box contains a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// <p>Information about a moderation label detection in a stored video.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ContentModerationDetection {
    /// <p>The moderation label detected by in the stored video.</p>
    #[serde(rename = "ModerationLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_label: Option<ModerationLabel>,
    /// <p>Time, in milliseconds from the beginning of the video, that the moderation label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCollectionRequest {
    /// <p>ID for the collection that you are creating.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCollectionResponse {
    /// <p>Amazon Resource Name (ARN) of the collection. You can use this to manage permissions on your resources. </p>
    #[serde(rename = "CollectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    /// <p>Version number of the face detection model associated with the collection you are creating.</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>HTTP status code indicating the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStreamProcessorRequest {
    /// <p>Kinesis video stream stream that provides the source streaming video. If you are using the AWS CLI, the parameter name is <code>StreamProcessorInput</code>.</p>
    #[serde(rename = "Input")]
    pub input: StreamProcessorInput,
    /// <p>An identifier you assign to the stream processor. You can use <code>Name</code> to manage the stream processor. For example, you can get the current status of the stream processor by calling . <code>Name</code> is idempotent. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Kinesis data stream stream to which Amazon Rekognition Video puts the analysis results. If you are using the AWS CLI, the parameter name is <code>StreamProcessorOutput</code>.</p>
    #[serde(rename = "Output")]
    pub output: StreamProcessorOutput,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Face recognition input parameters to be used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    pub settings: StreamProcessorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateStreamProcessorResponse {
    /// <p>ARN for the newly create stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCollectionRequest {
    /// <p>ID of the collection to delete.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCollectionResponse {
    /// <p>HTTP status code that indicates the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFacesRequest {
    /// <p>Collection from which to remove the specific faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of face IDs to delete.</p>
    #[serde(rename = "FaceIds")]
    pub face_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFacesResponse {
    /// <p>An array of strings (face IDs) of the faces that were deleted.</p>
    #[serde(rename = "DeletedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_faces: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStreamProcessorRequest {
    /// <p>The name of the stream processor you want to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStreamProcessorRequest {
    /// <p>Name of the stream processor for which you want information.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeStreamProcessorResponse {
    /// <p>Date and time the stream processor was created</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>Kinesis video stream that provides the source streaming video.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<StreamProcessorInput>,
    /// <p>The time, in Unix format, the stream processor was last updated. For example, when the stream processor moves from a running state to a failed state, or when the user starts or stops the stream processor.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>Name of the stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Kinesis data stream to which Amazon Rekognition Video puts the analysis results.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<StreamProcessorOutput>,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Face recognition input parameters that are being used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<StreamProcessorSettings>,
    /// <p>Current status of the stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Detailed status message about the stream processor.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>ARN of the stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectFacesRequest {
    /// <p>An array of facial attributes you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code> and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned but the operation will take longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectFacesResponse {
    /// <p>Details of each face found in the image. </p>
    #[serde(rename = "FaceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_details: Option<Vec<FaceDetail>>,
    /// <p><p> The orientation of the input image (counter-clockwise direction). If your application displays the image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>FaceDetails</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image&#39;s orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>FaceDetails</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don&#39;t contain Exif metadata.</p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectLabelsRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of labels you want the service to return in the response. The service returns the specified number of highest confidence labels. </p>
    #[serde(rename = "MaxLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_labels: Option<i64>,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with confidence lower than this specified value.</p> <p>If <code>MinConfidence</code> is not specified, the operation returns labels with a confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectLabelsResponse {
    /// <p>An array of labels for the real-world objects detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    /// <p><p> The orientation of the input image (counter-clockwise direction). If your application displays the image, you can use this value to correct the orientation. If Amazon Rekognition detects that the input image was rotated (for example, by 90 degrees), it first corrects the orientation before detecting the labels. </p> <note> <p>If the input image Exif metadata populates the orientation field, Amazon Rekognition does not perform orientation correction and the value of OrientationCorrection will be null.</p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectModerationLabelsRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectModerationLabelsResponse {
    /// <p>Array of detected Moderation labels and the time, in millseconds from the start of the video, they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ModerationLabel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectTextRequest {
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes. </p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectTextResponse {
    /// <p>An array of text that was detected in the input image.</p>
    #[serde(rename = "TextDetections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detections: Option<Vec<TextDetection>>,
}

/// <p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Emotion {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Type of emotion detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EyeOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the eyes on the face are open.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Eyeglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing eye glasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Face {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree).</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Identifier that you assign to all the faces in the input image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the face.</p>
    #[serde(rename = "FaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the input image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// <p>Structure containing attributes of the face that the algorithm detected.</p> <p>A <code>FaceDetail</code> object contains either the default facial attributes or all facial attributes. The default attributes are <code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>.</p> <p> is the only Amazon Rekognition Video stored video operation that can return a <code>FaceDetail</code> object with all attributes. To specify which attributes to return, use the <code>FaceAttributes</code> input parameter for . The following Amazon Rekognition Video operations return only the default attributes. The corresponding Start operations don't have a <code>FaceAttributes</code> input parameter.</p> <ul> <li> <p>GetCelebrityRecognition</p> </li> <li> <p>GetPersonTracking</p> </li> <li> <p>GetFaceSearch</p> </li> </ul> <p>The Amazon Rekognition Image and operations can return all facial attributes. To specify which attributes to return, use the <code>Attributes</code> input parameter for <code>DetectFaces</code>. For <code>IndexFaces</code>, use the <code>DetectAttributes</code> input parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceDetail {
    /// <p>The estimated age range, in years, for the face. Low represents the lowest estimated age and High represents the highest estimated age.</p>
    #[serde(rename = "AgeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRange>,
    /// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
    #[serde(rename = "Beard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beard: Option<Beard>,
    /// <p>Bounding box of the face. Default attribute.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree). Default attribute.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY. </p>
    #[serde(rename = "Emotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotions: Option<Vec<Emotion>>,
    /// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
    #[serde(rename = "Eyeglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyeglasses: Option<Eyeglasses>,
    /// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
    #[serde(rename = "EyesOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyes_open: Option<EyeOpen>,
    /// <p>Gender of the face and the confidence level in the determination.</p>
    #[serde(rename = "Gender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// <p>Indicates the location of landmarks on the face. Default attribute.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
    #[serde(rename = "MouthOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_open: Option<MouthOpen>,
    /// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
    #[serde(rename = "Mustache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mustache: Option<Mustache>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw. Default attribute.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies image brightness and sharpness. Default attribute.</p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
    #[serde(rename = "Smile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<Smile>,
    /// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
    #[serde(rename = "Sunglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunglasses: Option<Sunglasses>,
}

/// <p>Information about a face detected in a video analysis request and the time the face was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceDetection {
    /// <p>The face properties for the detected face.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Time, in milliseconds from the start of the video, that the face was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides face metadata. In addition, it also provides the confidence in the match of this face with the input face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceMatch {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Confidence in the match of this face with the input face.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

/// <p>Object containing both the face metadata (stored in the back-end database) and facial attributes that are detected but aren't stored in the database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceRecord {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Structure containing attributes of the face that the algorithm detected.</p>
    #[serde(rename = "FaceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

/// <p>Input face recognition parameters for an Amazon Rekognition stream processor. <code>FaceRecognitionSettings</code> is a request parameter for .</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaceSearchSettings {
    /// <p>The ID of a collection that contains faces that you want to search for.</p>
    #[serde(rename = "CollectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// <p>Minimum face match confidence score that must be met to return a result for a recognized face. Default is 70. 0 is the lowest confidence. 100 is the highest confidence.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
}

/// <p>Gender of the face and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Gender {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Gender of the face.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about where text detected by is located on an image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Geometry {
    /// <p>An axis-aligned coarse representation of the detected text's location on the image.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Within the bounding box, a fine-grained polygon around the detected text.</p>
    #[serde(rename = "Polygon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCelebrityInfoRequest {
    /// <p>The ID for the celebrity. You get the celebrity ID from a call to the operation, which recognizes celebrities in an image. </p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCelebrityInfoResponse {
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCelebrityRecognitionRequest {
    /// <p>Job identifier for the required celebrity recognition analysis. You can get the job identifer from a call to <code>StartCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more recognized celebrities to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of celebrities. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for celebrities returned in <code>Celebrities</code> field. Specify <code>ID</code> to sort by the celebrity identifier, specify <code>TIMESTAMP</code> to sort by the time the celebrity was recognized.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCelebrityRecognitionResponse {
    /// <p>Array of celebrities recognized in the video.</p>
    #[serde(rename = "Celebrities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrities: Option<Vec<CelebrityRecognition>>,
    /// <p>The current status of the celebrity recognition job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContentModerationRequest {
    /// <p>The identifier for the content moderation job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of content moderation labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>ModerationLabelDetections</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetContentModerationResponse {
    /// <p>The current status of the content moderation job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The detected moderation labels and the time(s) they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ContentModerationDetection>>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of moderation labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from <code>GetContentModeration</code>. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFaceDetectionRequest {
    /// <p>Unique identifier for the face detection job. The <code>JobId</code> is returned from <code>StartFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more faces to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFaceDetectionResponse {
    /// <p>An array of faces detected in the video. Each element contains a detected face's details and the time, in milliseconds from the start of the video, the face was detected. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<FaceDetection>>,
    /// <p>The current status of the face detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFaceSearchRequest {
    /// <p>The job identifer for the search request. You get the job identifier from an initial call to <code>StartFaceSearch</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more search results to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for grouping faces in the response. Use <code>TIMESTAMP</code> to group faces by the time that they are recognized. Use <code>INDEX</code> to sort by recognized faces. </p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFaceSearchResponse {
    /// <p>The current status of the face search job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of persons, , in the video whose face(s) match the face(s) in an Amazon Rekognition collection. It also includes time information for when persons are matched in the video. You specify the input collection in an initial call to <code>StartFaceSearch</code>. Each <code>Persons</code> element includes a time the person was matched, face match details (<code>FaceMatches</code>) for matching faces in the collection, and person information (<code>Person</code>) for the matched person. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonMatch>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLabelDetectionRequest {
    /// <p>Job identifier for the label detection operation for which you want results returned. You get the job identifer from an initial call to <code>StartlabelDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more labels to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Labels</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLabelDetectionResponse {
    /// <p>The current status of the label detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>An array of labels detected in the video. Each element contains the detected label and the time, in milliseconds from the start of the video, that the label was detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDetection>>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPersonTrackingRequest {
    /// <p>The identifier for a job that tracks persons in a video. You get the <code>JobId</code> from a call to <code>StartPersonTracking</code>. </p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more persons to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Persons</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time persons are detected. Use <code>INDEX</code> to sort by the tracked persons. If you sort by <code>INDEX</code>, the array elements for each person are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPersonTrackingResponse {
    /// <p>The current status of the person tracking job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of the persons detected in the video and the times they are tracked throughout the video. An array element will exist for each time the person is tracked. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonDetection>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

/// <p>Provides the input image either as bytes or an S3 object.</p> <p>You pass image bytes to a Rekognition API operation by using the <code>Bytes</code> property. For example, you would use the <code>Bytes</code> property to pass an image loaded from a local file system. Image bytes passed by using the <code>Bytes</code> property must be base64-encoded. Your code may not need to encode image bytes if you are using an AWS SDK to call Rekognition API operations. </p> <p>For more information, see Analyzing an Image Loaded from a Local File System in the Amazon Rekognition Developer Guide.</p> <p> You pass images stored in an S3 bucket to a Rekognition API operation by using the <code>S3Object</code> property. Images stored in an S3 bucket do not need to be base64-encoded.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes using the Bytes property is not supported. You must first upload the image to an Amazon S3 bucket and then call the operation using the S3Object property.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Image {
    /// <p>Blob of image bytes up to 5 MBs.</p>
    #[serde(rename = "Bytes")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub bytes: Option<Vec<u8>>,
    /// <p>Identifies an S3 object as the image source.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Identifies face image brightness and sharpness. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImageQuality {
    /// <p>Value representing brightness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a brighter face image.</p>
    #[serde(rename = "Brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    /// <p>Value representing sharpness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a sharper face image.</p>
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IndexFacesRequest {
    /// <p>The ID of an existing collection to which you want to add the faces that are detected in the input images.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of facial attributes that you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code> and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned but the operation will take longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "DetectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_attributes: Option<Vec<String>>,
    /// <p>ID you want to assign to all the faces detected in the image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IndexFacesResponse {
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of faces detected and added to the collection. For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide. </p>
    #[serde(rename = "FaceRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_records: Option<Vec<FaceRecord>>,
    /// <p><p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>FaceRecords</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in jpeg format, it might contain exchangeable image (Exif) metadata. If so, and the Exif metadata populates the orientation field, the value of <code>OrientationCorrection</code> is null and the bounding box coordinates in <code>FaceRecords</code> represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don&#39;t contain Exif metadata.</p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

/// <p>The Kinesis data stream Amazon Rekognition to which the analysis results of a Amazon Rekognition stream processor are streamed. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisDataStream {
    /// <p>ARN of the output Amazon Kinesis Data Streams stream.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Kinesis video stream stream that provides the source streaming video for a Amazon Rekognition Video stream processor. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisVideoStream {
    /// <p>ARN of the Kinesis video stream stream that streams the source video.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Structure containing details about the detected label, including name, and level of confidence.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Label {
    /// <p>Level of confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The name (label) of the object.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a label detected in a video analysis request and the time the label was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelDetection {
    /// <p>Details about the detected label.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// <p>Time, in milliseconds from the start of the video, that the label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Indicates the location of the landmark on the face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Landmark {
    /// <p>Type of the landmark.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>x-coordinate from the top left of the landmark expressed as the ratio of the width of the image. For example, if the images is 700x200 and the x-coordinate of the landmark is at 350 pixels, this value is 0.5. </p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>y-coordinate from the top left of the landmark expressed as the ratio of the height of the image. For example, if the images is 700x200 and the y-coordinate of the landmark is at 100 pixels, this value is 0.5.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCollectionsRequest {
    /// <p>Maximum number of collection IDs to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListCollectionsResponse {
    /// <p>An array of collection IDs.</p>
    #[serde(rename = "CollectionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    /// <p>Version numbers of the face detection models associated with the collections in the array <code>CollectionIds</code>. For example, the value of <code>FaceModelVersions[2]</code> is the version number for the face detection model used by the collection in <code>CollectionId[2]</code>.</p>
    #[serde(rename = "FaceModelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_versions: Option<Vec<String>>,
    /// <p>If the result is truncated, the response provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFacesRequest {
    /// <p>ID of the collection from which to list the faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>Maximum number of faces to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFacesResponse {
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of <code>Face</code> objects. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<Face>>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListStreamProcessorsRequest {
    /// <p>Maximum number of stream processors you want Amazon Rekognition Video to return in the response. The default is 1000. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more stream processors to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListStreamProcessorsResponse {
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of stream processors that you have created.</p>
    #[serde(rename = "StreamProcessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processors: Option<Vec<StreamProcessor>>,
}

/// <p>Provides information about a single type of moderated content found in an image or video. Each type of moderated content has a label within a hierarchical taxonomy. For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModerationLabel {
    /// <p>Specifies the confidence that Amazon Rekognition has that the label has been correctly identified.</p> <p>If you don't specify the <code>MinConfidence</code> parameter in the call to <code>DetectModerationLabels</code>, the operation returns labels with a confidence value greater than or equal to 50 percent.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The label name for the type of content detected in the image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name for the parent label. Labels at the top-level of the hierarchy have the parent label <code>""</code>.</p>
    #[serde(rename = "ParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
}

/// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MouthOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the mouth on the face is open or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Mustache {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has mustache or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status of a video analysis operation. For more information, see <a>api-video</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotificationChannel {
    /// <p>The ARN of an IAM role that gives Amazon Rekognition publishing permissions to the Amazon SNS topic. </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The Amazon SNS topic to which Amazon Rekognition to posts the completion status.</p>
    #[serde(rename = "SNSTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>Details about a person detected in a video analysis request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonDetail {
    /// <p>Bounding box around the detected person.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Face details for the detected person.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Identifier for the person detected person within a video. Use to keep track of the person throughout the video. The identifier is not stored by Amazon Rekognition.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

/// <p>Details and tracking information for a single time a person is tracked in a video. Amazon Rekognition operations that track persons return an array of <code>PersonDetection</code> objects with elements for each time a person is tracked in a video. </p> <p>For more information, see API_GetPersonTracking in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonDetection {
    /// <p>Details about a person tracked in a video.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the person was tracked.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Information about a person whose face matches a face(s) in a Amazon Rekognition collection. Includes information about the faces in the Amazon Rekognition collection (, information about the person (<a>PersonDetail</a>) and the timestamp for when the person was detected in a video. An array of <code>PersonMatch</code> objects is returned by . </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonMatch {
    /// <p>Information about the faces in the input collection that match the face of a person in the video.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Information about the matched person.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the beginning of the video, that the person was matched in the video.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>The X and Y coordinates of a point on an image. The X and Y values returned are ratios of the overall image size. For example, if the input image is 700x200 and the operation returns X=0.5 and Y=0.25, then the point is at the (350,50) pixel coordinate on the image.</p> <p>An array of <code>Point</code> objects, <code>Polygon</code>, is returned by . <code>Polygon</code> represents a fine-grained polygon around detected text. For more information, see Geometry in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Point {
    /// <p>The value of the X coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>The value of the Y coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

/// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Pose {
    /// <p>Value representing the face rotation on the pitch axis.</p>
    #[serde(rename = "Pitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    /// <p>Value representing the face rotation on the roll axis.</p>
    #[serde(rename = "Roll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f32>,
    /// <p>Value representing the face rotation on the yaw axis.</p>
    #[serde(rename = "Yaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RecognizeCelebritiesRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecognizeCelebritiesResponse {
    /// <p>Details about each celebrity found in the image. Amazon Rekognition can detect a maximum of 15 celebrities in an image.</p>
    #[serde(rename = "CelebrityFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity_faces: Option<Vec<Celebrity>>,
    /// <p><p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct the orientation. The bounding box coordinates returned in <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image&#39;s orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don&#39;t contain Exif metadata. </p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    /// <p>Details about each unrecognized face in the image.</p>
    #[serde(rename = "UnrecognizedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrecognized_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides the S3 bucket name and object name.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3Object {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>S3 object key name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the bucket is versioning enabled, you can specify the object version. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchFacesByImageRequest {
    /// <p>ID of the collection to search.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>(Optional) Specifies the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchFacesByImageResponse {
    /// <p>An array of faces that match the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>The bounding box around the face in the input image that Amazon Rekognition used for the search.</p>
    #[serde(rename = "SearchedFaceBoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_bounding_box: Option<BoundingBox>,
    /// <p>The level of confidence that the <code>searchedFaceBoundingBox</code>, contains a face.</p>
    #[serde(rename = "SearchedFaceConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchFacesRequest {
    /// <p>ID of the collection the face belongs to.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>ID of a face to find matches for in the collection.</p>
    #[serde(rename = "FaceId")]
    pub face_id: String,
    /// <p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchFacesResponse {
    /// <p>An array of faces that matched the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>ID of the face that was searched for matches in a collection.</p>
    #[serde(rename = "SearchedFaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_id: Option<String>,
}

/// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Smile {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is smiling or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCelebrityRecognitionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartCelebrityRecognition</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the celebrity recognition analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to recognize celebrities. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartCelebrityRecognitionResponse {
    /// <p>The identifier for the celebrity recognition analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartContentModerationRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartContentModeration</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition must have in order to return a moderated content label. Confidence represents how certain Amazon Rekognition is that the moderated content is correctly identified. 0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition doesn't return any moderated content labels with a confidence level lower than this specified value.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the content moderation analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to moderate content. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartContentModerationResponse {
    /// <p>The identifier for the content moderation analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartFaceDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The face attributes you want returned.</p> <p> <code>DEFAULT</code> - The following subset of facial attributes are returned: BoundingBox, Confidence, Pose, Quality and Landmarks. </p> <p> <code>ALL</code> - All facial attributes are returned.</p>
    #[serde(rename = "FaceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_attributes: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the face detection operation.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect faces. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartFaceDetectionResponse {
    /// <p>The identifier for the face detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartFaceSearchRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceSearch</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>ID of the collection that contains the faces you want to search for.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>The minimum confidence in the person match to return. For example, don't return any matches where confidence in matches is less than 70%. </p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the search. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video you want to search. The video must be stored in an Amazon S3 bucket. </p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartFaceSearchResponse {
    /// <p>The identifier for the search job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceSearch</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartLabelDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartLabelDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition Video must have in order to return a detected label. Confidence represents how certain Amazon Rekognition is that a label is correctly identified.0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition Video doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the label detection operation to. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect labels. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartLabelDetectionResponse {
    /// <p>The identifier for the label detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetLabelDetection</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartPersonTrackingRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartPersonTrackingResponse {
    /// <p>The identifier for the person detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetPersonTracking</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartStreamProcessorRequest {
    /// <p>The name of the stream processor to start processing.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopStreamProcessorRequest {
    /// <p>The name of a stream processor created by .</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopStreamProcessorResponse {}

/// <p>An object that recognizes faces in a streaming video. An Amazon Rekognition stream processor is created by a call to . The request parameters for <code>CreateStreamProcessor</code> describe the Kinesis video stream source for the streaming video, face recognition parameters, and where to stream the analysis resullts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StreamProcessor {
    /// <p>Name of the Amazon Rekognition stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Current status of the Amazon Rekognition stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about the source streaming video. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorInput {
    /// <p>The Kinesis video stream input stream for the source streaming video.</p>
    #[serde(rename = "KinesisVideoStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream: Option<KinesisVideoStream>,
}

/// <p>Information about the Amazon Kinesis Data Streams stream to which a Amazon Rekognition Video stream processor streams the results of a video analysis. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorOutput {
    /// <p>The Amazon Kinesis Data Streams stream to which the Amazon Rekognition stream processor streams the analysis results.</p>
    #[serde(rename = "KinesisDataStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream: Option<KinesisDataStream>,
}

/// <p>Input parameters used to recognize faces in a streaming video analyzed by a Amazon Rekognition stream processor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorSettings {
    /// <p>Face search settings to use on a streaming video. </p>
    #[serde(rename = "FaceSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_search: Option<FaceSearchSettings>,
}

/// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Sunglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing sunglasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Information about a word or line of text detected by .</p> <p>The <code>DetectedText</code> field contains the text that Amazon Rekognition detected in the image. </p> <p>Every word and line has an identifier (<code>Id</code>). Each word belongs to a line and has a parent identifier (<code>ParentId</code>) that identifies the line of text in which the word appears. The word <code>Id</code> is also an index for the word within a line of words. </p> <p>For more information, see Detecting Text in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TextDetection {
    /// <p>The confidence that Amazon Rekognition has in the accuracy of the detected text and the accuracy of the geometry points around the detected text.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The word or line of text recognized by Amazon Rekognition. </p>
    #[serde(rename = "DetectedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_text: Option<String>,
    /// <p>The location of the detected text on the image. Includes an axis aligned coarse bounding box surrounding the text and a finer grain polygon for more accurate spatial information.</p>
    #[serde(rename = "Geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    /// <p>The identifier for the detected text. The identifier is only unique for a single call to <code>DetectText</code>. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The Parent identifier for the detected text identified by the value of <code>ID</code>. If the type of detected text is <code>LINE</code>, the value of <code>ParentId</code> is <code>Null</code>. </p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// <p>The type of text that was detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Video {
    /// <p>The Amazon S3 bucket name and file name for the video.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VideoMetadata {
    /// <p>Type of compression used in the analyzed video. </p>
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Length of the video in milliseconds.</p>
    #[serde(rename = "DurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    /// <p>Format of the analyzed video. Possible values are MP4, MOV and AVI. </p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Vertical pixel dimension of the video.</p>
    #[serde(rename = "FrameHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_height: Option<i64>,
    /// <p>Number of frames per second in the video.</p>
    #[serde(rename = "FrameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    /// <p>Horizontal pixel dimension of the video.</p>
    #[serde(rename = "FrameWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<i64>,
}

/// Errors returned by CompareFaces
#[derive(Debug, PartialEq)]
pub enum CompareFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CompareFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> CompareFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CompareFacesError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return CompareFacesError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return CompareFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return CompareFacesError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return CompareFacesError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return CompareFacesError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return CompareFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return CompareFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return CompareFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CompareFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CompareFacesError {
    fn from(err: serde_json::error::Error) -> CompareFacesError {
        CompareFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CompareFacesError {
    fn from(err: CredentialsError) -> CompareFacesError {
        CompareFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompareFacesError {
    fn from(err: HttpDispatchError) -> CompareFacesError {
        CompareFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompareFacesError {
    fn from(err: io::Error) -> CompareFacesError {
        CompareFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompareFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompareFacesError {
    fn description(&self) -> &str {
        match *self {
            CompareFacesError::AccessDenied(ref cause) => cause,
            CompareFacesError::ImageTooLarge(ref cause) => cause,
            CompareFacesError::InternalServerError(ref cause) => cause,
            CompareFacesError::InvalidImageFormat(ref cause) => cause,
            CompareFacesError::InvalidParameter(ref cause) => cause,
            CompareFacesError::InvalidS3Object(ref cause) => cause,
            CompareFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            CompareFacesError::Throttling(ref cause) => cause,
            CompareFacesError::Validation(ref cause) => cause,
            CompareFacesError::Credentials(ref err) => err.description(),
            CompareFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CompareFacesError::ParseError(ref cause) => cause,
            CompareFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateCollection
#[derive(Debug, PartialEq)]
pub enum CreateCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>A collection with the specified ID already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateCollectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateCollectionError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return CreateCollectionError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return CreateCollectionError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return CreateCollectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return CreateCollectionError::ResourceAlreadyExists(String::from(error_message))
                }
                "ThrottlingException" => {
                    return CreateCollectionError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateCollectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateCollectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateCollectionError {
    fn from(err: serde_json::error::Error) -> CreateCollectionError {
        CreateCollectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCollectionError {
    fn from(err: CredentialsError) -> CreateCollectionError {
        CreateCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCollectionError {
    fn from(err: HttpDispatchError) -> CreateCollectionError {
        CreateCollectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCollectionError {
    fn from(err: io::Error) -> CreateCollectionError {
        CreateCollectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCollectionError {
    fn description(&self) -> &str {
        match *self {
            CreateCollectionError::AccessDenied(ref cause) => cause,
            CreateCollectionError::InternalServerError(ref cause) => cause,
            CreateCollectionError::InvalidParameter(ref cause) => cause,
            CreateCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            CreateCollectionError::ResourceAlreadyExists(ref cause) => cause,
            CreateCollectionError::Throttling(ref cause) => cause,
            CreateCollectionError::Validation(ref cause) => cause,
            CreateCollectionError::Credentials(ref err) => err.description(),
            CreateCollectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCollectionError::ParseError(ref cause) => cause,
            CreateCollectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateStreamProcessor
#[derive(Debug, PartialEq)]
pub enum CreateStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateStreamProcessorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return CreateStreamProcessorError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return CreateStreamProcessorError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return CreateStreamProcessorError::InvalidParameter(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateStreamProcessorError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return CreateStreamProcessorError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceInUseException" => {
                    return CreateStreamProcessorError::ResourceInUse(String::from(error_message))
                }
                "ThrottlingException" => {
                    return CreateStreamProcessorError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateStreamProcessorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateStreamProcessorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateStreamProcessorError {
    fn from(err: serde_json::error::Error) -> CreateStreamProcessorError {
        CreateStreamProcessorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStreamProcessorError {
    fn from(err: CredentialsError) -> CreateStreamProcessorError {
        CreateStreamProcessorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamProcessorError {
    fn from(err: HttpDispatchError) -> CreateStreamProcessorError {
        CreateStreamProcessorError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamProcessorError {
    fn from(err: io::Error) -> CreateStreamProcessorError {
        CreateStreamProcessorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamProcessorError::AccessDenied(ref cause) => cause,
            CreateStreamProcessorError::InternalServerError(ref cause) => cause,
            CreateStreamProcessorError::InvalidParameter(ref cause) => cause,
            CreateStreamProcessorError::LimitExceeded(ref cause) => cause,
            CreateStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            CreateStreamProcessorError::ResourceInUse(ref cause) => cause,
            CreateStreamProcessorError::Throttling(ref cause) => cause,
            CreateStreamProcessorError::Validation(ref cause) => cause,
            CreateStreamProcessorError::Credentials(ref err) => err.description(),
            CreateStreamProcessorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStreamProcessorError::ParseError(ref cause) => cause,
            CreateStreamProcessorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCollection
#[derive(Debug, PartialEq)]
pub enum DeleteCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCollectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteCollectionError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteCollectionError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DeleteCollectionError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DeleteCollectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DeleteCollectionError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return DeleteCollectionError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteCollectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteCollectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCollectionError {
    fn from(err: serde_json::error::Error) -> DeleteCollectionError {
        DeleteCollectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCollectionError {
    fn from(err: CredentialsError) -> DeleteCollectionError {
        DeleteCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCollectionError {
    fn from(err: HttpDispatchError) -> DeleteCollectionError {
        DeleteCollectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCollectionError {
    fn from(err: io::Error) -> DeleteCollectionError {
        DeleteCollectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCollectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteCollectionError::AccessDenied(ref cause) => cause,
            DeleteCollectionError::InternalServerError(ref cause) => cause,
            DeleteCollectionError::InvalidParameter(ref cause) => cause,
            DeleteCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteCollectionError::ResourceNotFound(ref cause) => cause,
            DeleteCollectionError::Throttling(ref cause) => cause,
            DeleteCollectionError::Validation(ref cause) => cause,
            DeleteCollectionError::Credentials(ref err) => err.description(),
            DeleteCollectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCollectionError::ParseError(ref cause) => cause,
            DeleteCollectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteFaces
#[derive(Debug, PartialEq)]
pub enum DeleteFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteFacesError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DeleteFacesError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DeleteFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DeleteFacesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return DeleteFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteFacesError {
    fn from(err: serde_json::error::Error) -> DeleteFacesError {
        DeleteFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFacesError {
    fn from(err: CredentialsError) -> DeleteFacesError {
        DeleteFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFacesError {
    fn from(err: HttpDispatchError) -> DeleteFacesError {
        DeleteFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFacesError {
    fn from(err: io::Error) -> DeleteFacesError {
        DeleteFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFacesError {
    fn description(&self) -> &str {
        match *self {
            DeleteFacesError::AccessDenied(ref cause) => cause,
            DeleteFacesError::InternalServerError(ref cause) => cause,
            DeleteFacesError::InvalidParameter(ref cause) => cause,
            DeleteFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteFacesError::ResourceNotFound(ref cause) => cause,
            DeleteFacesError::Throttling(ref cause) => cause,
            DeleteFacesError::Validation(ref cause) => cause,
            DeleteFacesError::Credentials(ref err) => err.description(),
            DeleteFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFacesError::ParseError(ref cause) => cause,
            DeleteFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DeleteStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteStreamProcessorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DeleteStreamProcessorError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return DeleteStreamProcessorError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DeleteStreamProcessorError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DeleteStreamProcessorError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceInUseException" => {
                    return DeleteStreamProcessorError::ResourceInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteStreamProcessorError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return DeleteStreamProcessorError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteStreamProcessorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteStreamProcessorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteStreamProcessorError {
    fn from(err: serde_json::error::Error) -> DeleteStreamProcessorError {
        DeleteStreamProcessorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStreamProcessorError {
    fn from(err: CredentialsError) -> DeleteStreamProcessorError {
        DeleteStreamProcessorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStreamProcessorError {
    fn from(err: HttpDispatchError) -> DeleteStreamProcessorError {
        DeleteStreamProcessorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStreamProcessorError {
    fn from(err: io::Error) -> DeleteStreamProcessorError {
        DeleteStreamProcessorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            DeleteStreamProcessorError::AccessDenied(ref cause) => cause,
            DeleteStreamProcessorError::InternalServerError(ref cause) => cause,
            DeleteStreamProcessorError::InvalidParameter(ref cause) => cause,
            DeleteStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteStreamProcessorError::ResourceInUse(ref cause) => cause,
            DeleteStreamProcessorError::ResourceNotFound(ref cause) => cause,
            DeleteStreamProcessorError::Throttling(ref cause) => cause,
            DeleteStreamProcessorError::Validation(ref cause) => cause,
            DeleteStreamProcessorError::Credentials(ref err) => err.description(),
            DeleteStreamProcessorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteStreamProcessorError::ParseError(ref cause) => cause,
            DeleteStreamProcessorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DescribeStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeStreamProcessorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DescribeStreamProcessorError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return DescribeStreamProcessorError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DescribeStreamProcessorError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return DescribeStreamProcessorError::ProvisionedThroughputExceeded(
                        String::from(error_message),
                    )
                }
                "ResourceNotFoundException" => {
                    return DescribeStreamProcessorError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DescribeStreamProcessorError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeStreamProcessorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeStreamProcessorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeStreamProcessorError {
    fn from(err: serde_json::error::Error) -> DescribeStreamProcessorError {
        DescribeStreamProcessorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStreamProcessorError {
    fn from(err: CredentialsError) -> DescribeStreamProcessorError {
        DescribeStreamProcessorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStreamProcessorError {
    fn from(err: HttpDispatchError) -> DescribeStreamProcessorError {
        DescribeStreamProcessorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStreamProcessorError {
    fn from(err: io::Error) -> DescribeStreamProcessorError {
        DescribeStreamProcessorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            DescribeStreamProcessorError::AccessDenied(ref cause) => cause,
            DescribeStreamProcessorError::InternalServerError(ref cause) => cause,
            DescribeStreamProcessorError::InvalidParameter(ref cause) => cause,
            DescribeStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            DescribeStreamProcessorError::ResourceNotFound(ref cause) => cause,
            DescribeStreamProcessorError::Throttling(ref cause) => cause,
            DescribeStreamProcessorError::Validation(ref cause) => cause,
            DescribeStreamProcessorError::Credentials(ref err) => err.description(),
            DescribeStreamProcessorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStreamProcessorError::ParseError(ref cause) => cause,
            DescribeStreamProcessorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectFaces
#[derive(Debug, PartialEq)]
pub enum DetectFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetectFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetectFacesError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return DetectFacesError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return DetectFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return DetectFacesError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DetectFacesError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return DetectFacesError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DetectFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DetectFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectFacesError {
    fn from(err: serde_json::error::Error) -> DetectFacesError {
        DetectFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectFacesError {
    fn from(err: CredentialsError) -> DetectFacesError {
        DetectFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectFacesError {
    fn from(err: HttpDispatchError) -> DetectFacesError {
        DetectFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectFacesError {
    fn from(err: io::Error) -> DetectFacesError {
        DetectFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectFacesError {
    fn description(&self) -> &str {
        match *self {
            DetectFacesError::AccessDenied(ref cause) => cause,
            DetectFacesError::ImageTooLarge(ref cause) => cause,
            DetectFacesError::InternalServerError(ref cause) => cause,
            DetectFacesError::InvalidImageFormat(ref cause) => cause,
            DetectFacesError::InvalidParameter(ref cause) => cause,
            DetectFacesError::InvalidS3Object(ref cause) => cause,
            DetectFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectFacesError::Throttling(ref cause) => cause,
            DetectFacesError::Validation(ref cause) => cause,
            DetectFacesError::Credentials(ref err) => err.description(),
            DetectFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectFacesError::ParseError(ref cause) => cause,
            DetectFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectLabels
#[derive(Debug, PartialEq)]
pub enum DetectLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetectLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectLabelsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetectLabelsError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return DetectLabelsError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return DetectLabelsError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return DetectLabelsError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DetectLabelsError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return DetectLabelsError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DetectLabelsError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DetectLabelsError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectLabelsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectLabelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectLabelsError {
    fn from(err: serde_json::error::Error) -> DetectLabelsError {
        DetectLabelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectLabelsError {
    fn from(err: CredentialsError) -> DetectLabelsError {
        DetectLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectLabelsError {
    fn from(err: HttpDispatchError) -> DetectLabelsError {
        DetectLabelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectLabelsError {
    fn from(err: io::Error) -> DetectLabelsError {
        DetectLabelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectLabelsError::AccessDenied(ref cause) => cause,
            DetectLabelsError::ImageTooLarge(ref cause) => cause,
            DetectLabelsError::InternalServerError(ref cause) => cause,
            DetectLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectLabelsError::InvalidParameter(ref cause) => cause,
            DetectLabelsError::InvalidS3Object(ref cause) => cause,
            DetectLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectLabelsError::Throttling(ref cause) => cause,
            DetectLabelsError::Validation(ref cause) => cause,
            DetectLabelsError::Credentials(ref err) => err.description(),
            DetectLabelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectLabelsError::ParseError(ref cause) => cause,
            DetectLabelsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectModerationLabels
#[derive(Debug, PartialEq)]
pub enum DetectModerationLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetectModerationLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectModerationLabelsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetectModerationLabelsError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return DetectModerationLabelsError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return DetectModerationLabelsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidImageFormatException" => {
                    return DetectModerationLabelsError::InvalidImageFormat(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DetectModerationLabelsError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return DetectModerationLabelsError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DetectModerationLabelsError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DetectModerationLabelsError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectModerationLabelsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectModerationLabelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectModerationLabelsError {
    fn from(err: serde_json::error::Error) -> DetectModerationLabelsError {
        DetectModerationLabelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectModerationLabelsError {
    fn from(err: CredentialsError) -> DetectModerationLabelsError {
        DetectModerationLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectModerationLabelsError {
    fn from(err: HttpDispatchError) -> DetectModerationLabelsError {
        DetectModerationLabelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectModerationLabelsError {
    fn from(err: io::Error) -> DetectModerationLabelsError {
        DetectModerationLabelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectModerationLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectModerationLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectModerationLabelsError::AccessDenied(ref cause) => cause,
            DetectModerationLabelsError::ImageTooLarge(ref cause) => cause,
            DetectModerationLabelsError::InternalServerError(ref cause) => cause,
            DetectModerationLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectModerationLabelsError::InvalidParameter(ref cause) => cause,
            DetectModerationLabelsError::InvalidS3Object(ref cause) => cause,
            DetectModerationLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectModerationLabelsError::Throttling(ref cause) => cause,
            DetectModerationLabelsError::Validation(ref cause) => cause,
            DetectModerationLabelsError::Credentials(ref err) => err.description(),
            DetectModerationLabelsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetectModerationLabelsError::ParseError(ref cause) => cause,
            DetectModerationLabelsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectText
#[derive(Debug, PartialEq)]
pub enum DetectTextError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DetectTextError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectTextError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DetectTextError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return DetectTextError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return DetectTextError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return DetectTextError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DetectTextError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return DetectTextError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return DetectTextError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return DetectTextError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectTextError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectTextError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectTextError {
    fn from(err: serde_json::error::Error) -> DetectTextError {
        DetectTextError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectTextError {
    fn from(err: CredentialsError) -> DetectTextError {
        DetectTextError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectTextError {
    fn from(err: HttpDispatchError) -> DetectTextError {
        DetectTextError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectTextError {
    fn from(err: io::Error) -> DetectTextError {
        DetectTextError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectTextError {
    fn description(&self) -> &str {
        match *self {
            DetectTextError::AccessDenied(ref cause) => cause,
            DetectTextError::ImageTooLarge(ref cause) => cause,
            DetectTextError::InternalServerError(ref cause) => cause,
            DetectTextError::InvalidImageFormat(ref cause) => cause,
            DetectTextError::InvalidParameter(ref cause) => cause,
            DetectTextError::InvalidS3Object(ref cause) => cause,
            DetectTextError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectTextError::Throttling(ref cause) => cause,
            DetectTextError::Validation(ref cause) => cause,
            DetectTextError::Credentials(ref err) => err.description(),
            DetectTextError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectTextError::ParseError(ref cause) => cause,
            DetectTextError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCelebrityInfo
#[derive(Debug, PartialEq)]
pub enum GetCelebrityInfoError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCelebrityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCelebrityInfoError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetCelebrityInfoError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetCelebrityInfoError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return GetCelebrityInfoError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetCelebrityInfoError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetCelebrityInfoError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetCelebrityInfoError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCelebrityInfoError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCelebrityInfoError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCelebrityInfoError {
    fn from(err: serde_json::error::Error) -> GetCelebrityInfoError {
        GetCelebrityInfoError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCelebrityInfoError {
    fn from(err: CredentialsError) -> GetCelebrityInfoError {
        GetCelebrityInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCelebrityInfoError {
    fn from(err: HttpDispatchError) -> GetCelebrityInfoError {
        GetCelebrityInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCelebrityInfoError {
    fn from(err: io::Error) -> GetCelebrityInfoError {
        GetCelebrityInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCelebrityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCelebrityInfoError {
    fn description(&self) -> &str {
        match *self {
            GetCelebrityInfoError::AccessDenied(ref cause) => cause,
            GetCelebrityInfoError::InternalServerError(ref cause) => cause,
            GetCelebrityInfoError::InvalidParameter(ref cause) => cause,
            GetCelebrityInfoError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetCelebrityInfoError::ResourceNotFound(ref cause) => cause,
            GetCelebrityInfoError::Throttling(ref cause) => cause,
            GetCelebrityInfoError::Validation(ref cause) => cause,
            GetCelebrityInfoError::Credentials(ref err) => err.description(),
            GetCelebrityInfoError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCelebrityInfoError::ParseError(ref cause) => cause,
            GetCelebrityInfoError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum GetCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetCelebrityRecognitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetCelebrityRecognitionError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetCelebrityRecognitionError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return GetCelebrityRecognitionError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetCelebrityRecognitionError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetCelebrityRecognitionError::ProvisionedThroughputExceeded(
                        String::from(error_message),
                    )
                }
                "ResourceNotFoundException" => {
                    return GetCelebrityRecognitionError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return GetCelebrityRecognitionError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCelebrityRecognitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCelebrityRecognitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCelebrityRecognitionError {
    fn from(err: serde_json::error::Error) -> GetCelebrityRecognitionError {
        GetCelebrityRecognitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCelebrityRecognitionError {
    fn from(err: CredentialsError) -> GetCelebrityRecognitionError {
        GetCelebrityRecognitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCelebrityRecognitionError {
    fn from(err: HttpDispatchError) -> GetCelebrityRecognitionError {
        GetCelebrityRecognitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCelebrityRecognitionError {
    fn from(err: io::Error) -> GetCelebrityRecognitionError {
        GetCelebrityRecognitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCelebrityRecognitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCelebrityRecognitionError {
    fn description(&self) -> &str {
        match *self {
            GetCelebrityRecognitionError::AccessDenied(ref cause) => cause,
            GetCelebrityRecognitionError::InternalServerError(ref cause) => cause,
            GetCelebrityRecognitionError::InvalidPaginationToken(ref cause) => cause,
            GetCelebrityRecognitionError::InvalidParameter(ref cause) => cause,
            GetCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetCelebrityRecognitionError::ResourceNotFound(ref cause) => cause,
            GetCelebrityRecognitionError::Throttling(ref cause) => cause,
            GetCelebrityRecognitionError::Validation(ref cause) => cause,
            GetCelebrityRecognitionError::Credentials(ref err) => err.description(),
            GetCelebrityRecognitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCelebrityRecognitionError::ParseError(ref cause) => cause,
            GetCelebrityRecognitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetContentModeration
#[derive(Debug, PartialEq)]
pub enum GetContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetContentModerationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetContentModerationError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetContentModerationError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return GetContentModerationError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetContentModerationError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetContentModerationError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetContentModerationError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetContentModerationError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetContentModerationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetContentModerationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetContentModerationError {
    fn from(err: serde_json::error::Error) -> GetContentModerationError {
        GetContentModerationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetContentModerationError {
    fn from(err: CredentialsError) -> GetContentModerationError {
        GetContentModerationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetContentModerationError {
    fn from(err: HttpDispatchError) -> GetContentModerationError {
        GetContentModerationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetContentModerationError {
    fn from(err: io::Error) -> GetContentModerationError {
        GetContentModerationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetContentModerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContentModerationError {
    fn description(&self) -> &str {
        match *self {
            GetContentModerationError::AccessDenied(ref cause) => cause,
            GetContentModerationError::InternalServerError(ref cause) => cause,
            GetContentModerationError::InvalidPaginationToken(ref cause) => cause,
            GetContentModerationError::InvalidParameter(ref cause) => cause,
            GetContentModerationError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetContentModerationError::ResourceNotFound(ref cause) => cause,
            GetContentModerationError::Throttling(ref cause) => cause,
            GetContentModerationError::Validation(ref cause) => cause,
            GetContentModerationError::Credentials(ref err) => err.description(),
            GetContentModerationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetContentModerationError::ParseError(ref cause) => cause,
            GetContentModerationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFaceDetection
#[derive(Debug, PartialEq)]
pub enum GetFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetFaceDetectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetFaceDetectionError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetFaceDetectionError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return GetFaceDetectionError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetFaceDetectionError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetFaceDetectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetFaceDetectionError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetFaceDetectionError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetFaceDetectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetFaceDetectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFaceDetectionError {
    fn from(err: serde_json::error::Error) -> GetFaceDetectionError {
        GetFaceDetectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFaceDetectionError {
    fn from(err: CredentialsError) -> GetFaceDetectionError {
        GetFaceDetectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFaceDetectionError {
    fn from(err: HttpDispatchError) -> GetFaceDetectionError {
        GetFaceDetectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFaceDetectionError {
    fn from(err: io::Error) -> GetFaceDetectionError {
        GetFaceDetectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFaceDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFaceDetectionError {
    fn description(&self) -> &str {
        match *self {
            GetFaceDetectionError::AccessDenied(ref cause) => cause,
            GetFaceDetectionError::InternalServerError(ref cause) => cause,
            GetFaceDetectionError::InvalidPaginationToken(ref cause) => cause,
            GetFaceDetectionError::InvalidParameter(ref cause) => cause,
            GetFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetFaceDetectionError::ResourceNotFound(ref cause) => cause,
            GetFaceDetectionError::Throttling(ref cause) => cause,
            GetFaceDetectionError::Validation(ref cause) => cause,
            GetFaceDetectionError::Credentials(ref err) => err.description(),
            GetFaceDetectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFaceDetectionError::ParseError(ref cause) => cause,
            GetFaceDetectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetFaceSearch
#[derive(Debug, PartialEq)]
pub enum GetFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> GetFaceSearchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetFaceSearchError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetFaceSearchError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return GetFaceSearchError::InvalidPaginationToken(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return GetFaceSearchError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetFaceSearchError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetFaceSearchError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetFaceSearchError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetFaceSearchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetFaceSearchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFaceSearchError {
    fn from(err: serde_json::error::Error) -> GetFaceSearchError {
        GetFaceSearchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFaceSearchError {
    fn from(err: CredentialsError) -> GetFaceSearchError {
        GetFaceSearchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFaceSearchError {
    fn from(err: HttpDispatchError) -> GetFaceSearchError {
        GetFaceSearchError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFaceSearchError {
    fn from(err: io::Error) -> GetFaceSearchError {
        GetFaceSearchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFaceSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFaceSearchError {
    fn description(&self) -> &str {
        match *self {
            GetFaceSearchError::AccessDenied(ref cause) => cause,
            GetFaceSearchError::InternalServerError(ref cause) => cause,
            GetFaceSearchError::InvalidPaginationToken(ref cause) => cause,
            GetFaceSearchError::InvalidParameter(ref cause) => cause,
            GetFaceSearchError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetFaceSearchError::ResourceNotFound(ref cause) => cause,
            GetFaceSearchError::Throttling(ref cause) => cause,
            GetFaceSearchError::Validation(ref cause) => cause,
            GetFaceSearchError::Credentials(ref err) => err.description(),
            GetFaceSearchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFaceSearchError::ParseError(ref cause) => cause,
            GetFaceSearchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetLabelDetection
#[derive(Debug, PartialEq)]
pub enum GetLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetLabelDetectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetLabelDetectionError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetLabelDetectionError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return GetLabelDetectionError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetLabelDetectionError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetLabelDetectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetLabelDetectionError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetLabelDetectionError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetLabelDetectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetLabelDetectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetLabelDetectionError {
    fn from(err: serde_json::error::Error) -> GetLabelDetectionError {
        GetLabelDetectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLabelDetectionError {
    fn from(err: CredentialsError) -> GetLabelDetectionError {
        GetLabelDetectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLabelDetectionError {
    fn from(err: HttpDispatchError) -> GetLabelDetectionError {
        GetLabelDetectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLabelDetectionError {
    fn from(err: io::Error) -> GetLabelDetectionError {
        GetLabelDetectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLabelDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLabelDetectionError {
    fn description(&self) -> &str {
        match *self {
            GetLabelDetectionError::AccessDenied(ref cause) => cause,
            GetLabelDetectionError::InternalServerError(ref cause) => cause,
            GetLabelDetectionError::InvalidPaginationToken(ref cause) => cause,
            GetLabelDetectionError::InvalidParameter(ref cause) => cause,
            GetLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetLabelDetectionError::ResourceNotFound(ref cause) => cause,
            GetLabelDetectionError::Throttling(ref cause) => cause,
            GetLabelDetectionError::Validation(ref cause) => cause,
            GetLabelDetectionError::Credentials(ref err) => err.description(),
            GetLabelDetectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLabelDetectionError::ParseError(ref cause) => cause,
            GetLabelDetectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPersonTracking
#[derive(Debug, PartialEq)]
pub enum GetPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> GetPersonTrackingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return GetPersonTrackingError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return GetPersonTrackingError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return GetPersonTrackingError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetPersonTrackingError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return GetPersonTrackingError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetPersonTrackingError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return GetPersonTrackingError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return GetPersonTrackingError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetPersonTrackingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPersonTrackingError {
    fn from(err: serde_json::error::Error) -> GetPersonTrackingError {
        GetPersonTrackingError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPersonTrackingError {
    fn from(err: CredentialsError) -> GetPersonTrackingError {
        GetPersonTrackingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPersonTrackingError {
    fn from(err: HttpDispatchError) -> GetPersonTrackingError {
        GetPersonTrackingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPersonTrackingError {
    fn from(err: io::Error) -> GetPersonTrackingError {
        GetPersonTrackingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPersonTrackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPersonTrackingError {
    fn description(&self) -> &str {
        match *self {
            GetPersonTrackingError::AccessDenied(ref cause) => cause,
            GetPersonTrackingError::InternalServerError(ref cause) => cause,
            GetPersonTrackingError::InvalidPaginationToken(ref cause) => cause,
            GetPersonTrackingError::InvalidParameter(ref cause) => cause,
            GetPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetPersonTrackingError::ResourceNotFound(ref cause) => cause,
            GetPersonTrackingError::Throttling(ref cause) => cause,
            GetPersonTrackingError::Validation(ref cause) => cause,
            GetPersonTrackingError::Credentials(ref err) => err.description(),
            GetPersonTrackingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPersonTrackingError::ParseError(ref cause) => cause,
            GetPersonTrackingError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by IndexFaces
#[derive(Debug, PartialEq)]
pub enum IndexFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl IndexFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> IndexFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return IndexFacesError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return IndexFacesError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return IndexFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return IndexFacesError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return IndexFacesError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return IndexFacesError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return IndexFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return IndexFacesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return IndexFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return IndexFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return IndexFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for IndexFacesError {
    fn from(err: serde_json::error::Error) -> IndexFacesError {
        IndexFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for IndexFacesError {
    fn from(err: CredentialsError) -> IndexFacesError {
        IndexFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IndexFacesError {
    fn from(err: HttpDispatchError) -> IndexFacesError {
        IndexFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for IndexFacesError {
    fn from(err: io::Error) -> IndexFacesError {
        IndexFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IndexFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IndexFacesError {
    fn description(&self) -> &str {
        match *self {
            IndexFacesError::AccessDenied(ref cause) => cause,
            IndexFacesError::ImageTooLarge(ref cause) => cause,
            IndexFacesError::InternalServerError(ref cause) => cause,
            IndexFacesError::InvalidImageFormat(ref cause) => cause,
            IndexFacesError::InvalidParameter(ref cause) => cause,
            IndexFacesError::InvalidS3Object(ref cause) => cause,
            IndexFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            IndexFacesError::ResourceNotFound(ref cause) => cause,
            IndexFacesError::Throttling(ref cause) => cause,
            IndexFacesError::Validation(ref cause) => cause,
            IndexFacesError::Credentials(ref err) => err.description(),
            IndexFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            IndexFacesError::ParseError(ref cause) => cause,
            IndexFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListCollections
#[derive(Debug, PartialEq)]
pub enum ListCollectionsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListCollectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListCollectionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListCollectionsError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return ListCollectionsError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return ListCollectionsError::InvalidPaginationToken(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListCollectionsError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return ListCollectionsError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return ListCollectionsError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return ListCollectionsError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListCollectionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListCollectionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListCollectionsError {
    fn from(err: serde_json::error::Error) -> ListCollectionsError {
        ListCollectionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCollectionsError {
    fn from(err: CredentialsError) -> ListCollectionsError {
        ListCollectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCollectionsError {
    fn from(err: HttpDispatchError) -> ListCollectionsError {
        ListCollectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCollectionsError {
    fn from(err: io::Error) -> ListCollectionsError {
        ListCollectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCollectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCollectionsError {
    fn description(&self) -> &str {
        match *self {
            ListCollectionsError::AccessDenied(ref cause) => cause,
            ListCollectionsError::InternalServerError(ref cause) => cause,
            ListCollectionsError::InvalidPaginationToken(ref cause) => cause,
            ListCollectionsError::InvalidParameter(ref cause) => cause,
            ListCollectionsError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListCollectionsError::ResourceNotFound(ref cause) => cause,
            ListCollectionsError::Throttling(ref cause) => cause,
            ListCollectionsError::Validation(ref cause) => cause,
            ListCollectionsError::Credentials(ref err) => err.description(),
            ListCollectionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCollectionsError::ParseError(ref cause) => cause,
            ListCollectionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFaces
#[derive(Debug, PartialEq)]
pub enum ListFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListFacesError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return ListFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidPaginationTokenException" => {
                    return ListFacesError::InvalidPaginationToken(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListFacesError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return ListFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return ListFacesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return ListFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFacesError {
    fn from(err: serde_json::error::Error) -> ListFacesError {
        ListFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacesError {
    fn from(err: CredentialsError) -> ListFacesError {
        ListFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacesError {
    fn from(err: HttpDispatchError) -> ListFacesError {
        ListFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFacesError {
    fn from(err: io::Error) -> ListFacesError {
        ListFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacesError {
    fn description(&self) -> &str {
        match *self {
            ListFacesError::AccessDenied(ref cause) => cause,
            ListFacesError::InternalServerError(ref cause) => cause,
            ListFacesError::InvalidPaginationToken(ref cause) => cause,
            ListFacesError::InvalidParameter(ref cause) => cause,
            ListFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListFacesError::ResourceNotFound(ref cause) => cause,
            ListFacesError::Throttling(ref cause) => cause,
            ListFacesError::Validation(ref cause) => cause,
            ListFacesError::Credentials(ref err) => err.description(),
            ListFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFacesError::ParseError(ref cause) => cause,
            ListFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListStreamProcessors
#[derive(Debug, PartialEq)]
pub enum ListStreamProcessorsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListStreamProcessorsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListStreamProcessorsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListStreamProcessorsError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return ListStreamProcessorsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return ListStreamProcessorsError::InvalidPaginationToken(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return ListStreamProcessorsError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return ListStreamProcessorsError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return ListStreamProcessorsError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return ListStreamProcessorsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListStreamProcessorsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListStreamProcessorsError {
    fn from(err: serde_json::error::Error) -> ListStreamProcessorsError {
        ListStreamProcessorsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListStreamProcessorsError {
    fn from(err: CredentialsError) -> ListStreamProcessorsError {
        ListStreamProcessorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStreamProcessorsError {
    fn from(err: HttpDispatchError) -> ListStreamProcessorsError {
        ListStreamProcessorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStreamProcessorsError {
    fn from(err: io::Error) -> ListStreamProcessorsError {
        ListStreamProcessorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStreamProcessorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStreamProcessorsError {
    fn description(&self) -> &str {
        match *self {
            ListStreamProcessorsError::AccessDenied(ref cause) => cause,
            ListStreamProcessorsError::InternalServerError(ref cause) => cause,
            ListStreamProcessorsError::InvalidPaginationToken(ref cause) => cause,
            ListStreamProcessorsError::InvalidParameter(ref cause) => cause,
            ListStreamProcessorsError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListStreamProcessorsError::Throttling(ref cause) => cause,
            ListStreamProcessorsError::Validation(ref cause) => cause,
            ListStreamProcessorsError::Credentials(ref err) => err.description(),
            ListStreamProcessorsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStreamProcessorsError::ParseError(ref cause) => cause,
            ListStreamProcessorsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RecognizeCelebrities
#[derive(Debug, PartialEq)]
pub enum RecognizeCelebritiesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RecognizeCelebritiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RecognizeCelebritiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return RecognizeCelebritiesError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return RecognizeCelebritiesError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return RecognizeCelebritiesError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RecognizeCelebritiesError::InvalidImageFormat(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return RecognizeCelebritiesError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return RecognizeCelebritiesError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return RecognizeCelebritiesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return RecognizeCelebritiesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return RecognizeCelebritiesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RecognizeCelebritiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RecognizeCelebritiesError {
    fn from(err: serde_json::error::Error) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RecognizeCelebritiesError {
    fn from(err: CredentialsError) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RecognizeCelebritiesError {
    fn from(err: HttpDispatchError) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RecognizeCelebritiesError {
    fn from(err: io::Error) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RecognizeCelebritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RecognizeCelebritiesError {
    fn description(&self) -> &str {
        match *self {
            RecognizeCelebritiesError::AccessDenied(ref cause) => cause,
            RecognizeCelebritiesError::ImageTooLarge(ref cause) => cause,
            RecognizeCelebritiesError::InternalServerError(ref cause) => cause,
            RecognizeCelebritiesError::InvalidImageFormat(ref cause) => cause,
            RecognizeCelebritiesError::InvalidParameter(ref cause) => cause,
            RecognizeCelebritiesError::InvalidS3Object(ref cause) => cause,
            RecognizeCelebritiesError::ProvisionedThroughputExceeded(ref cause) => cause,
            RecognizeCelebritiesError::Throttling(ref cause) => cause,
            RecognizeCelebritiesError::Validation(ref cause) => cause,
            RecognizeCelebritiesError::Credentials(ref err) => err.description(),
            RecognizeCelebritiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RecognizeCelebritiesError::ParseError(ref cause) => cause,
            RecognizeCelebritiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SearchFaces
#[derive(Debug, PartialEq)]
pub enum SearchFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchFacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return SearchFacesError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return SearchFacesError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return SearchFacesError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return SearchFacesError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return SearchFacesError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return SearchFacesError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return SearchFacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SearchFacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchFacesError {
    fn from(err: serde_json::error::Error) -> SearchFacesError {
        SearchFacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchFacesError {
    fn from(err: CredentialsError) -> SearchFacesError {
        SearchFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchFacesError {
    fn from(err: HttpDispatchError) -> SearchFacesError {
        SearchFacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchFacesError {
    fn from(err: io::Error) -> SearchFacesError {
        SearchFacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesError::AccessDenied(ref cause) => cause,
            SearchFacesError::InternalServerError(ref cause) => cause,
            SearchFacesError::InvalidParameter(ref cause) => cause,
            SearchFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesError::ResourceNotFound(ref cause) => cause,
            SearchFacesError::Throttling(ref cause) => cause,
            SearchFacesError::Validation(ref cause) => cause,
            SearchFacesError::Credentials(ref err) => err.description(),
            SearchFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchFacesError::ParseError(ref cause) => cause,
            SearchFacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SearchFacesByImage
#[derive(Debug, PartialEq)]
pub enum SearchFacesByImageError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SearchFacesByImageError {
    pub fn from_response(res: BufferedHttpResponse) -> SearchFacesByImageError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return SearchFacesByImageError::AccessDenied(String::from(error_message))
                }
                "ImageTooLargeException" => {
                    return SearchFacesByImageError::ImageTooLarge(String::from(error_message))
                }
                "InternalServerError" => {
                    return SearchFacesByImageError::InternalServerError(String::from(error_message))
                }
                "InvalidImageFormatException" => {
                    return SearchFacesByImageError::InvalidImageFormat(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return SearchFacesByImageError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return SearchFacesByImageError::InvalidS3Object(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return SearchFacesByImageError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return SearchFacesByImageError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return SearchFacesByImageError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return SearchFacesByImageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SearchFacesByImageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SearchFacesByImageError {
    fn from(err: serde_json::error::Error) -> SearchFacesByImageError {
        SearchFacesByImageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchFacesByImageError {
    fn from(err: CredentialsError) -> SearchFacesByImageError {
        SearchFacesByImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchFacesByImageError {
    fn from(err: HttpDispatchError) -> SearchFacesByImageError {
        SearchFacesByImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchFacesByImageError {
    fn from(err: io::Error) -> SearchFacesByImageError {
        SearchFacesByImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchFacesByImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesByImageError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesByImageError::AccessDenied(ref cause) => cause,
            SearchFacesByImageError::ImageTooLarge(ref cause) => cause,
            SearchFacesByImageError::InternalServerError(ref cause) => cause,
            SearchFacesByImageError::InvalidImageFormat(ref cause) => cause,
            SearchFacesByImageError::InvalidParameter(ref cause) => cause,
            SearchFacesByImageError::InvalidS3Object(ref cause) => cause,
            SearchFacesByImageError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesByImageError::ResourceNotFound(ref cause) => cause,
            SearchFacesByImageError::Throttling(ref cause) => cause,
            SearchFacesByImageError::Validation(ref cause) => cause,
            SearchFacesByImageError::Credentials(ref err) => err.description(),
            SearchFacesByImageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchFacesByImageError::ParseError(ref cause) => cause,
            SearchFacesByImageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum StartCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> StartCelebrityRecognitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartCelebrityRecognitionError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartCelebrityRecognitionError::IdempotentParameterMismatch(
                        String::from(error_message),
                    )
                }
                "InternalServerError" => {
                    return StartCelebrityRecognitionError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartCelebrityRecognitionError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return StartCelebrityRecognitionError::InvalidS3Object(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return StartCelebrityRecognitionError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartCelebrityRecognitionError::ProvisionedThroughputExceeded(
                        String::from(error_message),
                    )
                }
                "ThrottlingException" => {
                    return StartCelebrityRecognitionError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartCelebrityRecognitionError::VideoTooLarge(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartCelebrityRecognitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartCelebrityRecognitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartCelebrityRecognitionError {
    fn from(err: serde_json::error::Error) -> StartCelebrityRecognitionError {
        StartCelebrityRecognitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartCelebrityRecognitionError {
    fn from(err: CredentialsError) -> StartCelebrityRecognitionError {
        StartCelebrityRecognitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartCelebrityRecognitionError {
    fn from(err: HttpDispatchError) -> StartCelebrityRecognitionError {
        StartCelebrityRecognitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartCelebrityRecognitionError {
    fn from(err: io::Error) -> StartCelebrityRecognitionError {
        StartCelebrityRecognitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartCelebrityRecognitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCelebrityRecognitionError {
    fn description(&self) -> &str {
        match *self {
            StartCelebrityRecognitionError::AccessDenied(ref cause) => cause,
            StartCelebrityRecognitionError::IdempotentParameterMismatch(ref cause) => cause,
            StartCelebrityRecognitionError::InternalServerError(ref cause) => cause,
            StartCelebrityRecognitionError::InvalidParameter(ref cause) => cause,
            StartCelebrityRecognitionError::InvalidS3Object(ref cause) => cause,
            StartCelebrityRecognitionError::LimitExceeded(ref cause) => cause,
            StartCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartCelebrityRecognitionError::Throttling(ref cause) => cause,
            StartCelebrityRecognitionError::VideoTooLarge(ref cause) => cause,
            StartCelebrityRecognitionError::Validation(ref cause) => cause,
            StartCelebrityRecognitionError::Credentials(ref err) => err.description(),
            StartCelebrityRecognitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartCelebrityRecognitionError::ParseError(ref cause) => cause,
            StartCelebrityRecognitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartContentModeration
#[derive(Debug, PartialEq)]
pub enum StartContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> StartContentModerationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartContentModerationError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartContentModerationError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return StartContentModerationError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartContentModerationError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return StartContentModerationError::InvalidS3Object(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartContentModerationError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartContentModerationError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return StartContentModerationError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartContentModerationError::VideoTooLarge(String::from(error_message))
                }
                "ValidationException" => {
                    return StartContentModerationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartContentModerationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartContentModerationError {
    fn from(err: serde_json::error::Error) -> StartContentModerationError {
        StartContentModerationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartContentModerationError {
    fn from(err: CredentialsError) -> StartContentModerationError {
        StartContentModerationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartContentModerationError {
    fn from(err: HttpDispatchError) -> StartContentModerationError {
        StartContentModerationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartContentModerationError {
    fn from(err: io::Error) -> StartContentModerationError {
        StartContentModerationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartContentModerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartContentModerationError {
    fn description(&self) -> &str {
        match *self {
            StartContentModerationError::AccessDenied(ref cause) => cause,
            StartContentModerationError::IdempotentParameterMismatch(ref cause) => cause,
            StartContentModerationError::InternalServerError(ref cause) => cause,
            StartContentModerationError::InvalidParameter(ref cause) => cause,
            StartContentModerationError::InvalidS3Object(ref cause) => cause,
            StartContentModerationError::LimitExceeded(ref cause) => cause,
            StartContentModerationError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartContentModerationError::Throttling(ref cause) => cause,
            StartContentModerationError::VideoTooLarge(ref cause) => cause,
            StartContentModerationError::Validation(ref cause) => cause,
            StartContentModerationError::Credentials(ref err) => err.description(),
            StartContentModerationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartContentModerationError::ParseError(ref cause) => cause,
            StartContentModerationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartFaceDetection
#[derive(Debug, PartialEq)]
pub enum StartFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> StartFaceDetectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartFaceDetectionError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartFaceDetectionError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return StartFaceDetectionError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return StartFaceDetectionError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return StartFaceDetectionError::InvalidS3Object(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartFaceDetectionError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartFaceDetectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return StartFaceDetectionError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartFaceDetectionError::VideoTooLarge(String::from(error_message))
                }
                "ValidationException" => {
                    return StartFaceDetectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartFaceDetectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartFaceDetectionError {
    fn from(err: serde_json::error::Error) -> StartFaceDetectionError {
        StartFaceDetectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartFaceDetectionError {
    fn from(err: CredentialsError) -> StartFaceDetectionError {
        StartFaceDetectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartFaceDetectionError {
    fn from(err: HttpDispatchError) -> StartFaceDetectionError {
        StartFaceDetectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartFaceDetectionError {
    fn from(err: io::Error) -> StartFaceDetectionError {
        StartFaceDetectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartFaceDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFaceDetectionError {
    fn description(&self) -> &str {
        match *self {
            StartFaceDetectionError::AccessDenied(ref cause) => cause,
            StartFaceDetectionError::IdempotentParameterMismatch(ref cause) => cause,
            StartFaceDetectionError::InternalServerError(ref cause) => cause,
            StartFaceDetectionError::InvalidParameter(ref cause) => cause,
            StartFaceDetectionError::InvalidS3Object(ref cause) => cause,
            StartFaceDetectionError::LimitExceeded(ref cause) => cause,
            StartFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartFaceDetectionError::Throttling(ref cause) => cause,
            StartFaceDetectionError::VideoTooLarge(ref cause) => cause,
            StartFaceDetectionError::Validation(ref cause) => cause,
            StartFaceDetectionError::Credentials(ref err) => err.description(),
            StartFaceDetectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartFaceDetectionError::ParseError(ref cause) => cause,
            StartFaceDetectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartFaceSearch
#[derive(Debug, PartialEq)]
pub enum StartFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> StartFaceSearchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartFaceSearchError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartFaceSearchError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return StartFaceSearchError::InternalServerError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return StartFaceSearchError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return StartFaceSearchError::InvalidS3Object(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartFaceSearchError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartFaceSearchError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return StartFaceSearchError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return StartFaceSearchError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartFaceSearchError::VideoTooLarge(String::from(error_message))
                }
                "ValidationException" => {
                    return StartFaceSearchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartFaceSearchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartFaceSearchError {
    fn from(err: serde_json::error::Error) -> StartFaceSearchError {
        StartFaceSearchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartFaceSearchError {
    fn from(err: CredentialsError) -> StartFaceSearchError {
        StartFaceSearchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartFaceSearchError {
    fn from(err: HttpDispatchError) -> StartFaceSearchError {
        StartFaceSearchError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartFaceSearchError {
    fn from(err: io::Error) -> StartFaceSearchError {
        StartFaceSearchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartFaceSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFaceSearchError {
    fn description(&self) -> &str {
        match *self {
            StartFaceSearchError::AccessDenied(ref cause) => cause,
            StartFaceSearchError::IdempotentParameterMismatch(ref cause) => cause,
            StartFaceSearchError::InternalServerError(ref cause) => cause,
            StartFaceSearchError::InvalidParameter(ref cause) => cause,
            StartFaceSearchError::InvalidS3Object(ref cause) => cause,
            StartFaceSearchError::LimitExceeded(ref cause) => cause,
            StartFaceSearchError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartFaceSearchError::ResourceNotFound(ref cause) => cause,
            StartFaceSearchError::Throttling(ref cause) => cause,
            StartFaceSearchError::VideoTooLarge(ref cause) => cause,
            StartFaceSearchError::Validation(ref cause) => cause,
            StartFaceSearchError::Credentials(ref err) => err.description(),
            StartFaceSearchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartFaceSearchError::ParseError(ref cause) => cause,
            StartFaceSearchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartLabelDetection
#[derive(Debug, PartialEq)]
pub enum StartLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> StartLabelDetectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartLabelDetectionError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartLabelDetectionError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return StartLabelDetectionError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartLabelDetectionError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return StartLabelDetectionError::InvalidS3Object(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartLabelDetectionError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartLabelDetectionError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return StartLabelDetectionError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartLabelDetectionError::VideoTooLarge(String::from(error_message))
                }
                "ValidationException" => {
                    return StartLabelDetectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartLabelDetectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartLabelDetectionError {
    fn from(err: serde_json::error::Error) -> StartLabelDetectionError {
        StartLabelDetectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartLabelDetectionError {
    fn from(err: CredentialsError) -> StartLabelDetectionError {
        StartLabelDetectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartLabelDetectionError {
    fn from(err: HttpDispatchError) -> StartLabelDetectionError {
        StartLabelDetectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartLabelDetectionError {
    fn from(err: io::Error) -> StartLabelDetectionError {
        StartLabelDetectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartLabelDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartLabelDetectionError {
    fn description(&self) -> &str {
        match *self {
            StartLabelDetectionError::AccessDenied(ref cause) => cause,
            StartLabelDetectionError::IdempotentParameterMismatch(ref cause) => cause,
            StartLabelDetectionError::InternalServerError(ref cause) => cause,
            StartLabelDetectionError::InvalidParameter(ref cause) => cause,
            StartLabelDetectionError::InvalidS3Object(ref cause) => cause,
            StartLabelDetectionError::LimitExceeded(ref cause) => cause,
            StartLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartLabelDetectionError::Throttling(ref cause) => cause,
            StartLabelDetectionError::VideoTooLarge(ref cause) => cause,
            StartLabelDetectionError::Validation(ref cause) => cause,
            StartLabelDetectionError::Credentials(ref err) => err.description(),
            StartLabelDetectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartLabelDetectionError::ParseError(ref cause) => cause,
            StartLabelDetectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartPersonTracking
#[derive(Debug, PartialEq)]
pub enum StartPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> StartPersonTrackingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartPersonTrackingError::AccessDenied(String::from(error_message))
                }
                "IdempotentParameterMismatchException" => {
                    return StartPersonTrackingError::IdempotentParameterMismatch(String::from(
                        error_message,
                    ))
                }
                "InternalServerError" => {
                    return StartPersonTrackingError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartPersonTrackingError::InvalidParameter(String::from(error_message))
                }
                "InvalidS3ObjectException" => {
                    return StartPersonTrackingError::InvalidS3Object(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartPersonTrackingError::LimitExceeded(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartPersonTrackingError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ThrottlingException" => {
                    return StartPersonTrackingError::Throttling(String::from(error_message))
                }
                "VideoTooLargeException" => {
                    return StartPersonTrackingError::VideoTooLarge(String::from(error_message))
                }
                "ValidationException" => {
                    return StartPersonTrackingError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartPersonTrackingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartPersonTrackingError {
    fn from(err: serde_json::error::Error) -> StartPersonTrackingError {
        StartPersonTrackingError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartPersonTrackingError {
    fn from(err: CredentialsError) -> StartPersonTrackingError {
        StartPersonTrackingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartPersonTrackingError {
    fn from(err: HttpDispatchError) -> StartPersonTrackingError {
        StartPersonTrackingError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartPersonTrackingError {
    fn from(err: io::Error) -> StartPersonTrackingError {
        StartPersonTrackingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartPersonTrackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartPersonTrackingError {
    fn description(&self) -> &str {
        match *self {
            StartPersonTrackingError::AccessDenied(ref cause) => cause,
            StartPersonTrackingError::IdempotentParameterMismatch(ref cause) => cause,
            StartPersonTrackingError::InternalServerError(ref cause) => cause,
            StartPersonTrackingError::InvalidParameter(ref cause) => cause,
            StartPersonTrackingError::InvalidS3Object(ref cause) => cause,
            StartPersonTrackingError::LimitExceeded(ref cause) => cause,
            StartPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartPersonTrackingError::Throttling(ref cause) => cause,
            StartPersonTrackingError::VideoTooLarge(ref cause) => cause,
            StartPersonTrackingError::Validation(ref cause) => cause,
            StartPersonTrackingError::Credentials(ref err) => err.description(),
            StartPersonTrackingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartPersonTrackingError::ParseError(ref cause) => cause,
            StartPersonTrackingError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StartStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> StartStreamProcessorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StartStreamProcessorError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return StartStreamProcessorError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StartStreamProcessorError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StartStreamProcessorError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceInUseException" => {
                    return StartStreamProcessorError::ResourceInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return StartStreamProcessorError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return StartStreamProcessorError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return StartStreamProcessorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartStreamProcessorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartStreamProcessorError {
    fn from(err: serde_json::error::Error) -> StartStreamProcessorError {
        StartStreamProcessorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartStreamProcessorError {
    fn from(err: CredentialsError) -> StartStreamProcessorError {
        StartStreamProcessorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartStreamProcessorError {
    fn from(err: HttpDispatchError) -> StartStreamProcessorError {
        StartStreamProcessorError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartStreamProcessorError {
    fn from(err: io::Error) -> StartStreamProcessorError {
        StartStreamProcessorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            StartStreamProcessorError::AccessDenied(ref cause) => cause,
            StartStreamProcessorError::InternalServerError(ref cause) => cause,
            StartStreamProcessorError::InvalidParameter(ref cause) => cause,
            StartStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartStreamProcessorError::ResourceInUse(ref cause) => cause,
            StartStreamProcessorError::ResourceNotFound(ref cause) => cause,
            StartStreamProcessorError::Throttling(ref cause) => cause,
            StartStreamProcessorError::Validation(ref cause) => cause,
            StartStreamProcessorError::Credentials(ref err) => err.description(),
            StartStreamProcessorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartStreamProcessorError::ParseError(ref cause) => cause,
            StartStreamProcessorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StopStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StopStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> StopStreamProcessorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return StopStreamProcessorError::AccessDenied(String::from(error_message))
                }
                "InternalServerError" => {
                    return StopStreamProcessorError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return StopStreamProcessorError::InvalidParameter(String::from(error_message))
                }
                "ProvisionedThroughputExceededException" => {
                    return StopStreamProcessorError::ProvisionedThroughputExceeded(String::from(
                        error_message,
                    ))
                }
                "ResourceInUseException" => {
                    return StopStreamProcessorError::ResourceInUse(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return StopStreamProcessorError::ResourceNotFound(String::from(error_message))
                }
                "ThrottlingException" => {
                    return StopStreamProcessorError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return StopStreamProcessorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopStreamProcessorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopStreamProcessorError {
    fn from(err: serde_json::error::Error) -> StopStreamProcessorError {
        StopStreamProcessorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopStreamProcessorError {
    fn from(err: CredentialsError) -> StopStreamProcessorError {
        StopStreamProcessorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopStreamProcessorError {
    fn from(err: HttpDispatchError) -> StopStreamProcessorError {
        StopStreamProcessorError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopStreamProcessorError {
    fn from(err: io::Error) -> StopStreamProcessorError {
        StopStreamProcessorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            StopStreamProcessorError::AccessDenied(ref cause) => cause,
            StopStreamProcessorError::InternalServerError(ref cause) => cause,
            StopStreamProcessorError::InvalidParameter(ref cause) => cause,
            StopStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            StopStreamProcessorError::ResourceInUse(ref cause) => cause,
            StopStreamProcessorError::ResourceNotFound(ref cause) => cause,
            StopStreamProcessorError::Throttling(ref cause) => cause,
            StopStreamProcessorError::Validation(ref cause) => cause,
            StopStreamProcessorError::Credentials(ref err) => err.description(),
            StopStreamProcessorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopStreamProcessorError::ParseError(ref cause) => cause,
            StopStreamProcessorError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Rekognition API. Amazon Rekognition clients implement this trait.
pub trait Rekognition {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as a references to images in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> RusotoFuture<CompareFacesResponse, CompareFacesError>;

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> RusotoFuture<CreateCollectionResponse, CreateCollectionError>;

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use to stop processing. You can delete the stream processor by calling .</p>
    fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> RusotoFuture<CreateStreamProcessorResponse, CreateStreamProcessorError>;

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> RusotoFuture<DeleteCollectionResponse, DeleteCollectionError>;

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> RusotoFuture<DeleteFacesResponse, DeleteFacesError>;

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with . You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> RusotoFuture<DeleteStreamProcessorResponse, DeleteStreamProcessorError>;

    /// <p>Provides information about a stream processor created by . You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> RusotoFuture<DescribeStreamProcessorResponse, DescribeStreamProcessorError>;

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details including a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, etc. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm may not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> RusotoFuture<DetectFacesResponse, DetectFacesError>;

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response will include all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 50%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> RusotoFuture<DetectLabelsResponse, DetectLabelsError>;

    /// <p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> RusotoFuture<DetectModerationLabelsResponse, DetectModerationLabelsError>;

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 30 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> RusotoFuture<DetectTextResponse, DetectTextError>;

    /// <p>Gets the name and additional information about a celebrity based on his or her Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> RusotoFuture<GetCelebrityInfoResponse, GetCelebrityInfoError>;

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by .</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> RusotoFuture<GetCelebrityRecognitionResponse, GetCelebrityRecognitionError>;

    /// <p>Gets the content moderation analysis results for a Amazon Rekognition Video analysis started by .</p> <p>Content moderation analysis of a video is an asynchronous operation. You start analysis by calling . which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected content moderation labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> RusotoFuture<GetContentModerationResponse, GetContentModerationError>;

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by .</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> RusotoFuture<GetFaceDetectionResponse, GetFaceDetectionError>;

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by . The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> RusotoFuture<GetFaceSearchResponse, GetFaceSearchError>;

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by . </p> <p>The label detection operation is started by a call to which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> RusotoFuture<GetLabelDetectionResponse, GetLabelDetectionError>;

    /// <p>Gets the person tracking results of a Amazon Rekognition Video analysis started by .</p> <p>The person detection operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the person detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) they were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> RusotoFuture<GetPersonTrackingResponse, GetPersonTrackingError>;

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition does not save the actual faces detected. Instead, the underlying detection algorithm first detects the faces in the input image, and for each face extracts facial features into a feature vector, and stores it in the back-end database. Amazon Rekognition uses feature vectors when performing face match and search operations using the and operations.</p> <p>If you are using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. To determine which version of the model you are using, check the the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code>. </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image. </p> <p>In response, the operation returns an array of metadata for all detected faces. This includes, the bounding box of the detected face, confidence value (indicating the bounding box contains a face), a face ID assigned by the service for each face that is detected and stored, and an image ID assigned by the service for the input image. If you request all facial attributes (using the <code>detectionAttributes</code> parameter, Amazon Rekognition returns detailed facial attributes such as facial landmarks (for example, location of eye and mount) and other facial attributes such gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>The input image is passed either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> RusotoFuture<IndexFacesResponse, IndexFacesError>;

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> RusotoFuture<ListCollectionsResponse, ListCollectionsError>;

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> RusotoFuture<ListFacesResponse, ListFacesError>;

    /// <p>Gets a list of stream processors that you have created with . </p>
    fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> RusotoFuture<ListStreamProcessorsResponse, ListStreamProcessorsError>;

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces are not amongst the largest 100 faces in the image.</p> <p>For each celebrity recognized, the <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Rekognition does not retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the operation.</p> <p>You pass the imput image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> RusotoFuture<RecognizeCelebritiesResponse, RecognizeCelebritiesError>;

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> RusotoFuture<SearchFacesResponse, SearchFacesError>;

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the operation, and then use the face IDs returned in subsequent calls to the operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> RusotoFuture<SearchFacesByImageResponse, SearchFacesByImageError>;

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> RusotoFuture<StartCelebrityRecognitionResponse, StartCelebrityRecognitionError>;

    /// <p> Starts asynchronous detection of explicit or suggestive adult content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When content moderation analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> RusotoFuture<StartContentModerationResponse, StartContentModerationError>;

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> RusotoFuture<StartFaceDetectionResponse, StartFaceDetectionError>;

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>collections-search-person</a>.</p>
    fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> RusotoFuture<StartFaceSearchResponse, StartFaceSearchError>;

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> RusotoFuture<StartLabelDetectionResponse, StartLabelDetectionError>;

    /// <p>Starts the asynchronous tracking of persons in a stored video.</p> <p>Amazon Rekognition Video can track persons in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> RusotoFuture<StartPersonTrackingResponse, StartPersonTrackingError>;

    /// <p>Starts processing a stream processor. You create a stream processor by calling . To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> RusotoFuture<StartStreamProcessorResponse, StartStreamProcessorError>;

    /// <p>Stops a running stream processor that was created by .</p>
    fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> RusotoFuture<StopStreamProcessorResponse, StopStreamProcessorError>;
}
/// A client for the Amazon Rekognition API.
pub struct RekognitionClient {
    client: Client,
    region: region::Region,
}

impl RekognitionClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RekognitionClient {
        RekognitionClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RekognitionClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        RekognitionClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Rekognition for RekognitionClient {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as a references to images in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> RusotoFuture<CompareFacesResponse, CompareFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CompareFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CompareFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CompareFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> RusotoFuture<CreateCollectionResponse, CreateCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateCollection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateCollectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCollectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use to stop processing. You can delete the stream processor by calling .</p>
    fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> RusotoFuture<CreateStreamProcessorResponse, CreateStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateStreamProcessorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> RusotoFuture<DeleteCollectionResponse, DeleteCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteCollection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteCollectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCollectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> RusotoFuture<DeleteFacesResponse, DeleteFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with . You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> RusotoFuture<DeleteStreamProcessorResponse, DeleteStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteStreamProcessorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides information about a stream processor created by . You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> RusotoFuture<DescribeStreamProcessorResponse, DescribeStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeStreamProcessorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStreamProcessorError::from_response(response))
                }))
            }
        })
    }

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details including a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, etc. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm may not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> RusotoFuture<DetectFacesResponse, DetectFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response will include all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 50%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> RusotoFuture<DetectLabelsResponse, DetectLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectLabelsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectLabelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> RusotoFuture<DetectModerationLabelsResponse, DetectModerationLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectModerationLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectModerationLabelsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DetectModerationLabelsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 30 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> RusotoFuture<DetectTextResponse, DetectTextError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectText");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectTextResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectTextError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the name and additional information about a celebrity based on his or her Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> RusotoFuture<GetCelebrityInfoResponse, GetCelebrityInfoError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityInfo");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCelebrityInfoResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCelebrityInfoError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by .</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> RusotoFuture<GetCelebrityRecognitionResponse, GetCelebrityRecognitionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityRecognition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCelebrityRecognitionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCelebrityRecognitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the content moderation analysis results for a Amazon Rekognition Video analysis started by .</p> <p>Content moderation analysis of a video is an asynchronous operation. You start analysis by calling . which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected content moderation labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> RusotoFuture<GetContentModerationResponse, GetContentModerationError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetContentModeration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetContentModerationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetContentModerationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by .</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> RusotoFuture<GetFaceDetectionResponse, GetFaceDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetFaceDetectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFaceDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by . The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> RusotoFuture<GetFaceSearchResponse, GetFaceSearchError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetFaceSearchResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFaceSearchError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by . </p> <p>The label detection operation is started by a call to which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> RusotoFuture<GetLabelDetectionResponse, GetLabelDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetLabelDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLabelDetectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLabelDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the person tracking results of a Amazon Rekognition Video analysis started by .</p> <p>The person detection operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the person detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) they were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> RusotoFuture<GetPersonTrackingResponse, GetPersonTrackingError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetPersonTracking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPersonTrackingResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPersonTrackingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition does not save the actual faces detected. Instead, the underlying detection algorithm first detects the faces in the input image, and for each face extracts facial features into a feature vector, and stores it in the back-end database. Amazon Rekognition uses feature vectors when performing face match and search operations using the and operations.</p> <p>If you are using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. To determine which version of the model you are using, check the the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code>. </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image. </p> <p>In response, the operation returns an array of metadata for all detected faces. This includes, the bounding box of the detected face, confidence value (indicating the bounding box contains a face), a face ID assigned by the service for each face that is detected and stored, and an image ID assigned by the service for the input image. If you request all facial attributes (using the <code>detectionAttributes</code> parameter, Amazon Rekognition returns detailed facial attributes such as facial landmarks (for example, location of eye and mount) and other facial attributes such gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>The input image is passed either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> RusotoFuture<IndexFacesResponse, IndexFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.IndexFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IndexFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(IndexFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> RusotoFuture<ListCollectionsResponse, ListCollectionsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListCollections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCollectionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListCollectionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> RusotoFuture<ListFacesResponse, ListFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of stream processors that you have created with . </p>
    fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> RusotoFuture<ListStreamProcessorsResponse, ListStreamProcessorsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListStreamProcessors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListStreamProcessorsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListStreamProcessorsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces are not amongst the largest 100 faces in the image.</p> <p>For each celebrity recognized, the <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Rekognition does not retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the operation.</p> <p>You pass the imput image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> RusotoFuture<RecognizeCelebritiesResponse, RecognizeCelebritiesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.RecognizeCelebrities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RecognizeCelebritiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RecognizeCelebritiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> RusotoFuture<SearchFacesResponse, SearchFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchFacesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the operation, and then use the face IDs returned in subsequent calls to the operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> RusotoFuture<SearchFacesByImageResponse, SearchFacesByImageError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFacesByImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchFacesByImageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchFacesByImageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> RusotoFuture<StartCelebrityRecognitionResponse, StartCelebrityRecognitionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "RekognitionService.StartCelebrityRecognition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartCelebrityRecognitionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartCelebrityRecognitionError::from_response(response))
                }))
            }
        })
    }

    /// <p> Starts asynchronous detection of explicit or suggestive adult content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When content moderation analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> RusotoFuture<StartContentModerationResponse, StartContentModerationError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartContentModeration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartContentModerationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartContentModerationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> RusotoFuture<StartFaceDetectionResponse, StartFaceDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartFaceDetectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartFaceDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>collections-search-person</a>.</p>
    fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> RusotoFuture<StartFaceSearchResponse, StartFaceSearchError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartFaceSearchResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartFaceSearchError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> RusotoFuture<StartLabelDetectionResponse, StartLabelDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartLabelDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartLabelDetectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartLabelDetectionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts the asynchronous tracking of persons in a stored video.</p> <p>Amazon Rekognition Video can track persons in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> RusotoFuture<StartPersonTrackingResponse, StartPersonTrackingError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartPersonTracking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartPersonTrackingResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartPersonTrackingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts processing a stream processor. You create a stream processor by calling . To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> RusotoFuture<StartStreamProcessorResponse, StartStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartStreamProcessorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a running stream processor that was created by .</p>
    fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> RusotoFuture<StopStreamProcessorResponse, StopStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StopStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopStreamProcessorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
