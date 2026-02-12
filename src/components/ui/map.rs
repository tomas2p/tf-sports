use dioxus::prelude::*;

#[component]
pub fn Map(
    lat: f64,
    lon: f64,
    zoom: Option<u8>,
    height: Option<String>,
    name: Option<String>,
) -> Element {
    let z = zoom.unwrap_or(15);
    let h = height.unwrap_or_else(|| "300px".to_string());
    let title = name.unwrap_or_else(|| "Ubicación".to_string());
    let esc_title = title.replace('"', "&quot;").replace('\'', "&#39;");

    let html = format!(
        r###"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
  <style>html,body,#map {{ height:100%; margin:0; padding:0; }}</style>
</head>
<body>
  <div id="map" aria-label="{esc_title}"></div>
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <script>
    (function() {{
      var lat = {lat};
      var lon = {lon};
      var z = {z};
      var lightUrl = 'https://tiles.stadiamaps.com/tiles/alidade/{z}/{{x}}/{{y}}.png';
      var darkUrl = 'https://tiles.stadiamaps.com/tiles/alidade_dark/{z}/{{x}}/{{y}}.png';
      var attr = '&copy; <a href="https://stadiamaps.com/">Stadia Maps</a> &amp; <a href="https://openmaptiles.org/">OpenMapTiles</a> &amp; <a href="https://openstreetmap.org">OSM</a>';

      var map = L.map('map', {{ center: [lat, lon], zoom: z, zoomControl: false }});

      function setLayer(isDark) {{
        if (map._tileLayer) map.removeLayer(map._tileLayer);
        var url = isDark ? darkUrl : lightUrl;
        map._tileLayer = L.tileLayer(url, {{ attribution: attr, maxZoom: 20 }}).addTo(map);
      }}

      var isDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
      setLayer(isDark);

      if (window.matchMedia) {{
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {{
          setLayer(e.matches);
        }});
      }}

      // Also detect theme toggles from the parent document (e.g. Tailwind `dark` class toggles).
      function updateFromParentPref() {{
        try {{
          var parentDark = false;
          if (window.parent && window.parent.document && window.parent.document.documentElement) {{
            parentDark = window.parent.document.documentElement.classList.contains('dark');
          }}
          if (parentDark !== isDark) {{
            isDark = parentDark;
            setLayer(isDark);
          }}
        }} catch (e) {{}}
      }}
      updateFromParentPref();
      setInterval(updateFromParentPref, 500);

      L.marker([lat, lon]).addTo(map).bindPopup('{esc_title}');
    }})();
  </script>
</body>
</html>"###,
        lat = lat,
        lon = lon,
        z = z,
        esc_title = esc_title
    );

    let style = format!("border:0; display:block; height:{};", h);

    rsx! {
        div { class: "overflow-hidden rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm",
                iframe { srcdoc: html, width: "100%", height: h, frame_border: "0", style: style }
        }
    }
}
