use std::borrow::Cow;

use magnus::{
    gc::register_mark_object,
    value::{InnerValue, Lazy},
    ExceptionClass, Ruby,
};

static ERROR_CLASS: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ExceptionClass::from_value(ruby.eval("TreeHouse::Error").unwrap()).unwrap();
    register_mark_object(ex);
    ex
});

pub fn build_error(message: impl Into<Cow<'static, str>>) -> magnus::Error {
    let ruby = Ruby::get().expect("Not in Ruby thread");
    let error_class = ERROR_CLASS.get_inner_with(&ruby);
    magnus::Error::new(error_class, message)
}
