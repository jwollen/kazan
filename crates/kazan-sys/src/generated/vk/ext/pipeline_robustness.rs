#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehavior;
pub type PipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehavior;
pub type PhysicalDevicePipelineRobustnessFeaturesEXT<'a> =
    PhysicalDevicePipelineRobustnessFeatures<'a>;
pub type PipelineRobustnessCreateInfoEXT<'a> = PipelineRobustnessCreateInfo<'a>;
pub type PhysicalDevicePipelineRobustnessPropertiesEXT<'a> =
    PhysicalDevicePipelineRobustnessProperties<'a>;
