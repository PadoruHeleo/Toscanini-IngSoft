# Documentación de Mapeo API vs Rust (Equipos)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `equipos.rs`.

## 📌 Base URL

Prefijo: **`/api/equipos`**

---

## 1. CRUD Equipos

| Endpoint (Express)            | Función Original (Rust)  | Descripción                        |
| :---------------------------- | :----------------------- | :--------------------------------- |
| **GET** `/`                   | `get_equipos`            | Obtiene todos los equipos activos. |
| **GET** `/:id`                | `get_equipo_by_id`       | Obtiene un equipo por su ID.       |
| **GET** `/cliente/:clienteId` | `get_equipos_by_cliente` | Obtiene equipos de un cliente.     |
| **POST** `/`                  | `create_equipo`          | Crea un nuevo equipo.              |
| **PUT** `/:id`                | `update_equipo`          | Actualiza datos de un equipo.      |
| **POST** `/delete`            | `delete_equipo`          | Elimina un equipo (Soft Delete).   |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Equipo>`
  - `equipo_id`, `numero_serie`, `equipo_marca`, `equipo_modelo`, `equipo_tipo`, `equipo_precio`, `equipo_ubicacion`, `cliente_id`, `created_at`.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Equipo` o `null`.

#### **GET** `/cliente/:clienteId`

- **Parámetros:** `clienteId` (URL param).
- **Respuesta:** `Array<Equipo>`.

#### **POST** `/`

- **Body:**
  - `numero_serie` (Requerido)
  - `equipo_marca`
  - `equipo_modelo`
  - `equipo_tipo`
  - `equipo_precio`
  - `equipo_ubicacion`
  - `cliente_id` (Requerido)
  - `created_by` (ID usuario)
- **Respuesta:** `Equipo` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `numero_serie`, `equipo_marca`, `equipo_modelo`, `equipo_tipo`, `equipo_precio`, `equipo_ubicacion` (Opcionales)
  - `updated_by` (ID usuario)
- **Respuesta:** `Equipo` (Objeto actualizado).

#### **POST** `/delete`

- **Body:**
  - `equipo_id`
  - `deleted_by`
  - `motivo`
- **Respuesta:** `{ success: true }`

---

## 2. Búsqueda y Filtros

| Endpoint (Express)      | Función Original (Rust) | Descripción                   |
| :---------------------- | :---------------------- | :---------------------------- |
| **GET** `/search/query` | `search_equipos`        | Búsqueda simple por texto.    |
| **POST** `/filter`      | `get_equipos_filtrados` | Filtros avanzados combinados. |

### Detalles de Endpoints

#### **GET** `/search/query`

- **Query Params:** `term` (Texto a buscar).
- **Respuesta:** `Array<Equipo>` (Coincidencias en serie, marca o modelo).

#### **POST** `/filter`

- **Body:**
  - `cliente_id`, `tipo`, `marca`, `ubicacion` (Arrays)
  - `search` (Texto)
  - `ordenamiento` ("asc" | "desc")
- **Respuesta:** `Array<Equipo>`

---

## 3. Listas Auxiliares y Estadísticas

| Endpoint (Express)          | Función Original (Rust)       | Descripción                                     |
| :-------------------------- | :---------------------------- | :---------------------------------------------- |
| **GET** `/list/tipos`       | `get_tipos_equipos`           | Lista única de Tipos.                           |
| **GET** `/list/marcas`      | `get_marcas_equipos`          | Lista única de Marcas.                          |
| **GET** `/list/ubicaciones` | `get_ubicaciones_equipos`     | Lista única de Ubicaciones.                     |
| **GET** `/list/estados`     | `get_estados_ordenes`         | Lista de estados de órdenes (usado en filtros). |
| **GET** `/stats/estados`    | `get_estadisticas_por_estado` | Cantidad de equipos por estado de orden.        |

### Detalles de Endpoints

#### **GET** `/list/*`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<String>` (Lista de valores únicos).

#### **GET** `/stats/estados`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<{ estado: String, cantidad: Number }>`
