use i3m::asset::manager::ResourceManager;
use i3m::scene::sound::{SoundBuffer, SoundBufferResource};

// ANCHOR: load_sound_buffer
fn build_sound_node(resource_manager: &ResourceManager) -> SoundBufferResource {
    resource_manager.request::<SoundBuffer>("/path/to/resource.ogg")
}
// ANCHOR_END: load_sound_buffer
