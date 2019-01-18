#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//!
//! ```rust
//! ```

mod object_type;
pub use crate::object_type::ObjectType;

use failure::{ensure, Error, ResultExt};
use url::Url;

/// OpenGraph Object
pub trait Object {
  /// Convert the Object to a string.
  fn build(self) -> String;
}

/// The canonical URL for your page.
///
/// This should be the undecorated URL, without session variables, user
/// identifying parameters, or counters. Likes and Shares for this URL will
/// aggregate at this URL. For example, mobile domain URLs should point to the
/// desktop version of the URL as the canonical URL to aggregate Likes and
/// Shares across different versions of the page.
#[inline]
pub fn create_url<'s>(content: impl Into<&'s str>) -> String {
  create("og:url", content.into())
}

/// The title of your article without any branding such as your site name.
#[inline]
pub fn create_title<'s>(content: impl Into<&'s str>) -> String {
  create("og:title", content.into())
}

/// A brief description of the content, usually between 2 and 4 sentences.
#[inline]
pub fn create_description<'s>(content: impl Into<&'s str>) -> String {
  create("og:description", content.into())
}

/// The URL of the image that appears when someone shares the content.
#[inline]
pub fn create_image<'s>(content: impl Into<&'s str>) -> Result<String, Error> {
  let url = Url::parse(content.into()).context("String must be a valid URL")?;
  Ok(create("og:image", url.as_str()))
}

/// Equivalent to `og:image`.
#[inline]
pub fn create_image_url<'s>(
  content: impl Into<&'s str>,
) -> Result<String, Error> {
  let url = Url::parse(content.into()).context("String must be a valid URL")?;
  Ok(create("og:image:url", url.as_str()))
}

/// https:// URL for the image.
#[inline]
pub fn create_secure_image_url<'s>(
  content: impl Into<&'s str>,
) -> Result<String, Error> {
  let url = Url::parse(content.into()).context("String must be a valid URL")?;
  ensure!(
    url.scheme() == "https",
    "URL must start with https://. Use `.image_url()` instead."
  );
  Ok(create("og:image:secure_url", url.as_str()))
}

/// MIME type of the image. One of image/jpeg, image/gif or image/png
#[inline]
pub fn create_image_type<'s>(
  content: impl Into<&'s str>,
) -> Result<String, Error> {
  let url = Url::parse(content.into()).context("String must be a valid URL")?;
  Ok(create("og:image:url", url.as_str()))
}

/// Create an HTML Open Graph tag
#[inline]
fn create(name: &str, content: &str) -> String {
  format!(r#"<meta property="{}" content="{}" />"#, name, content)
}
