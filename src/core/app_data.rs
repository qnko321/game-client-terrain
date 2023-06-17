use std::collections::HashMap;
use vulkanalia::vk;
use crate::graphics::font_data::FontData;
use crate::graphics::text_object::TextObject;
use crate::graphics::text_textures::Character;
use crate::graphics::vertex::Vertex;

#[derive(Clone, Debug, Default)]
pub(crate) struct AppData {
    // Debug
    pub(crate) messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    pub(crate) surface: vk::SurfaceKHR,
    // Physical Device / Logical Device
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) graphics_queue: vk::Queue,
    pub(crate) present_queue: vk::Queue,
    // Swapchain
    pub(crate) swapchain_format: vk::Format,
    pub(crate) swapchain_extent: vk::Extent2D,
    pub(crate) swapchain: vk::SwapchainKHR,
    pub(crate) swapchain_images: Vec<vk::Image>,
    pub(crate) swapchain_image_views: Vec<vk::ImageView>,
    // Pipeline
    pub(crate) render_pass: vk::RenderPass,
    pub(crate) descriptor_set_layout: vk::DescriptorSetLayout,
    pub(crate) pipeline_layout: vk::PipelineLayout,
    pub(crate) pipeline: vk::Pipeline,
    // Text Pipeline
    pub(crate) text_render_pass: vk::RenderPass,
    pub(crate) text_descriptor_set_layout: vk::DescriptorSetLayout,
    pub(crate) text_pipeline_layout: vk::PipelineLayout,
    pub(crate) text_pipeline: vk::Pipeline,
    // Framebuffers
    pub(crate) framebuffers: Vec<vk::Framebuffer>,
    // Command Pool
    pub(crate) command_pool: vk::CommandPool,
    // Texture
    pub(crate) texture_image: vk::Image,
    pub(crate) texture_image_memory: vk::DeviceMemory,
    pub(crate) texture_image_view: vk::ImageView,
    pub(crate) texture_sampler: vk::Sampler,
    // Text Texture/s
    pub(crate) text_texture_image: vk::Image,
    pub(crate) text_texture_image_memory: vk::DeviceMemory,
    pub(crate) text_texture_image_view: vk::ImageView,
    pub(crate) text_texture_sampler: vk::Sampler,
    // 3D Objects
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    // Buffers
    pub(crate) vertex_buffer: vk::Buffer,
    pub(crate) vertex_buffer_memory: vk::DeviceMemory,
    pub(crate) index_buffer: vk::Buffer,
    pub(crate) index_buffer_memory: vk::DeviceMemory,
    pub(crate) uniform_buffers: Vec<vk::Buffer>,
    pub(crate) uniform_buffers_memory: Vec<vk::DeviceMemory>,
    // Descriptors
    pub(crate) descriptor_pool: vk::DescriptorPool,
    pub(crate) descriptor_sets: Vec<vk::DescriptorSet>,
    //Text Descriptors
    pub(crate) text_descriptor_pool: vk::DescriptorPool,
    pub(crate) text_descriptor_sets: Vec<vk::DescriptorSet>,
    // Commands
    pub(crate) command_pools: Vec<vk::CommandPool>,
    pub(crate) command_buffers: Vec<vk::CommandBuffer>,
    pub(crate) secondary_command_buffers: Vec<Vec<vk::CommandBuffer>>,
    // Text Commands
    pub(crate) text_command_pools: Vec<vk::CommandPool>,
    pub(crate) text_command_buffers: Vec<vk::CommandBuffer>,
    pub(crate) text_secondary_command_buffers: Vec<Vec<vk::CommandBuffer>>,
    // Text
    pub(crate) text_characters: HashMap<u32, Character>,
    pub(crate) text_objects: Vec<TextObject>,
    pub(crate) font_data: FontData,
    // Text Buffers
    pub(crate) text_vertex_buffer: vk::Buffer,
    pub(crate) text_vertex_buffer_memory: vk::DeviceMemory,
    pub(crate) text_index_buffer_length: usize,
    pub(crate) text_index_buffer: vk::Buffer,
    pub(crate) text_index_buffer_memory: vk::DeviceMemory,
    // Sync Objects
    pub(crate) image_available_semaphores: Vec<vk::Semaphore>,
    pub(crate) render_finished_semaphores: Vec<vk::Semaphore>,
    pub(crate) in_flight_fences: Vec<vk::Fence>,
    pub(crate) images_in_flight: Vec<vk::Fence>,
    // Depth
    pub(crate) depth_image: vk::Image,
    pub(crate) depth_image_memory: vk::DeviceMemory,
    pub(crate) depth_image_view: vk::ImageView,
}
