#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
    handle_nondispatchable!(
        DataGraphPipelineSessionARM,
        DATA_GRAPH_PIPELINE_SESSION_ARM,
        doc = ""
    );
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDataGraphFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub data_graph: Bool32,
        pub data_graph_update_after_bind: Bool32,
        pub data_graph_specialization_constants: Bool32,
        pub data_graph_descriptor_buffer: Bool32,
        pub data_graph_shader_module: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDataGraphFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceDataGraphFeaturesARM<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDataGraphFeaturesARM<'a> {}
    impl Default for PhysicalDeviceDataGraphFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                data_graph: Default::default(),
                data_graph_update_after_bind: Default::default(),
                data_graph_specialization_constants: Default::default(),
                data_graph_descriptor_buffer: Default::default(),
                data_graph_shader_module: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDataGraphFeaturesARM<'a> {
        pub fn data_graph(mut self, data_graph: bool) -> Self {
            self.data_graph = data_graph.into();
            self
        }
        pub fn data_graph_update_after_bind(mut self, data_graph_update_after_bind: bool) -> Self {
            self.data_graph_update_after_bind = data_graph_update_after_bind.into();
            self
        }
        pub fn data_graph_specialization_constants(
            mut self,
            data_graph_specialization_constants: bool,
        ) -> Self {
            self.data_graph_specialization_constants = data_graph_specialization_constants.into();
            self
        }
        pub fn data_graph_descriptor_buffer(mut self, data_graph_descriptor_buffer: bool) -> Self {
            self.data_graph_descriptor_buffer = data_graph_descriptor_buffer.into();
            self
        }
        pub fn data_graph_shader_module(mut self, data_graph_shader_module: bool) -> Self {
            self.data_graph_shader_module = data_graph_shader_module.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dimension: u32,
        pub zero_count: u32,
        pub group_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a>
        for DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM;
    }
    unsafe impl<'a> Extends<DataGraphPipelineConstantARM<'a>>
        for DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a>
    {
    }
    impl Default for DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                dimension: Default::default(),
                zero_count: Default::default(),
                group_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {
        pub fn dimension(mut self, dimension: u32) -> Self {
            self.dimension = dimension;
            self
        }
        pub fn zero_count(mut self, zero_count: u32) -> Self {
            self.zero_count = zero_count;
            self
        }
        pub fn group_size(mut self, group_size: u32) -> Self {
            self.group_size = group_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineConstantARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub id: u32,
        pub p_constant_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineConstantARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DATA_GRAPH_PIPELINE_CONSTANT_ARM;
    }
    impl Default for DataGraphPipelineConstantARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                id: Default::default(),
                p_constant_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineConstantARM<'a> {
        pub fn id(mut self, id: u32) -> Self {
            self.id = id;
            self
        }
        pub fn constant_data(mut self, constant_data: *const c_void) -> Self {
            self.p_constant_data = constant_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineResourceInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_set: u32,
        pub binding: u32,
        pub array_element: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineResourceInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM;
    }
    impl Default for DataGraphPipelineResourceInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                descriptor_set: Default::default(),
                binding: Default::default(),
                array_element: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineResourceInfoARM<'a> {
        pub fn descriptor_set(mut self, descriptor_set: u32) -> Self {
            self.descriptor_set = descriptor_set;
            self
        }
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn array_element(mut self, array_element: u32) -> Self {
            self.array_element = array_element;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineCompilerControlCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_vendor_options: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineCompilerControlCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM;
    }
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for DataGraphPipelineCompilerControlCreateInfoARM<'a>
    {
    }
    impl Default for DataGraphPipelineCompilerControlCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_vendor_options: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineCompilerControlCreateInfoARM<'a> {
        pub fn vendor_options(mut self, vendor_options: &'a CStr) -> Self {
            self.p_vendor_options = vendor_options.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags2KHR,
        pub layout: PipelineLayout,
        pub resource_info_count: u32,
        pub p_resource_infos: *const DataGraphPipelineResourceInfoARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DATA_GRAPH_PIPELINE_CREATE_INFO_ARM;
    }
    impl Default for DataGraphPipelineCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                layout: Default::default(),
                resource_info_count: Default::default(),
                p_resource_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineCreateInfoARM<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags2KHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn resource_infos(
            mut self,
            resource_infos: &'a [DataGraphPipelineResourceInfoARM<'a>],
        ) -> Self {
            self.resource_info_count = resource_infos.len().try_into().unwrap();
            self.p_resource_infos = resource_infos.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineShaderModuleCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub module: ShaderModule,
        pub p_name: *const c_char,
        pub p_specialization_info: *const SpecializationInfo<'a>,
        pub constant_count: u32,
        pub p_constants: *const DataGraphPipelineConstantARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineShaderModuleCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM;
    }
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for DataGraphPipelineShaderModuleCreateInfoARM<'a>
    {
    }
    impl Default for DataGraphPipelineShaderModuleCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                module: Default::default(),
                p_name: core::ptr::null(),
                p_specialization_info: core::ptr::null(),
                constant_count: Default::default(),
                p_constants: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineShaderModuleCreateInfoARM<'a> {
        pub fn module(mut self, module: ShaderModule) -> Self {
            self.module = module;
            self
        }
        pub fn name(mut self, name: &'a CStr) -> Self {
            self.p_name = name.as_ptr();
            self
        }
        pub fn specialization_info(
            mut self,
            specialization_info: &'a SpecializationInfo<'a>,
        ) -> Self {
            self.p_specialization_info = specialization_info;
            self
        }
        pub fn constants(mut self, constants: &'a [DataGraphPipelineConstantARM<'a>]) -> Self {
            self.constant_count = constants.len().try_into().unwrap();
            self.p_constants = constants.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineSessionCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DataGraphPipelineSessionCreateFlagsARM,
        pub data_graph_pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSessionCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM;
    }
    impl Default for DataGraphPipelineSessionCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                data_graph_pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineSessionCreateInfoARM<'a> {
        pub fn flags(mut self, flags: DataGraphPipelineSessionCreateFlagsARM) -> Self {
            self.flags = flags;
            self
        }
        pub fn data_graph_pipeline(mut self, data_graph_pipeline: Pipeline) -> Self {
            self.data_graph_pipeline = data_graph_pipeline;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub session: DataGraphPipelineSessionARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM;
    }
    impl Default for DataGraphPipelineSessionBindPointRequirementsInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                session: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
        pub fn session(mut self, session: DataGraphPipelineSessionARM) -> Self {
            self.session = session;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineSessionBindPointRequirementARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub bind_point: DataGraphPipelineSessionBindPointARM,
        pub bind_point_type: DataGraphPipelineSessionBindPointTypeARM,
        pub num_objects: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSessionBindPointRequirementARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM;
    }
    impl Default for DataGraphPipelineSessionBindPointRequirementARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                bind_point: Default::default(),
                bind_point_type: Default::default(),
                num_objects: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineSessionBindPointRequirementARM<'a> {
        pub fn bind_point(mut self, bind_point: DataGraphPipelineSessionBindPointARM) -> Self {
            self.bind_point = bind_point;
            self
        }
        pub fn bind_point_type(
            mut self,
            bind_point_type: DataGraphPipelineSessionBindPointTypeARM,
        ) -> Self {
            self.bind_point_type = bind_point_type;
            self
        }
        pub fn num_objects(mut self, num_objects: u32) -> Self {
            self.num_objects = num_objects;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub session: DataGraphPipelineSessionARM,
        pub bind_point: DataGraphPipelineSessionBindPointARM,
        pub object_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM;
    }
    impl Default for DataGraphPipelineSessionMemoryRequirementsInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                session: Default::default(),
                bind_point: Default::default(),
                object_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
        pub fn session(mut self, session: DataGraphPipelineSessionARM) -> Self {
            self.session = session;
            self
        }
        pub fn bind_point(mut self, bind_point: DataGraphPipelineSessionBindPointARM) -> Self {
            self.bind_point = bind_point;
            self
        }
        pub fn object_index(mut self, object_index: u32) -> Self {
            self.object_index = object_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindDataGraphPipelineSessionMemoryInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub session: DataGraphPipelineSessionARM,
        pub bind_point: DataGraphPipelineSessionBindPointARM,
        pub object_index: u32,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindDataGraphPipelineSessionMemoryInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM;
    }
    impl Default for BindDataGraphPipelineSessionMemoryInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                session: Default::default(),
                bind_point: Default::default(),
                object_index: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindDataGraphPipelineSessionMemoryInfoARM<'a> {
        pub fn session(mut self, session: DataGraphPipelineSessionARM) -> Self {
            self.session = session;
            self
        }
        pub fn bind_point(mut self, bind_point: DataGraphPipelineSessionBindPointARM) -> Self {
            self.bind_point = bind_point;
            self
        }
        pub fn object_index(mut self, object_index: u32) -> Self {
            self.object_index = object_index;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data_graph_pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DATA_GRAPH_PIPELINE_INFO_ARM;
    }
    impl Default for DataGraphPipelineInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                data_graph_pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineInfoARM<'a> {
        pub fn data_graph_pipeline(mut self, data_graph_pipeline: Pipeline) -> Self {
            self.data_graph_pipeline = data_graph_pipeline;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelinePropertyQueryResultARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub property: DataGraphPipelinePropertyARM,
        pub is_text: Bool32,
        pub data_size: usize,
        pub p_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelinePropertyQueryResultARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM;
    }
    impl Default for DataGraphPipelinePropertyQueryResultARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                property: Default::default(),
                is_text: Default::default(),
                data_size: Default::default(),
                p_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelinePropertyQueryResultARM<'a> {
        pub fn property(mut self, property: DataGraphPipelinePropertyARM) -> Self {
            self.property = property;
            self
        }
        pub fn is_text(mut self, is_text: bool) -> Self {
            self.is_text = is_text.into();
            self
        }
        pub fn data(mut self, data: &'a mut [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_mut_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineIdentifierCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub identifier_size: u32,
        pub p_identifier: *const u8,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineIdentifierCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM;
    }
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for DataGraphPipelineIdentifierCreateInfoARM<'a>
    {
    }
    impl Default for DataGraphPipelineIdentifierCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                identifier_size: Default::default(),
                p_identifier: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineIdentifierCreateInfoARM<'a> {
        pub fn identifier(mut self, identifier: &'a [u8]) -> Self {
            self.identifier_size = identifier.len().try_into().unwrap();
            self.p_identifier = identifier.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphPipelineDispatchInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DataGraphPipelineDispatchFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineDispatchInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM;
    }
    impl Default for DataGraphPipelineDispatchInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphPipelineDispatchInfoARM<'a> {
        pub fn flags(mut self, flags: DataGraphPipelineDispatchFlagsARM) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PhysicalDeviceDataGraphProcessingEngineARM {
        pub ty: PhysicalDeviceDataGraphProcessingEngineTypeARM,
        pub is_foreign: Bool32,
    }
    impl PhysicalDeviceDataGraphProcessingEngineARM {
        pub fn ty(mut self, ty: PhysicalDeviceDataGraphProcessingEngineTypeARM) -> Self {
            self.ty = ty;
            self
        }
        pub fn is_foreign(mut self, is_foreign: bool) -> Self {
            self.is_foreign = is_foreign.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDataGraphOperationSupportARM {
        pub operation_type: PhysicalDeviceDataGraphOperationTypeARM,
        pub name: [c_char; MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
        pub version: u32,
    }
    impl Default for PhysicalDeviceDataGraphOperationSupportARM {
        fn default() -> Self {
            Self {
                operation_type: Default::default(),
                name: [Default::default(); _],
                version: Default::default(),
            }
        }
    }
    impl PhysicalDeviceDataGraphOperationSupportARM {
        pub fn operation_type(
            mut self,
            operation_type: PhysicalDeviceDataGraphOperationTypeARM,
        ) -> Self {
            self.operation_type = operation_type;
            self
        }
        pub fn version(mut self, version: u32) -> Self {
            self.version = version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyDataGraphPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub engine: PhysicalDeviceDataGraphProcessingEngineARM,
        pub operation: PhysicalDeviceDataGraphOperationSupportARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyDataGraphPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM;
    }
    impl Default for QueueFamilyDataGraphPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                engine: Default::default(),
                operation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyDataGraphPropertiesARM<'a> {
        pub fn engine(mut self, engine: PhysicalDeviceDataGraphProcessingEngineARM) -> Self {
            self.engine = engine;
            self
        }
        pub fn operation(mut self, operation: PhysicalDeviceDataGraphOperationSupportARM) -> Self {
            self.operation = operation;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub queue_family_index: u32,
        pub engine_type: PhysicalDeviceDataGraphProcessingEngineTypeARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM;
    }
    impl Default for PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                queue_family_index: Default::default(),
                engine_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }
        pub fn engine_type(
            mut self,
            engine_type: PhysicalDeviceDataGraphProcessingEngineTypeARM,
        ) -> Self {
            self.engine_type = engine_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub foreign_semaphore_handle_types: ExternalSemaphoreHandleTypeFlags,
        pub foreign_memory_handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM;
    }
    impl Default for QueueFamilyDataGraphProcessingEnginePropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                foreign_semaphore_handle_types: Default::default(),
                foreign_memory_handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
        pub fn foreign_semaphore_handle_types(
            mut self,
            foreign_semaphore_handle_types: ExternalSemaphoreHandleTypeFlags,
        ) -> Self {
            self.foreign_semaphore_handle_types = foreign_semaphore_handle_types;
            self
        }
        pub fn foreign_memory_handle_types(
            mut self,
            foreign_memory_handle_types: ExternalMemoryHandleTypeFlags,
        ) -> Self {
            self.foreign_memory_handle_types = foreign_memory_handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DataGraphProcessingEngineCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub processing_engine_count: u32,
        pub p_processing_engines: *mut PhysicalDeviceDataGraphProcessingEngineARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DataGraphProcessingEngineCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM;
    }
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for DataGraphProcessingEngineCreateInfoARM<'a>
    {
    }
    unsafe impl<'a> Extends<DescriptorPoolCreateInfo<'a>>
        for DataGraphProcessingEngineCreateInfoARM<'a>
    {
    }
    unsafe impl<'a> Extends<CommandPoolCreateInfo<'a>> for DataGraphProcessingEngineCreateInfoARM<'a> {}
    impl Default for DataGraphProcessingEngineCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                processing_engine_count: Default::default(),
                p_processing_engines: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DataGraphProcessingEngineCreateInfoARM<'a> {
        pub fn processing_engines(
            mut self,
            processing_engines: &'a mut [PhysicalDeviceDataGraphProcessingEngineARM],
        ) -> Self {
            self.processing_engine_count = processing_engines.len().try_into().unwrap();
            self.p_processing_engines = processing_engines.as_mut_ptr();
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineSessionBindPointARM(i32);
    impl DataGraphPipelineSessionBindPointARM {
        pub const TRANSIENT_ARM: Self = Self(0);
    }
    impl fmt::Debug for DataGraphPipelineSessionBindPointARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRANSIENT_ARM => Some("TRANSIENT_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineSessionBindPointTypeARM(i32);
    impl DataGraphPipelineSessionBindPointTypeARM {
        pub const MEMORY_ARM: Self = Self(0);
    }
    impl fmt::Debug for DataGraphPipelineSessionBindPointTypeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MEMORY_ARM => Some("MEMORY_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelinePropertyARM(i32);
    impl DataGraphPipelinePropertyARM {
        pub const CREATION_LOG_ARM: Self = Self(0);
        pub const IDENTIFIER_ARM: Self = Self(1);
    }
    impl fmt::Debug for DataGraphPipelinePropertyARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CREATION_LOG_ARM => Some("CREATION_LOG_ARM"),
                Self::IDENTIFIER_ARM => Some("IDENTIFIER_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(i32);
    impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
        pub const DEFAULT_ARM: Self = Self(0);
        // VK_QCOM_data_graph_model
        pub const NEURAL_QCOM: Self = Self(1000629000);
        pub const COMPUTE_QCOM: Self = Self(1000629001);
    }
    impl fmt::Debug for PhysicalDeviceDataGraphProcessingEngineTypeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_ARM => Some("DEFAULT_ARM"),
                Self::NEURAL_QCOM => Some("NEURAL_QCOM"),
                Self::COMPUTE_QCOM => Some("COMPUTE_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhysicalDeviceDataGraphOperationTypeARM(i32);
    impl PhysicalDeviceDataGraphOperationTypeARM {
        pub const SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
        // VK_QCOM_data_graph_model
        pub const NEURAL_MODEL_QCOM: Self = Self(1000629000);
        pub const BUILTIN_MODEL_QCOM: Self = Self(1000629001);
    }
    impl fmt::Debug for PhysicalDeviceDataGraphOperationTypeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SPIRV_EXTENDED_INSTRUCTION_SET_ARM => {
                    Some("SPIRV_EXTENDED_INSTRUCTION_SET_ARM")
                }
                Self::NEURAL_MODEL_QCOM => Some("NEURAL_MODEL_QCOM"),
                Self::BUILTIN_MODEL_QCOM => Some("BUILTIN_MODEL_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphPipelineSessionCreateFlagsARM(Flags64);
    vk_bitflags_wrapped!(DataGraphPipelineSessionCreateFlagsARM, Flags64);
    impl DataGraphPipelineSessionCreateFlagsARM {
        pub const PROTECTED_ARM: Self =
            Self(DataGraphPipelineSessionCreateFlagBitsARM::PROTECTED_ARM.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphPipelineSessionCreateFlagBitsARM(u64);
    impl DataGraphPipelineSessionCreateFlagBitsARM {
        pub const PROTECTED_ARM: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphPipelineDispatchFlagsARM(Flags64);
    vk_bitflags_wrapped!(DataGraphPipelineDispatchFlagsARM, Flags64);
    impl DataGraphPipelineDispatchFlagsARM {}
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphPipelineDispatchFlagBitsARM(u64);
    impl DataGraphPipelineDispatchFlagBitsARM {}
    pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const DataGraphPipelineCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
    pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DataGraphPipelineSessionCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut DataGraphPipelineSessionARM,
    ) -> vk::Result;
    pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
            p_bind_point_requirement_count: *mut u32,
            p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
        device: Device,
        p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
    )
        -> vk::Result;
    pub type PFN_vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
        device: Device,
        session: DataGraphPipelineSessionARM,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        p_info: *const DataGraphPipelineDispatchInfoARM<'_>,
    );
    pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM =
        unsafe extern "system" fn(
            device: Device,
            p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
            p_properties_count: *mut u32,
            p_properties: *mut DataGraphPipelinePropertyARM,
        ) -> vk::Result;
    pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
        properties_count: u32,
        p_properties: *mut DataGraphPipelinePropertyQueryResultARM<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            p_queue_family_data_graph_property_count: *mut u32,
            p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
    p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
);
}
pub struct InstanceFn {
    get_physical_device_queue_family_data_graph_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
    get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_queue_family_data_graph_properties_arm: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
                    transmute(
                        load(
                            c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
                        )
                        .ok_or(MissingEntryPointError)?,
                    ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        mut queue_family_data_graph_properties: impl ExtendUninit<QueueFamilyDataGraphPropertiesARM<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |queue_family_data_graph_property_count,
                        queue_family_data_graph_properties| {
                let result = (self.get_physical_device_queue_family_data_graph_properties_arm)(
                    physical_device,
                    queue_family_index,
                    queue_family_data_graph_property_count,
                    queue_family_data_graph_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let queue_family_data_graph_properties_buf =
                queue_family_data_graph_properties.reserve(capacity);
            len = queue_family_data_graph_properties_buf
                .len()
                .try_into()
                .unwrap();
            let result = call(
                &mut len,
                queue_family_data_graph_properties_buf.as_mut_ptr() as *mut _,
            )?;
            queue_family_data_graph_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
        queue_family_data_graph_processing_engine_properties: &mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
    ) {
        unsafe {
            (self.get_physical_device_queue_family_data_graph_processing_engine_properties_arm)(
                physical_device,
                queue_family_data_graph_processing_engine_info,
                queue_family_data_graph_processing_engine_properties,
            )
        }
    }
}
pub struct DeviceFn {
    create_data_graph_pipelines_arm: PFN_vkCreateDataGraphPipelinesARM,
    create_data_graph_pipeline_session_arm: PFN_vkCreateDataGraphPipelineSessionARM,
    get_data_graph_pipeline_session_bind_point_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM,
    get_data_graph_pipeline_session_memory_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM,
    bind_data_graph_pipeline_session_memory_arm: PFN_vkBindDataGraphPipelineSessionMemoryARM,
    destroy_data_graph_pipeline_session_arm: PFN_vkDestroyDataGraphPipelineSessionARM,
    cmd_dispatch_data_graph_arm: PFN_vkCmdDispatchDataGraphARM,
    get_data_graph_pipeline_available_properties_arm:
        PFN_vkGetDataGraphPipelineAvailablePropertiesARM,
    get_data_graph_pipeline_properties_arm: PFN_vkGetDataGraphPipelinePropertiesARM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_data_graph_pipelines_arm: transmute(
                    load(c"vkCreateDataGraphPipelinesARM").ok_or(MissingEntryPointError)?,
                ),
                create_data_graph_pipeline_session_arm: transmute(
                    load(c"vkCreateDataGraphPipelineSessionARM").ok_or(MissingEntryPointError)?,
                ),
                get_data_graph_pipeline_session_bind_point_requirements_arm: transmute(
                    load(c"vkGetDataGraphPipelineSessionBindPointRequirementsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_data_graph_pipeline_session_memory_requirements_arm: transmute(
                    load(c"vkGetDataGraphPipelineSessionMemoryRequirementsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                bind_data_graph_pipeline_session_memory_arm: transmute(
                    load(c"vkBindDataGraphPipelineSessionMemoryARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                destroy_data_graph_pipeline_session_arm: transmute(
                    load(c"vkDestroyDataGraphPipelineSessionARM").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_data_graph_arm: transmute(
                    load(c"vkCmdDispatchDataGraphARM").ok_or(MissingEntryPointError)?,
                ),
                get_data_graph_pipeline_available_properties_arm: transmute(
                    load(c"vkGetDataGraphPipelineAvailablePropertiesARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_data_graph_pipeline_properties_arm: transmute(
                    load(c"vkGetDataGraphPipelinePropertiesARM").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_data_graph_pipelines_arm(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_data_graph_pipelines_arm)(
                device,
                deferred_operation,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        create_info: &DataGraphPipelineSessionCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DataGraphPipelineSessionARM> {
        unsafe {
            let mut session = core::mem::MaybeUninit::uninit();
            let result = (self.create_data_graph_pipeline_session_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(session.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm<'a>(
        &self,
        device: Device,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM<'a>,
        mut bind_point_requirements: impl ExtendUninit<
            DataGraphPipelineSessionBindPointRequirementARM<'a>,
        >,
    ) -> crate::Result<()> {
        unsafe {
            let call = |bind_point_requirement_count, bind_point_requirements| {
                let result = (self.get_data_graph_pipeline_session_bind_point_requirements_arm)(
                    device,
                    info,
                    bind_point_requirement_count,
                    bind_point_requirements as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let bind_point_requirements_buf = bind_point_requirements.reserve(capacity);
            len = bind_point_requirements_buf.len().try_into().unwrap();
            let result = call(&mut len, bind_point_requirements_buf.as_mut_ptr() as *mut _)?;
            bind_point_requirements.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm(
        &self,
        device: Device,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_data_graph_pipeline_session_memory_requirements_arm)(
                device,
                info,
                memory_requirements,
            )
        }
    }
    pub unsafe fn bind_data_graph_pipeline_session_memory_arm(
        &self,
        device: Device,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_data_graph_pipeline_session_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        session: DataGraphPipelineSessionARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_data_graph_pipeline_session_arm)(device, session, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn cmd_dispatch_data_graph_arm(
        &self,
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM<'_>>,
    ) {
        unsafe { (self.cmd_dispatch_data_graph_arm)(command_buffer, session, info.to_raw_ptr()) }
    }
    pub unsafe fn get_data_graph_pipeline_available_properties_arm<'a>(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM<'a>,
        mut properties: impl ExtendUninit<DataGraphPipelinePropertyARM>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |properties_count, properties| {
                let result = (self.get_data_graph_pipeline_available_properties_arm)(
                    device,
                    pipeline_info,
                    properties_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_data_graph_pipeline_properties_arm(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM<'_>,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_data_graph_pipeline_properties_arm)(
                device,
                pipeline_info,
                properties.len().try_into().unwrap(),
                properties.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
