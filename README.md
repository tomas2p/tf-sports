<h1 style="display: flex; justify-content: center; align-items: center; gap: 10px;">
   <img src="assets/pintadera.svg" alt="Logo de tf-sports" width="80" height="80" />
   tf-sports
</h1>

<div id="badges" align="center">

[![Autor](https://img.shields.io/badge/Autor-Tomás_Pino_Pérez-gold?logo=github&style=for-the-badge)](https://tomas2p.vercel.app)
[![Version](https://img.shields.io/badge/Version-1.0.3-white?logo=version&style=for-the-badge)](https://github.com/tomas2p/tf-sports/releases)
[![Licencia_Proyecto](https://img.shields.io/badge/Licencia_Proyecto-EUPL_v1.2-blue?logo=license&style=for-the-badge)](LICENSE)
[![Licencia_Multimedia](https://img.shields.io/badge/Licencia_Multimedia-CC_BY--NC_4.0-ec5933?logo=creativecommons&style=for-the-badge)](https://creativecommons.org/licenses/by-nc/4.0/)
![Rust](https://img.shields.io/badge/Rust-1.93.0-orange?logo=rust&style=for-the-badge)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue?logo=rust&style=for-the-badge)
![TailwindCSS](https://img.shields.io/badge/TailwindCSS-4.1.5-38bdf8?logo=tailwindcss&style=for-the-badge)

Aplicación web para explorar eventos y espacios deportivos en Tenerife. Construida con [Dioxus 0.7](https://dioxuslabs.com/learn/0.7) y Tailwind CSS.
</div>

## Instrucciones de instalación

Sigue estos pasos para preparar y ejecutar la aplicación (máximo 10 pasos):

1. Instala Rust y Cargo (si no lo tienes):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clona el repositorio y entra en la carpeta:
   ```sh
   git clone <url-del-repo>
   cd tf-sports
   ```
3. Verifica dependencias y compilación rápida:
   ```sh
   cargo check
   ```
4. Instala Dioxus CLI:
   ```sh
   curl -sSL http://dioxus.dev/install.sh | sh
   ```
5. Ejecuta la app en modo desarrollo:
   ```sh
   dx serve
   ```
   Abre http://localhost:8080 en tu navegador.
6. Para compilaciones de producción o builds personalizados, usa los scripts disponibles (por ejemplo `all-build.sh` para todo o `android-build.sh` para Android, o `build-pages.sh` para web):
   ```sh
   ./scripts/all-build.sh
   ```
7. (Opcional) Si trabajas con assets o Tailwind, asegúrate de regenerar estilos según tu flujo local.

Notas:
- Solo se incluyen pasos manuales cuando aplican (por ejemplo, instalación de Rust o Dioxus CLI).
- Los datos usados por la app están en la carpeta `data/`.

## Características principales

- Visualización de eventos deportivos y espacios en Tenerife.
- Navegación por categorías, calendario y filtros.
- UI moderna y responsiva con Dioxus y Tailwind.
- Carga de datos desde archivos JSON y GeoJSON locales (no requiere servidor externo).

## Estructura del proyecto

- `src/` — Código fuente principal
   - `main.rs`: Punto de entrada y montaje de la app.
   - `models.rs`: Modelos de datos.
   - `data.rs`: Carga y manejo de los datos locales.
   - `styles.rs`, `theme.rs`: Helpers de estilos y tema.
   - `components/`: Componentes reutilizables de UI (cards, navbar, filtros, etc.).
   - `views/`: Vistas principales (home, eventos, lugares, deportes).
   - `utils/`: Utilidades y helpers compartidos.
- `assets/` — Imágenes y estilos (por ejemplo `tailwind.css`).
- `data/` — Archivos de datos locales: JSON y GeoJSON usados por la app.
- `scripts/` — Scripts de build y utilidades (`all-build.sh`, `android-build.sh`, etc.).

## Uso de datos

Los datos se cargan desde archivos locales en `data/`, por ejemplo:

- `data/agenda-de-eventos-deportivos-en-tenerife.json`
- `data/instalaciones-deportivas.geojson`
- `data/espacios-deportivos.json`

## Personalización de estilos

La app utiliza Tailwind CSS. Para ajustar estilos modifica `assets/tailwind.css` y, si es necesario, `tailwind.config.js` en la raíz del proyecto.

## Firma Android (keystore)

Para compilar y publicar builds Android firmadas se necesita un keystore y un `keystore.properties` con las credenciales. Hay un ejemplo a continuación.

Ejemplo (contenido de `keystore.properties`):

```
# keystore.properties
password=TU_PASSWORD
keyAlias=upload
storeFile=/home/TU_USUARIO/upload-keystore.jks
```

> [!warning] 
> NUNCA comites `keystore.properties` ni el `.jks` al repositorio.

Pasos resumidos:

1. Genera el keystore (ejemplo):
   ```sh
   keytool -genkey -v -keystore ~/upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload
   ```
2. Copia el ejemplo a `keystore.properties` y rellena los valores (`password`, `keyAlias`, `storeFile`).
3. Para CI (por ejemplo GitHub Actions) puedes subir el `.jks` como secret en base64:
   ```sh
   base64 -w 0 ~/upload-keystore.jks   # copiar salida como ANDROID_KEY_BASE64
   ```
4. Ajusta las variables de entorno o secrets en tu pipeline (`ANDROID_KEY_BASE64`, `ANDROID_KEY_PASSWORD`, `ANDROID_KEY_ALIAS`) y decodifica el keystore durante el build.

## Licencias

- Código fuente: [EUPL v1.2](LICENSE)
- Web, Imágenes y multimedia: [CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/)

## Contacto y soporte

Para dudas, sugerencias o soporte, abre un issue en el repositorio o contacta al mantenedor principal.