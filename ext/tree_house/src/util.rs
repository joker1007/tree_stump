use magnus::{
    value::{InnerValue, Lazy},
    ExceptionClass, Ruby,
};

static ERROR_CLASS: Lazy<ExceptionClass> =
    Lazy::new(|ruby| ExceptionClass::from_value(ruby.eval("TreeHouse::Error").unwrap()).unwrap());

pub fn build_error(message: String) -> magnus::Error {
    let ruby = Ruby::get().expect("Not in Ruby thread");
    let error_class = ERROR_CLASS.get_inner_with(&ruby);
    magnus::Error::new(error_class, message)
}