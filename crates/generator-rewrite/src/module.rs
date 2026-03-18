use itertools::Itertools as _;

use crate::xml;

#[derive(Debug)]
pub enum Module<'a> {
    Version(VersionInfo<'a>),
    Extension(&'a xml::Extension),
}

#[derive(Debug)]
pub struct ModuleName {
    pub vendor: Option<String>,
    pub name: String,
}

#[derive(Debug)]
pub struct VersionInfo<'a> {
    pub major: u32,
    pub minor: u32,
    pub features: Vec<&'a xml::Feature>,
}

impl Module<'_> {
    pub fn display_name(&self) -> String {
        match self {
            Module::Version(version) => format!("VK_VERSION_{}_{}", version.major, version.minor),
            Module::Extension(extension) => extension.name.to_string(),
        }
    }

    pub fn ext_number(&self) -> Option<u32> {
        match self {
            Module::Version(_) => None,
            Module::Extension(extension) => extension.number,
        }
    }

    pub fn name(&self) -> ModuleName {
        match self {
            Module::Version(version) => get_version_name(version),
            Module::Extension(extension) => get_extension_name(extension),
        }
    }

    pub fn requires(&self) -> Vec<&xml::Require> {
        match self {
            Module::Version(version) => version
                .features
                .iter()
                .flat_map(|feature| feature.requires.iter())
                .collect(),
            Module::Extension(extension) => extension.requires.iter().collect(),
        }
    }

    pub fn from_registry(registry: &xml::Registry) -> impl Iterator<Item = Module<'_>> {
        let versions = registry
            .features
            .iter()
            .chunk_by(|feature| (feature.version.major, feature.version.minor))
            .into_iter()
            .map(|(version, features)| VersionInfo {
                major: version.0,
                minor: version.1,
                features: features.into_iter().collect(),
            })
            .collect::<Vec<_>>();

        versions
            .into_iter()
            .map(Module::Version)
            .chain(registry.extensions.iter().map(Module::Extension))
    }
}

fn get_version_name(version: &VersionInfo<'_>) -> ModuleName {
    let name = format!("vk{}_{}", version.major, version.minor);
    ModuleName {
        vendor: None,
        name: name.to_ascii_lowercase(),
    }
}

fn get_extension_name(extension: &xml::Extension) -> ModuleName {
    let (vendor, name) = if extension.name.starts_with("VK_") {
        let (vendor, name) = extension
            .name
            .strip_prefix("VK_")
            .unwrap()
            .split_once("_")
            .unwrap();

        (vendor.to_lowercase(), name)
    } else {
        let name = extension.name.strip_prefix("vulkan_video_").unwrap();
        ("video".to_string(), name)
    };

    let name = if name.chars().next().unwrap().is_ascii_digit() {
        format!("_{name}")
    } else {
        name.to_string()
    };

    ModuleName {
        vendor: Some(vendor),
        name,
    }
}
