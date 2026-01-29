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

## 🛠️ Instalación y Uso

### Prerrequisitos
Asegúrate de tener instalado el [toolchain de Rust](https://www.rust-lang.org/tools/install) (edición 2021 o superior).

### Pasos

1.  **Clonar el repositorio**:
    ```bash
    git clone https://github.com/tu-usuario/oxiproc.git
    cd oxiproc
    ```

2.  **Compilar para producción**:
    ```bash
    cargo build --release
    ```

3.  **Ejecutar**:
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
