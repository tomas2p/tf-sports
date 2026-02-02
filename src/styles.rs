/// Constantes de estilos CSS reutilizables
/// 
/// Este módulo contiene clases de Tailwind CSS comúnmente usadas
/// para mantener consistencia visual en toda la aplicación.

// Colores de texto
pub const TEXT_PRIMARY: &str = "text-zinc-950 dark:text-zinc-50";
pub const TEXT_SECONDARY: &str = "text-zinc-600 dark:text-zinc-400";
pub const TEXT_TERTIARY: &str = "text-zinc-500 dark:text-zinc-500";
pub const TEXT_MUTED: &str = "text-zinc-400 dark:text-zinc-600";

// Efectos de hover
pub const CARD_HOVER: &str = "hover:shadow-md transition-shadow cursor-pointer";
pub const LINK_HOVER: &str = "hover:text-zinc-900 dark:hover:text-zinc-100";

// Gradientes
pub const GRADIENT_BG: &str = "bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900";

// Tipografía
pub const HEADING_XL: &str = "text-4xl font-bold tracking-tight";
pub const HEADING_LG: &str = "text-3xl font-bold";
pub const HEADING_MD: &str = "text-2xl font-semibold";
pub const TEXT_SM: &str = "text-sm";
pub const TEXT_XS: &str = "text-xs";

// Espaciado
pub const SPACE_Y_4: &str = "space-y-4";
pub const SPACE_Y_8: &str = "space-y-8";
pub const MB_8: &str = "mb-8";
pub const MB_4: &str = "mb-4";

// Grid layouts
pub const GRID_2: &str = "grid gap-6 md:grid-cols-2";
pub const GRID_3: &str = "grid gap-6 md:grid-cols-3";
pub const GRID_4: &str = "grid gap-6 md:grid-cols-2 lg:grid-cols-4";
pub const GRID_EVENTS: &str = "grid gap-6 md:grid-cols-2 lg:grid-cols-6";

// Helpers para combinar clases
pub fn card_interactive() -> &'static str {
    "hover:shadow-md transition-shadow cursor-pointer h-full"
}

pub fn text_label() -> &'static str {
    "block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2"
}
