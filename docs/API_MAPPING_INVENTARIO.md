# API Mapping - Gestión de Inventario (Equipos y Piezas)

Este documento detalla los endpoints relacionados con la gestión de inventario de equipos (para venta/baja) y piezas (repuestos), así como el registro de salidas de equipos de taller.

Estos endpoints permiten mantener el control del stock, crear nuevos ítems y gestionar el ciclo de vida de los equipos que salen del taller.

## 1. Gestión de Piezas (Repuestos)

Endpoints para administrar el catálogo de repuestos y componentes disponibles.

- **Base URL:** `/api/piezas`
- **Controlador:** `cotizacionController.js`

| Método   | Ruta   | Descripción                             | Parámetros Path | Body (JSON)           |
| :------- | :----- | :-------------------------------------- | :-------------- | :-------------------- |
| `GET`    | `/`    | Listar todas las piezas.                | -               | -                     |
| `GET`    | `/:id` | Obtener detalles de una pieza esp.      | `id`            | -                     |
| `POST`   | `/`    | Crear una nueva pieza.                  | -               | Ver **Payload Pieza** |
| `PUT`    | `/:id` | Actualizar datos de una pieza.          | `id`            | Ver **Payload Pieza** |
| `DELETE` | `/:id` | Eliminar una pieza (si no está en uso). | `id`            | -                     |

### Payload Pieza (POST/PUT)

```json
{
  "pieza_nombre": "string (req)",
  "pieza_marca": "string",
  "pieza_desc": "string",
  "pieza_precio": "number",
  "pieza_stock": "int (default 0)"
}
```

---

## 2. Inventario de Equipos (Venta/Interno)

Endpoints para gestionar equipos que son parte del inventario interno (no equipos de clientes para reparación), como equipos para la venta o herramientas.

- **Base URL:** `/api/inventario-equipos`
- **Controlador:** `cotizacionController.js`

| Método   | Ruta         | Descripción                          | Parámetros Path | Body (JSON)                                                         |
| :------- | :----------- | :----------------------------------- | :-------------- | :------------------------------------------------------------------ |
| `GET`    | `/`          | Listar inventario de equipos.        | -               | -                                                                   |
| `POST`   | `/`          | Ingresar nuevo equipo al inventario. | -               | Ver **Payload Inventario** (Incluye `created_by`)                   |
| `PUT`    | `/:id`       | Actualizar datos de un equipo.       | `id`            | Ver **Payload Inventario**                                          |
| `DELETE` | `/:id`       | Eliminar equipo del inventario.      | `id`            | -                                                                   |
| `POST`   | `/:id/stock` | Ajustar stock (sumar/restar).        | `id`            | `{ "cantidad": int, "tipo": "add"\|"subtract", "updated_by": int }` |

### Payload Inventario (POST/PUT)

```json
{
  "equipo_codigo": "string (opcional, auto-gen en create)",
  "equipo_nombre": "string",
  "equipo_marca": "string",
  "equipo_modelo": "string",
  "equipo_tipo": "string (radio|antena|repetidor|herramienta|accesorio|otro)",
  "equipo_descripcion": "string",
  "equipo_precio": "number",
  "equipo_stock": "int",
  "equipo_estado": "string",
  "equipo_ubicacion": "string",
  "fecha_adquisicion": "date (YYYY-MM-DD)",
  "proveedor": "string",
  "numero_serie": "string",
  "garantia_vencimiento": "date (YYYY-MM-DD)",
  "observaciones": "string",
  "created_by": "int (solo POST)"
}
```

> **Nota:** Existe un endpoint redundante en `equipoRoutes.js` (`POST /api/equipos/inventario/:id/stock`) que apunta a la misma lógica de stock. Se recomienda usar `/api/inventario-equipos/:id/stock`.

---

## 3. Salidas de Equipos (Taller)

Endpoints para gestionar el proceso de entrega o retiro de equipos que estaban en reparación (Orden de Trabajo).

- **Base URL:** `/api/salidas-equipos`
- **Controlador:** `cotizacionController.js`

| Método | Ruta         | Descripción                                  | Parámetros Path | Body (JSON)                                                                                        |
| :----- | :----------- | :------------------------------------------- | :-------------- | :------------------------------------------------------------------------------------------------- |
| `GET`  | `/`          | Listar historial de salidas.                 | -               | -                                                                                                  |
| `POST` | `/`          | Registrar salida (entrega) de equipo.        | -               | `{ "orden_trabajo_id": int, "motivo_salida": string, "usuario_id": int, "observaciones": string }` |
| `GET`  | `/check/:id` | Verificar si una OT puede registrar salida.  | `id` (orden_id) | -                                                                                                  |
| `GET`  | `/orden/:id` | Obtener info de salida de una OT específica. | `id` (orden_id) | -                                                                                                  |

### Respuesta `/check/:id`

```json
{
  "puede": boolean,
  "mensaje": "string"
}
```

Estados válidos para salida: `recibido`, `cotizacion_enviada`, `aprobacion_pendiente`, `en_reparacion`, `espera_de_retiro`, `cotizacion_rechazada`.
Si ya existe salida, retorna `false`.
