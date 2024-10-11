use i3m::asset::manager::ResourceManager;
use i3m::core::pool::Handle;
use i3m::resource::texture::Texture;
use i3m::scene::base::BaseBuilder;
use i3m::scene::decal::DecalBuilder;
use i3m::scene::node::Node;
use i3m::scene::Scene;

// ANCHOR: create_decal
fn create_decal(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Node> {
    DecalBuilder::new(BaseBuilder::new())
        .with_diffuse_texture(resource_manager.request::<Texture>("path/to/your/decal.png"))
        .build(&mut scene.graph)
}
// ANCHOR_END: create_decal
