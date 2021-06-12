#[path = "./rust/render.rs"]
pub mod rust;

#[path = "./typescript/render.rs"]
pub mod typescript;

use super::{
    stop,
    helpers,
    ImplementationRender,
    workflow,
    WorkflowStore,
    Protocol,
};