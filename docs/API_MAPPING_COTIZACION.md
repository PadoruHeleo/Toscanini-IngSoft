# Documentación de Mapeo API vs Rust (Cotizaciones)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `cotizacion.rs`.

## 📌 Base URL

Prefijo: **`/api/cotizaciones`**

---

## 1. CRUD Cotización

| Endpoint (Express) | Función Original (Rust) | Descripción                                   |
| :----------------- | :---------------------- | :-------------------------------------------- |
| **GET** `/`        | `get_cotizaciones`      | Obtiene todas las cotizaciones activas.       |
| **GET** `/:id`     | `get_cotizacion_by_id`  | Obtiene detalles de una cotización.           |
| **POST** `/`       | `create_cotizacion`     | Crea cotización y copia términos por defecto. |
| **PUT** `/:id`     | `update_cotizacion`     | Actualiza costos (revisión/reparación).       |
| **POST** `/delete` | `delete_cotizacion`     | Soft delete (`deleted_at`).                   |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Cotizacion>`

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Cotizacion` o `null`.

#### **POST** `/`

- **Body:**
  - `costo_revision`
  - `costo_reparacion`
  - `informe`
  - `created_by`
  - `piezas` (Array opcional)
  - `is_aprobada`, `is_borrador` (Opcionales)
- **Respuesta:** `Cotizacion` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `cotizacion_codigo`, `costo_revision`, `costo_reparacion`, `costo_total`, `is_aprobada`, `is_borrador`, `informe` (Opcionales)
  - `updated_by`
- **Respuesta:** `Cotizacion` (Objeto actualizado).

#### **POST** `/delete`

- **Body:**
  - `cotizacion_id`
  - `deleted_by`
- **Respuesta:** `{ success: true }`

**Campos de Respuesta (Cotizacion):**

- `cotizacion_codigo` (Ej: "COT-2024-001")
- `costo_total`
- `is_aprobada` (Booleano)
- `is_borrador` (Booleano)
- `informe`
- `created_at`
- `costo_revision`, `costo_reparacion`

---

## 2. Gestión de Piezas (Items)

| Endpoint (Express)    | Función Original (Rust)    | Descripción                                                        |
| :-------------------- | :------------------------- | :----------------------------------------------------------------- |
| **GET** `/:id/piezas` | `get_piezas_cotizacion`    | Lista piezas asociadas a la cotización.                            |
| **PUT** `/:id/piezas` | `update_cotizacion_piezas` | Reemplaza todas las piezas (Borra y crea nuevas). Usa transacción. |

### Detalles de Endpoints

#### **GET** `/:id/piezas`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Array<PiezaCotizacion>`.

#### **PUT** `/:id/piezas`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `piezas`: Array de objetos `{ pieza_id: number, cantidad: number }`
  - `updated_by`
- **Respuesta:** `{ success: true }`

**Campos de Respuesta (PiezaCotizacion):**

- `pieza_nombre`
- `cantidad`
- `pieza_precio` (Unitario)
- `pieza_stock` (Stock actual)
- `pieza_marca`, `pieza_desc`

---

## 3. Acciones de Negocio

| Endpoint (Express)   | Función Original (Rust) | Descripción                                                      |
| :------------------- | :---------------------- | :--------------------------------------------------------------- |
| **POST** `/aprobar`  | `aprobar_cotizacion`    | Marca `is_aprobada = true`.                                      |
| **POST** `/duplicar` | `duplicate_cotizacion`  | Crea una copia exacta (piezas + términos) para un nuevo informe. |

### Detalles de Endpoints

#### **POST** `/aprobar`

- **Body:**
  - `cotizacion_id`
  - `approved_by`
- **Respuesta:** `{ success: true }`

#### **POST** `/duplicar`

- **Body:**
  - `cotizacion_id`
  - `created_by`
  - `new_informe_id` (Opcional)
- **Respuesta:** `Cotizacion` (La nueva copia).

---

## 4. Consultas Relacionales y PDF

| Endpoint (Express)      | Función Original (Rust)       | Descripción                                               |
| :---------------------- | :---------------------------- | :-------------------------------------------------------- |
| **GET** `/cliente/:id`  | `get_cotizaciones_by_cliente` | Historial de cotizaciones de un cliente.                  |
| **GET** `/equipo/:id`   | `get_cotizaciones_by_equipo`  | Historial de cotizaciones de un equipo.                   |
| **GET** `/:id/pdf-data` | `get_cotizacion_pdf_data`     | Datos completos (Cliente+Equipo+Montos) para generar PDF. |

### Detalles de Endpoints

#### **GET** `/cliente/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Array<Cotizacion>`.

#### **GET** `/:id/pdf-data`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `CotizacionPdfData`.

