use std::fmt;

use crate::{
    expression::{Array, Block, Group, Object, Resolved, Value},
    state::{ExternalEnv, LocalEnv},
    BatchContext, Context, Expression, TypeDef,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub variant: Variant,
}

impl Container {
    #[must_use]
    pub fn new(variant: Variant) -> Self {
        Self { variant }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Group(Group),
    Block(Block),
    Array(Array),
    Object(Object),
}

impl Expression for Container {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        use Variant::{Array, Block, Group, Object};

        match &self.variant {
            Group(v) => v.resolve(ctx),
            Block(v) => v.resolve(ctx),
            Array(v) => v.resolve(ctx),
            Object(v) => v.resolve(ctx),
        }
    }

    fn resolve_batch(&mut self, ctx: &mut BatchContext, selection_vector: &[usize]) {
        use Variant::{Array, Block, Group, Object};

        match &mut self.variant {
            Group(v) => v.resolve_batch(ctx, selection_vector),
            Block(v) => v.resolve_batch(ctx, selection_vector),
            Array(v) => v.resolve_batch(ctx, selection_vector),
            Object(v) => v.resolve_batch(ctx, selection_vector),
        }
    }

    fn as_value(&self) -> Option<Value> {
        use Variant::{Array, Block, Group, Object};

        match &self.variant {
            Group(v) => v.as_value(),
            Block(v) => v.as_value(),
            Array(v) => v.as_value(),
            Object(v) => v.as_value(),
        }
    }

    fn type_def(&self, state: (&LocalEnv, &ExternalEnv)) -> TypeDef {
        use Variant::{Array, Block, Group, Object};

        match &self.variant {
            Group(v) => v.type_def(state),
            Block(v) => v.type_def(state),
            Array(v) => v.type_def(state),
            Object(v) => v.type_def(state),
        }
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Variant::{Array, Block, Group, Object};

        match &self.variant {
            Group(v) => v.fmt(f),
            Block(v) => v.fmt(f),
            Array(v) => v.fmt(f),
            Object(v) => v.fmt(f),
        }
    }
}

impl From<Group> for Variant {
    fn from(group: Group) -> Self {
        Variant::Group(group)
    }
}

impl From<Block> for Variant {
    fn from(block: Block) -> Self {
        Variant::Block(block)
    }
}

impl From<Array> for Variant {
    fn from(array: Array) -> Self {
        Variant::Array(array)
    }
}

impl From<Object> for Variant {
    fn from(object: Object) -> Self {
        Variant::Object(object)
    }
}
