use crate::classic::name::Name;
use crate::classic::path::Path;
pub struct  Module {
    pub namespace: Box<Path>,
    pub name: Name,
    pub imports: Vec<SpecOrDef>,
    pub exports: Vec<Export>,
}


// pub struct AsExported<T>{
//     pub local_name: Name
// }

pub enum Export {
    Spec(),
    Def()
}


pub trait Spec {}
pub trait Def{}

pub enum SpecOrDef {
    Spec(Box<dyn Spec>),
    Def(Box<dyn Def>),
}