---

## 5. Gestión de Piezas (Catálogo)

**Prefijo:** `/api/piezas`

| Endpoint (Express) | Función Original (Rust) | Descripción                          |
| :----------------- | :---------------------- | :----------------------------------- |
| **GET** `/`        | `get_piezas`            | Obtiene catálogo completo de piezas. |
| **GET** `/:id`     | `get_pieza_by_id`       | Obtiene detalle de una pieza.        |
| **POST** `/`       | `create_pieza`          | Crea una nueva pieza en el catálogo. |
| **PUT** `/:id`     | `update_pieza`          | Actualiza datos de una pieza.        |
| **DELETE** `/:id`  | `delete_pieza`          | Elimina una pieza del catálogo.      |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Pieza>`.

#### **POST** `/`

- **Body:**
  - `pieza_nombre`, `pieza_marca`, `pieza_desc`, `pieza_precio`, `pieza_stock`, `created_by`
- **Respuesta:** `Pieza` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `pieza_nombre`, `pieza_marca`, `pieza_desc`, `pieza_precio`, `pieza_stock` (Opcionales)
  - `updated_by`
- **Respuesta:** `Pieza` (Objeto actualizado).

**Campos de Respuesta (Pieza):**

- `pieza_nombre`, `pieza_marca`, `pieza_desc`
- `pieza_precio`
- `pieza_stock`

---

## 6. Inventario de Equipos

**Prefijo:** `/api/inventario-equipos`

| Endpoint (Express)    | Función Original (Rust)          | Descripción                             |
| :-------------------- | :------------------------------- | :-------------------------------------- |
| **GET** `/`           | `get_inventario_equipos`         | Lista todo el inventario de equipos.    |
| **POST** `/`          | `create_inventario_equipo`       | Registra un nuevo equipo en inventario. |
| **PUT** `/:id`        | `update_inventario_equipo`       | Actualiza datos de un equipo.           |
| **DELETE** `/:id`     | `delete_inventario_equipo`       | Elimina un equipo del inventario.       |
| **POST** `/:id/stock` | `update_inventario_equipo_stock` | Ajusta stock (+/-) de un equipo.        |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<InventarioEquipo>`.

#### **POST** `/`

- **Body:**
  - `equipo_codigo` (Opcional, se autogenera)
  - `equipo_nombre`, `equipo_marca`, `equipo_modelo`, `equipo_tipo`
  - `equipo_descripcion`, `equipo_precio`, `equipo_stock`
  - `equipo_estado`, `equipo_ubicacion`
  - `fecha_adquisicion`, `proveedor`, `numero_serie`, `garantia_vencimiento`, `observaciones`
  - `created_by`
- **Respuesta:** `InventarioEquipo` (Objeto creado).

#### **POST** `/:id/stock`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `cantidad` (Number)
  - `tipo` ("add" | "remove")
  - `updated_by`
- **Respuesta:** `{ success: true, new_stock }`

**Campos de Respuesta (InventarioEquipo):**

- `equipo_tipo`, `equipo_marca`, `equipo_modelo`
- `numero_serie`
- `equipo_estado`, `equipo_ubicacion`

> **⚠️ ATENCIÓN:** La API retorna los siguientes campos adicionales que faltan en el modelo Rust actual:
>
> - `equipo_nombre`
> - `equipo_stock`
> - `equipo_precio`
> - `equipo_descripcion`

---

## 7. Salidas de Equipos

**Prefijo:** `/api/salidas-equipos`

| Endpoint (Express)   | Función Original (Rust)      | Descripción                                |
| :------------------- | :--------------------------- | :----------------------------------------- |
| **GET** `/`          | `get_salidas_equipo`         | Historial de salidas registradas.          |
| **POST** `/`         | `registrar_salida_equipo_v2` | Registra una nueva salida.                 |
| **GET** `/check/:id` | `puede_registrar_salida_v2`  | Verifica si una OT puede registrar salida. |
| **GET** `/orden/:id` | `get_salida_by_orden`        | Busca salida asociada a una OT.            |

### Detalles de Endpoints

#### **POST** `/`

- **Body:**
  - `orden_trabajo_id`
  - `motivo_salida`
  - `usuario_id`
  - `observaciones`
- **Respuesta:** `SalidaEquipo` (Objeto creado).

#### **GET** `/check/:id`

- **Parámetros:** `id` (orden_trabajo_id).
- **Respuesta:** `{ puede: boolean, mensaje: string }`

**Campos de Respuesta (SalidaEquipo):**

- `orden_codigo`
- `equipo_nombre` (Marca + Modelo)
- `cliente_nombre`
- `motivo_salida`
- `fecha_salida`
- `usuario_nombre`
