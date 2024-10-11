use i3m::asset::io::ResourceIo;
use i3m::asset::loader::LoaderPayload;
use i3m::asset::state::LoadError;
use i3m::plugin::{Plugin, PluginRegistrationContext};
use i3m::{
    asset::{
        loader::{BoxedLoaderFuture, ResourceLoader},
        ResourceData,
    },
    core::{
        io::{self},
        reflect::prelude::*,
        type_traits::prelude::*,
        uuid::Uuid,
        visitor::prelude::*,
        TypeUuidProvider,
    },
};
use i3m_engine_core_base::inspector::editors::resource::ResourceFieldPropertyEditorDefinition;
use i3m_engine_core_base::Editor;
use std::error::Error;
use std::sync::Arc;
use std::{
    any::Any,
    path::{Path, PathBuf},
};

// ANCHOR: custom_resource
#[derive(Default, Debug, Visit, Reflect, TypeUuidProvider)]
// Every resource must provide a unique identifier, that is used for dynamic type
// casting, serialization, etc.
#[type_uuid(id = "15551157-651b-4f1d-a5fb-6874fbfe8637")]
struct CustomResource {
    // You resource must store the path.
    path: PathBuf,
    some_data: String,
}

impl ResourceData for CustomResource {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }
}

struct CustomResourceLoader;

impl ResourceLoader for CustomResourceLoader {
    fn extensions(&self) -> &[&str] {
        // An array of extensions, supported by this loader. There could be any number of extensions
        // since sometimes multiple extensions map to a single resource (for instance, jpg, png, bmp, are
        // all images).
        &["my_resource"]
    }

    fn data_type_uuid(&self) -> Uuid {
        <CustomResource as TypeUuidProvider>::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        Box::pin(async move {
            match io::load_file(&path).await {
                Ok(content) => {
                    let my_resource = CustomResource {
                        path,
                        some_data: String::from_utf8(content).unwrap(),
                    };

                    Ok(LoaderPayload::new(my_resource))
                }
                Err(err) => Err(LoadError::new("Failed to load resource")),
            }
        })
    }
}
// ANCHOR_END: custom_resource

// ANCHOR: custom_resource_registration
#[derive(Visit, Reflect, Debug)]
struct MyGame {}

impl Plugin for MyGame {
    fn register(&self, context: PluginRegistrationContext) {
        context
            .resource_manager
            .state()
            .loaders
            .set(CustomResourceLoader);
    }
}
// ANCHOR_END: custom_resource_registration

// ANCHOR: editor_support
fn main() {
    // Your editor initialization stuff.
    let editor = Editor::new(None);

    // Register property editor.
    let sender = editor.message_sender.clone(); // Get the message sender
    editor.inspector.property_editors.insert(
        ResourceFieldPropertyEditorDefinition::<CustomResource>::new(sender),
    );

    // ...
}
// ANCHOR: editor_support
