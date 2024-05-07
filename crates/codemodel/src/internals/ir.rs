use crate::classic::name::Name;

use rdf_types::dataset::Graph;

pub enum Type {
    Variable(TypeVariable),
    Unit,
}

pub struct TypeVariable {
    pub name: Name,
}

pub enum Value {
    Literal(Literal),
}

pub enum Literal {
    Bool(bool),
    Unit,
}

pub enum Data {
    Bool(bool),
    String(String),
    Unit,
}

trait AnnotationGraph {}

trait With {}

pub struct TypeMeta<T> {
    pub annotations: Box<dyn AnnotationGraph>,
    pub type_info: T,
}
pub struct ValueAnnotation<T> {
    pub type_info: T,
    pub annotations: Box<dyn AnnotationGraph>,
}

pub enum NodeId {
    TypeId(),
    ValueId(),
    ModuleId(),
}
