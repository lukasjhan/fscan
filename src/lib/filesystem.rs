use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;


pub enum FilesystemBehaviour {
    Traverse,
    OneFileSystemRoot,
    OneFileSystemChild(OneFileSystemParentInner),
}

impl FilesystemBehaviour {
    pub fn next_level(&self, going_into: &Metadata) -> Option<FilesystemBehaviour> {
        match self {
            FilesystemBehaviour::Traverse => Some(FilesystemBehaviour::Traverse),
            FilesystemBehaviour::OneFileSystemRoot => Some(FilesystemBehaviour::OneFileSystemChild(metadata_to_inner(going_into))),
            FilesystemBehaviour::OneFileSystemChild(parent) => {
                let this_inner = metadata_to_inner(going_into);
                if *parent == this_inner {
                    Some(FilesystemBehaviour::OneFileSystemChild(this_inner))
                } else {
                    None
                }
            }
        }
    }
}

type OneFileSystemParentInner = u64;

fn metadata_to_inner(meta: &Metadata) -> OneFileSystemParentInner {
    meta.dev() 
}