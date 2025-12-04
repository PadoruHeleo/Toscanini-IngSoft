# Documentación de Mapeo API vs Rust (Clientes)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `clientes.rs`.

## 📌 Base URL

Prefijo: **`/api/clientes`**

---

## 1. CRUD Clientes

| Endpoint (Express)     | Función Original (Rust) | Descripción                         |
| :--------------------- | :---------------------- | :---------------------------------- |
| **GET** `/`            | `get_clientes`          | Obtiene todos los clientes activos. |
| **GET** `/:id`         | `get_cliente_by_id`     | Obtiene un cliente por su ID.       |
| **GET** `/rut/:rut`    | `get_cliente_by_rut`    | Obtiene un cliente por su RUT.      |
| **POST** `/`           | `create_cliente`        | Crea un nuevo cliente.              |
| **PUT** `/:id`         | `update_cliente`        | Actualiza datos de un cliente.      |
| **POST** `/delete`     | `delete_cliente`        | Inactiva un cliente (Soft Delete).  |
| **POST** `/reactivate` | `reactivate_cliente`    | Reactiva un cliente eliminado.      |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Cliente>`
  - `cliente_id`, `cliente_rut`, `cliente_nombre`, `cliente_correo`, `cliente_telefono`, `cliente_direccion`, `is_active`, `created_at`.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Cliente` o `null`.

#### **POST** `/`

- **Body:**
  - `cliente_rut` (Requerido)
  - `cliente_nombre` (Requerido)
  - `cliente_correo`
  - `cliente_telefono`
  - `cliente_direccion`
  - `created_by` (ID usuario)
- **Respuesta:** `Cliente` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `cliente_rut`, `cliente_nombre`, `cliente_correo`, `cliente_telefono`, `cliente_direccion` (Opcionales)
  - `updated_by` (ID usuario)
- **Respuesta:** `Cliente` (Objeto actualizado).

#### **POST** `/delete`

- **Body:**
  - `cliente_id`
  - `deleted_by`
  - `motivo`
- **Respuesta:** `{ success: true }`

#### **POST** `/reactivate`

- **Body:**
  - `cliente_id`
  - `reactivated_by`
- **Respuesta:** `{ success: true }`

---

## 2. Búsqueda y Filtros

| Endpoint (Express)      | Función Original (Rust)  | Descripción                   |
| :---------------------- | :----------------------- | :---------------------------- |
| **GET** `/search/query` | `search_clientes`        | Búsqueda simple por texto.    |
| **POST** `/filter`      | `get_clientes_filtrados` | Filtros avanzados combinados. |

### Detalles de Endpoints

#### **GET** `/search/query`

- **Query Params:** `term` (Texto a buscar).
- **Respuesta:** `Array<Cliente>` (Coincidencias en nombre, rut o correo).

#### **POST** `/filter`

- **Body:**
  - `fecha_inicio`, `fecha_fin` (Strings fecha)
  - `correo`, `rut`, `ciudad`, `estado` (Arrays de strings/booleanos)
  - `search` (Texto)
  - `ordenamiento` ("asc" | "desc")
- **Respuesta:** `Array<Cliente>`

---

## 3. Listas Auxiliares (Filtros)

| Endpoint (Express)       | Función Original (Rust) | Descripción              |
| :----------------------- | :---------------------- | :----------------------- |
| **GET** `/list/ruts`     | `get_ruts_clientes`     | Lista única de RUTs.     |
| **GET** `/list/emails`   | `get_correos_clientes`  | Lista única de Correos.  |
| **GET** `/list/ciudades` | `get_ciudades_clientes` | Lista única de Ciudades. |

### Detalles de Endpoints

#### **GET** `/list/*`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<String>` (Lista de valores únicos para dropdowns).
