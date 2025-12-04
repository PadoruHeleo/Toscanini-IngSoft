# Documentación de Mapeo API vs Rust (Informes Técnicos)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `informe_tecnico.rs`.

## 📌 Base URL

Prefijo: **`/api/informes`**

---

## 1. CRUD Informes

| Endpoint (Express)    | Función Original (Rust) | Descripción                                     |
| :-------------------- | :---------------------- | :---------------------------------------------- |
| **GET** `/`           | `get_informes`          | Obtiene todos los informes activos.             |
| **GET** `/:id`        | `get_informe_by_id`     | Obtiene un informe por su ID.                   |
| **POST** `/`          | `create_informe`        | Crea un nuevo informe (Borrador).               |
| **PUT** `/:id`        | `update_informe`        | Actualiza datos del informe.                    |
| **POST** `/delete`    | `delete_informe`        | Elimina un informe (Soft Delete).               |
| **POST** `/finalizar` | `finalizar_informe`     | Marca el informe como finalizado (no borrador). |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Informe>`
  - `informe_id`, `informe_codigo`, `orden_id`, `diagnostico`, `is_borrador`, `created_at`, etc.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Informe` o `null`.

#### **POST** `/`

- **Body:**
  - `informe_codigo`
  - `orden_id`
  - `informe_acciones`, `informe_obs`, `diagnostico`, `recomendaciones`, `solucion_aplicada`
  - `tecnico_responsable`
  - `created_by`
- **Respuesta:** `Informe` (Objeto creado, `is_borrador: true`).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `informe_acciones`, `informe_obs`, `diagnostico`, `recomendaciones`, `solucion_aplicada`, `tecnico_responsable` (Opcionales)
  - `updated_by`
- **Respuesta:** `Informe` (Objeto actualizado).

#### **POST** `/delete`

- **Body:**
  - `informe_id`
  - `deleted_by`
- **Respuesta:** `{ success: true }`

#### **POST** `/finalizar`

- **Body:**
  - `informe_id`
  - `user_id`
- **Respuesta:** `{ success: true }`

---

## 2. Gestión de Piezas (Items)

| Endpoint (Express)    | Función Original (Rust) | Descripción                          |
| :-------------------- | :---------------------- | :----------------------------------- |
| **GET** `/:id/piezas` | `get_piezas_informe`    | Obtiene piezas usadas en el informe. |
| **PUT** `/:id/piezas` | `update_informe_piezas` | Actualiza la lista de piezas.        |

### Detalles de Endpoints

#### **GET** `/:id/piezas`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Array<PiezaInforme>`
  - `pieza_id`, `pieza_nombre`, `cantidad`, ...

#### **PUT** `/:id/piezas`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `piezas`: Array de objetos `{ pieza_id: number, cantidad: number }`
  - `updated_by`
- **Respuesta:** `{ success: true }`

---

## 3. Consultas Relacionales y PDF

| Endpoint (Express)            | Función Original (Rust)   | Descripción                               |
| :---------------------------- | :------------------------ | :---------------------------------------- |
| **GET** `/cliente/:clienteId` | `get_informes_by_cliente` | Informes de un cliente.                   |
| **GET** `/equipo/:equipoId`   | `get_informes_by_equipo`  | Informes de un equipo.                    |
| **GET** `/:id/pdf-data`       | `get_informe_pdf_data`    | Datos completos para generar PDF.         |
| **POST** `/send`              | `send_informe_to_client`  | Registra el envío del informe por correo. |

### Detalles de Endpoints

#### **GET** `/cliente/:clienteId`

- **Parámetros:** `clienteId` (URL param).
- **Respuesta:** `Array<Informe>` (Resumen).

#### **GET** `/:id/pdf-data`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `InformePdfData` (Incluye datos de Cliente y Equipo).

#### **POST** `/send`

- **Body:**
  - `informe_id`
  - `sent_by`
  - `destinatario` (Email)
- **Respuesta:** `{ success: true }`
