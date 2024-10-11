use i3m::core::pool::Handle;
use i3m::scene::base::BaseBuilder;
use i3m::scene::collider::{ColliderBuilder, ColliderShape};
use i3m::scene::graph::Graph;
use i3m::scene::node::Node;

// ANCHOR: create_capsule_collider
fn create_capsule_collider(graph: &mut Graph) -> Handle<Node> {
    ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::capsule_y(0.5, 0.2))
        .with_friction(1.0)
        .build(graph)
}
// ANCHOR_END: create_capsule_collider
