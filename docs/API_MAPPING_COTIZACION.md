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

---

## 2. Gestión de Piezas (Items)

| Endpoint (Express)    | Función Original (Rust)    | Descripción                                                        |
| :-------------------- | :------------------------- | :----------------------------------------------------------------- |
| **GET** `/:id/piezas` | `get_piezas_cotizacion`    | Lista piezas asociadas a la cotización.                            |
| **PUT** `/:id/piezas` | `update_cotizacion_piezas` | Reemplaza todas las piezas (Borra y crea nuevas). Usa transacción. |

---

## 3. Acciones de Negocio

| Endpoint (Express)   | Función Original (Rust) | Descripción                                                      |
| :------------------- | :---------------------- | :--------------------------------------------------------------- |
| **POST** `/aprobar`  | `aprobar_cotizacion`    | Marca `is_aprobada = true`.                                      |
| **POST** `/duplicar` | `duplicate_cotizacion`  | Crea una copia exacta (piezas + términos) para un nuevo informe. |

---

## 4. Consultas Relacionales y PDF

| Endpoint (Express)      | Función Original (Rust)       | Descripción                                               |
| :---------------------- | :---------------------------- | :-------------------------------------------------------- |
| **GET** `/cliente/:id`  | `get_cotizaciones_by_cliente` | Historial de cotizaciones de un cliente.                  |
| **GET** `/equipo/:id`   | `get_cotizaciones_by_equipo`  | Historial de cotizaciones de un equipo.                   |
| **GET** `/:id/pdf-data` | `get_cotizacion_pdf_data`     | Datos completos (Cliente+Equipo+Montos) para generar PDF. |

---

## ⚠️ Notas de Implementación

1. **Transacciones:** Las operaciones críticas como `updateCotizacionPiezas` (que borra e inserta) y `createCotizacion` (que inserta cabecera y términos) usan transacciones de base de datos (`connection.beginTransaction`) para asegurar la integridad de datos.
2. **Cálculos:** La suma de `costo_total` se recalcula automáticamente en el servidor al actualizar `costo_revision` o `costo_reparacion`.

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

---

## 7. Salidas de Equipos

**Prefijo:** `/api/salidas-equipos`

| Endpoint (Express)   | Función Original (Rust)      | Descripción                                |
| :------------------- | :--------------------------- | :----------------------------------------- |
| **GET** `/`          | `get_salidas_equipo`         | Historial de salidas registradas.          |
| **POST** `/`         | `registrar_salida_equipo_v2` | Registra una nueva salida.                 |
| **GET** `/check/:id` | `puede_registrar_salida_v2`  | Verifica si una OT puede registrar salida. |
| **GET** `/orden/:id` | `get_salida_by_orden`        | Busca salida asociada a una OT.            |
