// Utilities to help build listing-related data (options, unique sorted lists, etc.)

/// Return a sorted, deduplicated Vec<String> from the provided Vec<String>
pub fn unique_sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

/// Convert a Vec<String> into options Vec<(String,String)> suitable for FilterConfig.
/// Optionally prepend a single "all" option given as (value, label).
pub fn vec_to_options(
    mut values: Vec<String>,
    prepend: Option<(&str, &str)>,
) -> Vec<(String, String)> {
    let mut opts: Vec<(String, String)> = Vec::new();
    if let Some((v, label)) = prepend {
        opts.push((v.to_string(), label.to_string()));
    }
    for v in values.drain(..) {
        opts.push((v.clone(), v));
    }
    opts
}
