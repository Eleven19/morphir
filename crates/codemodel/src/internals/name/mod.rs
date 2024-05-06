use std::ops::Deref;

pub struct Name<Repr>(Repr);

pub trait NameRepr: Deref<Target = str> {}