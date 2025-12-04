# Documentación de Mapeo API vs Rust (Órdenes de Trabajo)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `ordenes_trabajo.rs`.

## 📌 Base URL

Prefijo: **`/api/ordenes-trabajo`**

---

## 1. CRUD Órdenes

| Endpoint (Express)    | Función Original (Rust)       | Descripción                        |
| :-------------------- | :---------------------------- | :--------------------------------- |
| **GET** `/`           | `get_ordenes_trabajo`         | Obtiene todas las órdenes activas. |
| **GET** `/:id`        | `get_orden_trabajo_by_id`     | Obtiene una orden por su ID.       |
| **POST** `/`          | `create_orden_trabajo`        | Crea una nueva orden.              |
| **PUT** `/:id`        | `update_orden_trabajo`        | Actualiza datos generales.         |
| **PUT** `/:id/estado` | `update_orden_trabajo_estado` | Actualiza solo el estado.          |
| **POST** `/delete`    | `delete_orden_trabajo`        | Elimina una orden (Soft Delete).   |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<OrdenTrabajo>`
  - `orden_id`, `orden_codigo`, `orden_desc`, `prioridad`, `estado`, `has_garantia`, `equipo_id`, `created_at`, `finished_at`.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `OrdenTrabajo` o `null`.

#### **POST** `/`

- **Body:**
  - `orden_desc`
  - `prioridad`
  - `estado`
  - `has_garantia` (Booleano)
  - `equipo_id`
  - `created_by`
  - `pre_informe`
- **Respuesta:** `OrdenTrabajo` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `orden_desc`, `prioridad`, `has_garantia`, `pre_informe` (Opcionales)
  - `updated_by`
- **Respuesta:** `OrdenTrabajo` (Objeto actualizado).

#### **PUT** `/:id/estado`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `nuevo_estado` (String)
  - `updated_by`
- **Respuesta:** `{ success: true, nuevo_estado }`

#### **POST** `/delete`

- **Body:**
  - `orden_id`
  - `deleted_by`
- **Respuesta:** `{ success: true }`

---

## 2. Consultas Relacionales y Asociaciones

| Endpoint (Express)               | Función Original (Rust)         | Descripción                        |
| :------------------------------- | :------------------------------ | :--------------------------------- |
| **GET** `/cliente/:clienteId`    | `get_ordenes_by_cliente`        | Órdenes de un cliente.             |
| **GET** `/equipo/:equipoId`      | `get_ordenes_by_equipo`         | Órdenes de un equipo.              |
| **POST** `/associate-cotizacion` | `associate_cotizacion_to_orden` | Vincula una cotización a la orden. |
| **POST** `/associate-informe`    | `associate_informe_to_orden`    | Vincula un informe a la orden.     |
| **GET** `/:id/pdf-data`          | `get_orden_trabajo_pdf_data`    | Datos para ficha de ingreso (PDF). |

### Detalles de Endpoints

#### **GET** `/cliente/:clienteId`

- **Parámetros:** `clienteId` (URL param).
- **Respuesta:** `Array<OrdenTrabajo>`.

#### **POST** `/associate-cotizacion`

- **Body:**
  - `orden_id`
  - `cotizacion_id`
  - `updated_by`
- **Respuesta:** `{ success: true }`

#### **GET** `/:id/pdf-data`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `OrdenTrabajoPdfData` (Incluye Cliente, Equipo y Recepcionista).
