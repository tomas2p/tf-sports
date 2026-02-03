# tf-sports

![Rust](https://img.shields.io/badge/Rust-1.93.0-orange?logo=rust&style=for-the-badge)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue?logo=rust&style=for-the-badge)
![TailwindCSS](https://img.shields.io/badge/TailwindCSS-4.1.5-38bdf8?logo=tailwindcss&style=for-the-badge)
![Estado](https://img.shields.io/badge/Estado-Activo-brightgreen?style=for-the-badge)
<!-- ![Licencia](https://img.shields.io/badge/Licencia-MIT-green?style=for-the-badge) -->

Aplicación web para explorar eventos y espacios deportivos en Tenerife. Construida con [Dioxus 0.7](https://dioxuslabs.com/learn/0.7) y Tailwind CSS.

## Características principales
- Visualización de eventos deportivos y espacios en Tenerife.
- Navegación por categorías, calendario y filtros.
- UI moderna y responsiva con Dioxus y Tailwind.
- Datos cargados desde archivos JSON y GeoJSON locales.

## Instalación y requisitos
1. Instala [Rust](https://www.rust-lang.org/tools/install) y [Cargo](https://doc.rust-lang.org/cargo/getting-started.html).
2. Instala Dioxus CLI:
   ```sh
   curl -sSL http://dioxus.dev/install.sh | sh
   ```
3. Clona el repositorio y entra en la carpeta:
   ```sh
   git clone <url-del-repo>
   cd tf-sports
   ```
4. Instala dependencias (ver [Cargo.toml](Cargo.toml)):
   ```sh
   cargo check
   ```

## Ejecución
Para lanzar la app en modo desarrollo:
```sh
dx serve
```
Esto compila y sirve la aplicación web. Accede a `http://localhost:8080` en tu navegador.

## Estructura del proyecto
- `src/` — Código fuente principal
  - [`main.rs`](src/main.rs#L1): Punto de entrada y montaje de la app.
  - [`components/`](src/components): Componentes reutilizables de UI.
  - [`views/`](src/views): Vistas principales (home, eventos, lugares, deportes).
  - [`models.rs`](src/models.rs#L1): Modelos de datos.
  - [`utils/`](src/utils): Utilidades y helpers.
- `assets/` — Imágenes y estilos (ver [`tailwind.css`](assets/tailwind.css#L1))
- `data/` — Archivos de datos locales (JSON, GeoJSON)


## Uso de datos
Los datos se cargan desde archivos locales, por ejemplo:
- [`agenda-de-eventos-deportivos-en-tenerife.json`](data/agenda-de-eventos-deportivos-en-tenerife.json)
- [`instalaciones-deportivas.geojson`](data/instalaciones-deportivas.geojson)
- [`espacios-deportivos.json`](data/espacios-deportivos.json)

## Personalización de estilos
La app utiliza Tailwind CSS. Puedes modificar [`tailwind.config.js`](tailwind.config.js#L1) y [`tailwind.css`](assets/tailwind.css#L1) para personalizar colores, fuentes y diseño.

## Contacto y soporte
Para dudas, sugerencias o soporte, abre un issue en el repositorio o contacta al mantenedor principal.

---

> Documentación generada el 3 de febrero de 2026.