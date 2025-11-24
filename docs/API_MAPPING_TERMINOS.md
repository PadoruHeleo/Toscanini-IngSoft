# Documentación de Mapeo API vs Rust (Términos y Condiciones)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `terminos_condiciones.rs`.

## 📌 Base URL
Prefijo: **`/api/terminos`**

---

## 1. Catálogo Maestro (Configuración)

Gestión de la tabla `TERMINOS_CONDICIONES` (plantillas).

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/` | `get_terminos_condiciones` | Lista términos activos. Filtro opcional `?tipo=`. |
| **GET** `/:id` | `get_termino_by_id` | Detalle de un término. |
| **POST** `/` | `create_termino` | Crea una nueva plantilla de término. |
| **PUT** `/:id` | `update_termino` | Modifica el texto o configuración. |
| **POST** `/delete` | `delete_termino` | Soft delete (`is_active = 0`). |

---

## 2. Asociación a Informes

Gestión de la tabla `TERMINOS_INFORME`.

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/informe/:id` | `get_terminos_by_informe` | Lista términos aplicados a un informe. |
| **POST** `/informe/add` | `add_termino_to_informe` | Agrega texto de término a un informe. |
| **POST** `/informe/remove` | `remove_termino_from_informe` | Elimina un término de un informe. |

---

## 3. Asociación a Cotizaciones

Gestión de la tabla `TERMINOS_COTIZACION`.

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/cotizacion/:id` | `get_terminos_by_cotizacion` | Lista términos aplicados a una cotización. |
| **POST** `/cotizacion/add` | `add_termino_to_cotizacion` | Agrega texto de término a una cotización. |
| **POST** `/cotizacion/remove` | `remove_termino_from_cotizacion` | Elimina un término de una cotización. |

---

## ⚠️ Notas Técnicas

1. **Snapshot de Texto:** Al asociar un término a un documento (`add`), se copia el texto (`termino_desc`) en lugar de solo referenciar el ID. Esto asegura que si el término maestro cambia en el futuro, los documentos históricos conserven el texto legal original con el que fueron emitidos.
2. **Default:** La aplicación automática de términos marcados como `is_default` ocurre en el backend dentro de los controladores `createCotizacion` y `createInforme` (módulos respectivos), no en este controlador.