# Gestión de Pacientes

Aplicación de escritorio para la gestión de pacientes y sus historiales médicos, construida con **Tauri**, **React** y **Tailwind CSS**. Permite registrar pacientes, llevar el historial de sus visitas (motivo de consulta, diagnóstico, tratamiento y notas) con archivos adjuntos, y exportar toda la ficha a PDF. Pensada para consultorios y profesionales de la salud que necesitan una herramienta simple y local para administrar sus pacientes.

## Tabla de contenidos

- [Características](#características)
- [Stack tecnológico](#stack-tecnológico)
- [Requisitos previos](#requisitos-previos)
- [Instalación](#instalación)
- [Uso](#uso)
- [Scripts disponibles](#scripts-disponibles)
- [Estructura del proyecto](#estructura-del-proyecto)
- [Compilar para producción](#compilar-para-producción)
- [Licencia](#licencia)

## Características

- 🖥️ **Aplicación de escritorio multiplataforma** gracias a Tauri (Windows, macOS y Linux).
- 📋 **Gestión de pacientes**: alta, edición y búsqueda de pacientes por nombre o DNI.
- 🗂️ **Ficha de paciente** con datos personales, DNI, teléfono, obra social y número de afiliado.
- 🩺 **Historial de visitas**: registro de fecha de consulta, motivo, diagnóstico, tratamiento y notas adicionales por cada visita.
- 📎 **Archivos adjuntos** por visita (estudios, órdenes, imágenes, etc.).
- 📄 **Exportación a PDF** de la ficha completa del paciente: datos personales, historial de visitas y adjuntos.
- 🔍 **Listados eficientes** de gran volumen de datos mediante virtualización con `react-virtuoso`.
- 🎨 **Interfaz moderna** basada en componentes de Radix UI y `shadcn/ui`.

## Stack tecnológico

**Frontend**

- [React js](https://react.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Radix UI](https://www.radix-ui.com/) / [shadcn/ui](https://ui.shadcn.com/)
- [React Day Picker](https://daypicker.dev/) y [date-fns](https://date-fns.org/)
- [React Virtuoso](https://virtuoso.dev/)
- [Lucide Icons](https://lucide.dev/)
- [Sonner](https://sonner.emilkowal.ski/)

**Backend / Runtime de escritorio**

- [Tauri](https://tauri.app/) (Rust)

## Requisitos previos

Antes de instalar el proyecto, asegurate de tener:

- [Node.js](https://nodejs.org/)
- [Rust y Cargo](https://www.rust-lang.org/tools/install)
- Dependencias del sistema requeridas por Tauri según tu sistema operativo (ver la [guía oficial de prerrequisitos de Tauri](https://tauri.app/start/prerequisites/))

## Instalación

1. Cloná el repositorio:

   ```bash
   git clone https://github.com/Noxde/Gestion-Pacientes.git
   cd Gestion-Pacientes
   ```

2. Instalá las dependencias:

   ```bash
   npm install
   ```

## Uso

### Modo desarrollo (aplicación de escritorio)

Levanta la aplicación completa dentro de la ventana nativa de Tauri, con recarga en caliente:

```bash
npm run tauri dev
```

## Scripts disponibles

| Comando               | Descripción                                             |
| --------------------- | ------------------------------------------------------- |
| `npm run tauri dev`   | Ejecuta la app de escritorio en modo desarrollo         |
| `npm run tauri build` | Genera el instalador/ejecutable de la app de escritorio |

## Estructura del proyecto

```
Gestion-Pacientes/
├── .github/workflows/
├── public/              # Archivos estáticos
├── src/                 # Código fuente de la interfaz (React)
├── src-tauri/           # Backend nativo de la app (Rust) y configuración de Tauri
├── components.json      # Configuración de shadcn/ui
├── index.html           # Punto de entrada HTML
├── vite.config.js        # Configuración de Vite
└── package.json
```

## Compilar para producción

Para generar el instalador o ejecutable nativo de tu sistema operativo:

```bash
npm run tauri build
```

El resultado se genera dentro de `src-tauri/target/release/`, junto con los instaladores empaquetados (`.msi`/`.exe` en Windows, `.dmg`/`.app` en macOS, `.deb`/`.AppImage` en Linux) en `src-tauri/target/release/bundle/`.

## Licencia

MIT
