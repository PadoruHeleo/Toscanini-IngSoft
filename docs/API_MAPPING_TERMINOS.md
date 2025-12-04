# Documentación de Mapeo API vs Rust (Términos y Condiciones)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `terminos.rs`.

## 📌 Base URL

Prefijo: **`/api/terminos`**

---

## 1. Gestión del Catálogo Maestro

| Endpoint (Express) | Función Original (Rust) | Descripción                                           |
| :----------------- | :---------------------- | :---------------------------------------------------- |
| **GET** `/`        | `get_terminos`          | Obtiene términos activos. Soporta filtro `?tipo=...`. |
| **GET** `/:id`     | `get_termino_by_id`     | Obtiene un término por ID.                            |
| **POST** `/`       | `create_termino`        | Crea un nuevo término en el catálogo.                 |
| **PUT** `/:id`     | `update_termino`        | Actualiza un término existente.                       |
| **POST** `/delete` | `delete_termino`        | Elimina un término (Soft Delete).                     |

### Detalles de Endpoints

#### **GET** `/`

- **Query Params:** `tipo` (Opcional: 'informe', 'cotizacion', 'ambos').
- **Respuesta:** `Array<Termino>`
  - `termino_id`, `termino_nombre`, `termino_descripcion`, `tipo_referencia`, `is_default`, `is_active`.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Termino` o `null`.

#### **POST** `/`

- **Body:**
  - `termino_nombre`
  - `termino_descripcion`
  - `tipo_referencia`
  - `is_default` (Booleano)
  - `created_by`
- **Respuesta:** `Termino` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `termino_nombre`, `termino_descripcion`, `tipo_referencia`, `is_default` (Opcionales)
  - `updated_by`
- **Respuesta:** `Termino` (Objeto actualizado).

#### **POST** `/delete`

- **Body:**
  - `termino_id`
  - `deleted_by`
- **Respuesta:** `{ success: true }`

---

## 2. Asociación a Documentos (Informes)

| Endpoint (Express)            | Función Original (Rust)       | Descripción                            |
| :---------------------------- | :---------------------------- | :------------------------------------- |
| **GET** `/informe/:informeId` | `get_terminos_by_informe`     | Lista términos aplicados a un informe. |
| **POST** `/informe`           | `add_termino_to_informe`      | Asocia un término a un informe.        |
| **POST** `/informe/remove`    | `remove_termino_from_informe` | Desasocia un término de un informe.    |

### Detalles de Endpoints

#### **GET** `/informe/:informeId`

- **Parámetros:** `informeId` (URL param).
- **Respuesta:** `Array<TerminoAsociado>`
  - `termino_id`, `informe_id`, `termino_desc`.

#### **POST** `/informe`

- **Body:**
  - `informe_id`
  - `termino_id`
  - `added_by`
- **Respuesta:** `{ success: true }`

---

## 3. Asociación a Documentos (Cotizaciones)

| Endpoint (Express)                  | Función Original (Rust)          | Descripción                                |
| :---------------------------------- | :------------------------------- | :----------------------------------------- |
| **GET** `/cotizacion/:cotizacionId` | `get_terminos_by_cotizacion`     | Lista términos aplicados a una cotización. |
| **POST** `/cotizacion`              | `add_termino_to_cotizacion`      | Asocia un término a una cotización.        |
| **POST** `/cotizacion/remove`       | `remove_termino_from_cotizacion` | Desasocia un término de una cotización.    |

### Detalles de Endpoints

#### **GET** `/cotizacion/:cotizacionId`

- **Parámetros:** `cotizacionId` (URL param).
- **Respuesta:** `Array<TerminoAsociado>`
  - `termino_id`, `cotizacion_id`, `termino_desc`.

#### **POST** `/cotizacion`

- **Body:**
  - `cotizacion_id`
  - `termino_id`
  - `added_by`
- **Respuesta:** `{ success: true }`
