use crate::core::compile_context::CompileContext;
use crate::core::metadata::util;
use crate::core::metadata::RootMetadataProcessor;
use crate::core::node::MetadataNode;

#[derive(Debug)]
pub struct Filename;

impl RootMetadataProcessor for Filename {
    fn name(&self) -> &str {
        "filename"
    }
    fn process(&self, context: &mut CompileContext, metadatas: Vec<MetadataNode>) {
        if metadatas.is_empty() {
            return;
        }
        for metadata in (&metadatas).into_iter().take(metadatas.len() - 1) {
            util::no_duplicate(&metadata);
        }
        let last = metadatas.last().expect("Guaranteed by caller");
        let param = match util::check_param_exact(1, &last, false) {
            util::ParameterType::Less => return,
            util::ParameterType::Matched | util::ParameterType::Over => &last.parameters[0],
        };
        context.filename = param.clone();
    }
}
