# Markdown Viewer

Visor de archivos Markdown de escritorio construido con **Tauri 2**, **Vue 3**, **TypeScript** y **Vite**.

## Características

- Renderizado de Markdown con [markdown-it](https://github.com/markdown-it/markdown-it).
- Resaltado de sintaxis con [highlight.js](https://highlightjs.org/).
- Sanitización del HTML renderizado con [DOMPurify](https://github.com/cure53/DOMPurify).
- Apertura de archivos mediante diálogo nativo o doble clic desde el explorador
  (asociado a las extensiones `.md`, `.markdown`, `.mdown`, `.mkdn`).
- Instancia única: si la app ya está abierta y se abre otro archivo, la primera
  ventana lo carga y toma el foco.
- CSP restrictiva para mitigar XSS.

## Estructura

```
├── src/            Frontend Vue 3 (App.vue contiene toda la UI)
├── src-tauri/      Backend Rust (Tauri 2)
│   └── src/lib.rs  Comandos y plugins registrados
└── vite.config.ts
```

## Requisitos

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)
- Prerrequisitos de Tauri según tu SO: https://tauri.app/start/prerequisites/

## Desarrollo

```bash
npm install
npm run tauri dev
```

## Build de producción

```bash
npm run tauri build
```

El ejecutable y los instaladores se generan en `src-tauri/target/release/bundle/`.

## Scripts npm

| Script            | Descripción                                  |
| ----------------- | -------------------------------------------- |
| `dev`             | Servidor de desarrollo Vite                  |
| `build`           | Type-check (`vue-tsc`) + build de producción |
| `preview`         | Sirve el build de producción                 |
| `tauri dev`       | Ejecuta la app en modo desarrollo            |
| `tauri build`     | Genera binarios e instaladores               |
