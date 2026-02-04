/// Utilidades simples para paginación y cálculos relacionados
pub fn paginate<T: Clone>(items: &Vec<T>, page: usize, per_page: usize) -> Vec<T> {
    if per_page == 0 { return vec![]; }
    let start = (page.saturating_sub(1)).saturating_mul(per_page);
    if start >= items.len() { return vec![]; }
    let end = (start + per_page).min(items.len());
    items[start..end].to_vec()
}

pub fn total_pages(total_items: usize, per_page: usize) -> usize {
    if per_page == 0 { return 0; }
    (total_items + per_page - 1) / per_page
}
