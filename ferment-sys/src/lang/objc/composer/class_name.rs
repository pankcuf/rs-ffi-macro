use quote::{format_ident, ToTokens};
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::lang::objc::presentable::TypeContext;
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::Name;

pub struct ClassNameComposer {
    pub aspect: Aspect<TypeContext>
}

impl<'a> Composer<'a> for ClassNameComposer {
    type Source = (&'a str, &'a ScopeContext);
    type Output = Name;

    fn compose(&self, (class_prefix, source): &'a Self::Source) -> Self::Output {
        Name::Ident(format_ident!("{}{}", class_prefix, self.aspect.present(source).to_token_stream().to_string()))
    }
}