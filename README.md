# Oxiproc 🦀

**Oxiproc** es un guardián de procesos ligero y basado en terminal (TUI) escrito en Rust. Está diseñado para ser una alternativa eficiente al Administrador de Tareas de Windows, ideal para entornos donde cada ciclo de CPU cuenta.

A diferencia de los administradores de tareas tradicionales, Oxiproc **protege tu sistema** clasificando inteligentemente los procesos para evitar que cierres componentes críticos accidentalmente.

## ✨ Características Principales

*   **🚀 Ultraligero**: Consume menos de 10MB de RAM. Perfecto para servidores o máquinas con recursos limitados.
*   **🛡️ Protección Inteligente**: Sistema de semáforo para clasificar la seguridad de los procesos y prevenir errores fatales.
*   **🎮 Interfaz TUI**: Gráficos de alta fidelidad en tu terminal, sin la pesadez de una interfaz gráfica (GUI) convencional.
*   **⚡ Rendimiento Nativo**: Escrito en Rust, se comunica directamente con las APIs del sistema eliminando el overhead gráfico.
*   **📦 Portable**: Un solo archivo ejecutable, sin dependencias externas complejas.

## 🚦 Clasificación de Seguridad

Oxiproc utiliza un sistema visual intuitivo para identificar qué procesos son seguros de manipular:

| Estado | Color | Tipo | Descripción | Acción Permitida |
| :--- | :--- | :--- | :--- | :--- |
| **CRÍTICO** | 🔴 Rojo | Sistema | Procesos vitales (Kernel, RPC, Init) | **Bloqueada** (Protección activa) |
| **PRECAUCIÓN** | 🟡 Amarillo | Servicio | Servicios en segundo plano y drivers | **Requiere Confirmación** |
| **SEGURO** | 🟢 Verde | Usuario | Tus aplicaciones (Navegadores, Editores) | **Permitida** (Libre) |

## ⌨️ Controles

| Tecla | Acción |
| :--- | :--- |
| `↑` / `↓` | Navegar por la lista de procesos |
| `k` | Matar (cerrar) el proceso seleccionado (Solo procesos de Usuario) |
| `q` / `Esc` | Salir de Oxiproc |

## 🛠️ Instalación

### Opción 1: Instalación Rápida (Binarios)
No necesitas tener Rust instalado. Simplemente descarga la última versión disponible en la sección de [Releases](https://github.com/Omar52344/oxiproc/releases).

#### 🪟 Windows
1. Descarga y extrae el archivo ZIP.
2. Busca el archivo `install.ps1`, haz **clic derecho** sobre él y selecciona **"Ejecutar con PowerShell"**.
3. El script copiará el programa y lo añadirá a tu variable PATH.
4. Reinicia tu terminal y escribe `oxiproc` para empezar.

#### 🐧 Linux
1. Descarga y extrae el archivo TAR.GZ.
2. Abre una terminal en la carpeta extraída y ejecuta:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```
3. El script instalará el binario en `~/.local/bin`. Si esa ruta no está en tu PATH, el script te indicará cómo agregarla.

---

### Opción 2: Compilar desde Código Fuente
Ideal si quieres modificar el código o contribuir al proyecto.

1.  Asegúrate de tener el [toolchain de Rust](https://www.rust-lang.org/tools/install) instalado.
2.  **Clonar el repositorio**:
    ```bash
    git clone https://github.com/Omar52344/oxiproc.git
    cd oxiproc
    ```
3.  **Compilar para producción**:
    ```bash
    cargo build --release
    ```
4.  **Ejecutar**:
    ```bash
    ./target/release/oxiproc
    ```

## 🏗️ Arquitectura Técnica

La arquitectura sigue el patrón de **Monitorización Desacoplada**:

*   **Data Provider**: Utiliza `sysinfo` para consultar el kernel periódicamente.
*   **Categorizer Engine**: Valida el UID y nombre del ejecutable contra una lista blanca de servicios críticos.
*   **UI Renderer**: Basado en `ratatui`, gestiona la interfaz en un hilo independiente para garantizar fluidez.

---
📝 **Licencia**: MIT
