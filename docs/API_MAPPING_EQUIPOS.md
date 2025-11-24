# Documentación de Mapeo API vs Rust (Equipos)

Este documento detalla la relación entre los endpoints de la API REST (Express) y las funciones originales implementadas en el backend de Rust (`equipos.rs`).

## 📌 Base URL

Prefijo: **`/api/equipos`**

---

## 1. CRUD y Operaciones Básicas

| Endpoint (Express)     | Función Original (Rust)  | Descripción                                                           |
| :--------------------- | :----------------------- | :-------------------------------------------------------------------- |
| **GET** `/`            | `get_equipos`            | Obtiene todos los equipos activos (más recientes primero).            |
| **GET** `/:id`         | `get_equipo_by_id`       | Obtiene un equipo por su ID.                                          |
| **GET** `/cliente/:id` | `get_equipos_by_cliente` | Obtiene todos los equipos de un cliente específico.                   |
| **POST** `/`           | `create_equipo`          | Registra un nuevo equipo y audita la creación.                        |
| **PUT** `/:id`         | `update_equipo`          | Actualiza datos de un equipo y audita cambios.                        |
| **POST** `/delete`     | `delete_equipo`          | Inactivación lógica. Requiere JSON `{equipo_id, deleted_by, motivo}`. |

---

## 2. Filtros y Búsquedas

| Endpoint (Express)      | Función Original (Rust) | Descripción                                                |
| :---------------------- | :---------------------- | :--------------------------------------------------------- |
| **GET** `/search/query` | `search_equipos`        | Búsqueda rápida por serie, marca o modelo. Param `?term=`. |
| **POST** `/filter`      | `get_equipos_filtrados` | Filtro maestro (cliente, tipo, marca, ubicación, estado).  |

---

## 3. Listas Auxiliares (Selectores)

| Endpoint (Express)          | Función Original (Rust)       | Descripción                          |
| :-------------------------- | :---------------------------- | :----------------------------------- |
| **GET** `/list/tipos`       | `get_tipos_equipos`           | Tipos únicos (Radio, Antena, etc.).  |
| **GET** `/list/marcas`      | `get_marcas_equipos`          | Marcas únicas registradas.           |
| **GET** `/list/ubicaciones` | `get_ubicaciones_equipos`     | Ubicaciones únicas.                  |
| **GET** `/list/estados-ot`  | `get_estados_ordenes_trabajo` | Estados únicos extraídos de las OTs. |

---

## 4. Estadísticas

| Endpoint (Express)          | Función Original (Rust)               | Descripción                                                       |
| :-------------------------- | :------------------------------------ | :---------------------------------------------------------------- |
| **GET** `/stats/por-estado` | `get_estadisticas_equipos_por_estado` | Agrupa equipos según el estado de su **última** Orden de Trabajo. |

---

## ⚠️ Notas Técnicas

1. **Estadísticas Complejas:** La función `/stats/por-estado` implementa una consulta SQL avanzada con `WINDOW FUNCTIONS` (`ROW_NUMBER() OVER...`) para determinar el estado actual real de cada equipo basándose en su historial de órdenes. Esto se ha migrado tal cual a Node.js.
2. **Auditoría:** Al igual que en Rust, las actualizaciones (`PUT`) generan un log tipo "diff" mostrando qué valores cambiaron (ej: `Motorola|Ubic1` -> `Kenwood|Ubic2`).
