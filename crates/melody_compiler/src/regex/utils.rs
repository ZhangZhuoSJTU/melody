pub fn wrap_quantified(value: String) -> String {
    if directly_quantifiable(&value) {
        value
    } else {
        format!("(?:{value})")
    }
}

fn directly_quantifiable(value: &str) -> bool {
    // missing (and currently unsupported):
    // - \xYY
    // - \ddd
    // - \uYYYY
    let char_count = value.chars().count();

    // single char values are always quantifiable
    if char_count == 1 {
        return true;
    }

    // escaped single char values, e.g. \d, \w, \.
    if char_count == 2 {
        return value.starts_with('\\');
    }

    // longer values must be a self-contained unit:
    // a group, a character class, or a unicode property escape
    let is_group = value.starts_with('(') && value.ends_with(')');
    let is_character_class = value.starts_with('[') && value.ends_with(']');
    let is_unicode_property = (value.starts_with("\\p{") || value.starts_with("\\P{"))
        && value.ends_with('}');

    is_group || is_character_class || is_unicode_property
}

pub fn mark_lazy(quantifier: String, lazy: bool) -> String {
    if lazy {
        format!("{quantifier}?")
    } else {
        quantifier
    }
}
