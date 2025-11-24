# Documentación de Mapeo API vs Rust (Órdenes de Trabajo)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `ordenes_trabajo.rs`.

## 📌 Base URL
Prefijo: **`/api/ordenes`**

---

## 1. CRUD Orden

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/` | `get_ordenes_trabajo` | Listado completo de órdenes activas. |
| **GET** `/:id` | `get_orden_trabajo_by_id` | Detalle de una orden. |
| **POST** `/` | `create_orden_trabajo` | Crea orden. **Genera código** `OT-YYYY-NNN`. Valida equipo activo. |
| **PUT** `/:id` | `update_orden_trabajo` | Actualiza datos (descripción, prioridad, garantía). |
| **POST** `/delete` | `delete_orden_trabajo` | Borrado lógico. |

---

## 2. Gestión de Estado

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **PATCH** `/:id/estado` | `update_orden_trabajo_estado` | Actualiza el estado. Si es 'Entregado', fija `finished_at`. |

---

## 3. Asociaciones

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **POST** `/associate/cotizacion` | `associate_cotizacion_to_orden` | Vincula una ID de cotización a la orden. |
| **POST** `/associate/informe` | `associate_informe_to_orden` | Vincula una ID de informe a la orden. |

---

## 4. Consultas Relacionales y PDF

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/cliente/:id` | `get_ordenes_trabajo_by_cliente` | Historial de órdenes de un cliente. |
| **GET** `/equipo/:id` | `get_ordenes_trabajo_by_equipo` | Historial de órdenes de un equipo. |
| **GET** `/:id/pdf-data` | `get_orden_trabajo_pdf_data` | Datos completos para generar Ficha de Ingreso (PDF). |

---

## ⚠️ Notas Técnicas

1. **Validación de Negocio:** La API impide crear una nueva orden (`POST /`) si el equipo seleccionado ya tiene una orden en estado pendiente o en proceso (`activeOrders`). Esto evita duplicidad operativa.
2. **Generación de Código:** El código `OT-202X-XXX` se genera calculando el conteo de órdenes del año en curso dentro de una transacción para evitar colisiones.