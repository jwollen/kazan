use std::{fs, path::Path};
use tracing::{debug, error_span};

use crate::xml;

/// Holds the analysis results for easy querying.
#[derive(Debug)]
pub struct Analysis {
    registry: xml::Registry,
}

impl Analysis {
    /// Analyse the provided copy of the
    /// [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) repo.
    pub fn new(vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        let vulkan_headers_path = vulkan_headers_path.as_ref();

        let paths = [
            vulkan_headers_path.join("registry/vk.xml"),
            vulkan_headers_path.join("registry/video.xml"),
        ];

        let mut registry = xml::Registry::default();
        for path in paths {
            let xml = error_span!("xml", path = %path.display()).in_scope(|| {
                debug!("reading xml");
                // We leak the input string here for convenience, to avoid explicit lifetimes.
                let xml_input = Box::leak(fs::read_to_string(path).unwrap().into_boxed_str());
                debug!("parsing xml");
                xml::Registry::parse(xml_input, "vulkan")
            });
            registry.merge(xml);
        }

        Self { registry }
    }

    /// Get "raw" Vulkan XML registry.
    pub fn registry(&self) -> &xml::Registry {
        &self.registry
    }
}
