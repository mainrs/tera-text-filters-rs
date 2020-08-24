//! This crate offers filters for the [Tera](https://github.com/Keats/tera) engine that are centered around text transformations.

use heck::{CamelCase, KebabCase, MixedCase, SnakeCase, TitleCase};
use std::{collections::HashMap, hash::BuildHasher};
use tera::{to_value, try_get_value, Result, Tera, Value};

/// Registers all available filters for a given `Tera` instance.
pub fn register_all(tera: &mut Tera) {
    tera.register_filter("camel_case", camel_case);
    tera.register_filter("kebab_case", kebab_case);
    tera.register_filter("lower_case", lower_case);
    tera.register_filter("mixed_case", mixed_case);
    tera.register_filter("snake_case", snake_case);
    tera.register_filter("title_case", title_case);
    tera.register_filter("upper_case", upper_case);
}

/// Converts text into CamelCase.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::camel_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("camel_case", camel_case);
///
/// let i = "{{ i | camel_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "SomeText");
/// ```
pub fn camel_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("camel_case", "value", String, value);
    Ok(to_value(&s.to_camel_case()).unwrap())
}

/// Converts text into kebab-case.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::kebab_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("kebab_case", kebab_case);
///
/// let i = "{{ i | kebab_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "some-text");
/// ```
pub fn kebab_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("kebab_case", "value", String, value);
    Ok(to_value(&s.to_kebab_case()).unwrap())
}

/// Converts text into lowercase.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::lower_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "soMe Text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("lower_case", lower_case);
///
/// let i = "{{ i | lower_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "some text");
/// ```
pub fn lower_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("lower_case", "value", String, value);
    Ok(to_value(&s.to_lowercase()).unwrap())
}

/// Converts text into mixedCase.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::mixed_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "Some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("mixed_case", mixed_case);
///
/// let i = "{{ i | mixed_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "someText");
/// ```
pub fn mixed_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("mixed_case", "value", String, value);
    Ok(to_value(&s.to_mixed_case()).unwrap())
}

/// Converts text into snake_case.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::snake_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "soMe Text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("snake_case", snake_case);
///
/// let i = "{{ i | snake_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "some_text");
/// ```
pub fn snake_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("snake_case", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}

/// Converts text into Title Case.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::title_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "soMe Text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("title_case", title_case);
///
/// let i = "{{ i | title_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "some_text");
/// ```
pub fn title_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("title_case", "value", String, value);
    Ok(to_value(&s.to_title_case()).unwrap())
}

/// Converts text into UPPERCASE.
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_text_filters::upper_case;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "soMe Text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("upper_case", upper_case);
///
/// let i = "{{ i | upper_case }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "SOME TEXT");
/// ```
pub fn upper_case<S: BuildHasher>(value: &Value, _: &HashMap<String, Value, S>) -> Result<Value> {
    let s = try_get_value!("upper_case", "value", String, value);
    Ok(to_value(&s.to_uppercase()).unwrap())
}
