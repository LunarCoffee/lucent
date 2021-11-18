use async_std::fs;

use crate::{consts, server::template::Template};

// The templates used by `FileServer`. This should be initialized once, perhaps during initialization.
#[derive(Clone)]
pub struct Templates {
    // Error page for certain status codes (i.e. 404, 403, 500).
    pub error: Template,

    // Directory listing generated by `DirectoryLister`.
    pub dir_listing: Template,
}

impl Templates {
    // Attempts to read and parse each template.
    pub async fn new(template_root: &str) -> Option<Self> {
        let error_path = format!("{}/{}", template_root, consts::TEMPLATE_ERROR);
        let dir_listing_path = format!("{}/{}", template_root, consts::TEMPLATE_DIR_LISTING);

        let error_template = fs::read_to_string(error_path).await.ok()?;
        let dir_listing_template = fs::read_to_string(dir_listing_path).await.ok()?;

        let error = Template::new(error_template)?;
        let dir_listing = Template::new(dir_listing_template)?;
        Some(Templates { error, dir_listing })
    }

    // This always returns an unusable set of empty templates; this must only be used as a placeholder where it is
    // known that this value will never be used.
    pub fn new_empty() -> Self { Templates { error: Template::new_empty(), dir_listing: Template::new_empty() } }
}
