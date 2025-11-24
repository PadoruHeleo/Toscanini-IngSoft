# Documentación de Mapeo API vs Rust (Clientes)

Este documento detalla la relación entre los endpoints de la API REST (Express) y las funciones originales implementadas en el backend de Rust (`clientes.rs`).

La nueva API centraliza la lógica de validación de duplicados, verificación de dependencias (equipos asociados) y el registro de auditoría.

## 📌 Base URL

Todas las rutas listadas a continuación tienen el prefijo configurado en tu router: `/api/clientes`

---

## 1. CRUD y Consultas Básicas

Operaciones fundamentales sobre la entidad Cliente.

| Método   | Endpoint (Express) | Función Original (Rust) | Descripción                                                                |
| :------- | :----------------- | :---------------------- | :------------------------------------------------------------------------- |
| **GET**  | `/`                | `get_clientes`          | Obtiene todos los clientes activos ordenados por nombre.                   |
| **GET**  | `/:id`             | `get_cliente_by_id`     | Busca un cliente específico por su ID.                                     |
| **GET**  | `/rut/:rut`        | `get_cliente_by_rut`    | Busca un cliente específico por su RUT.                                    |
| **POST** | `/`                | `create_cliente`        | Crea un cliente nuevo. Valida que el RUT no exista previamente.            |
| **PUT**  | `/:id`             | `update_cliente`        | Actualiza datos parciales. Valida que el RUT no pertenezca a otro cliente. |

---

## 2. Gestión de Estado (Borrado Lógico)

Manejo de la activación e inactivación de clientes.

| Método   | Endpoint (Express) | Función Original (Rust) | Descripción                                                                                                    |
| :------- | :----------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------- |
| **POST** | `/delete`          | `delete_cliente`        | Inactiva un cliente. **Requiere Body** con `motivo`. Valida que no tenga equipos asociados antes de inactivar. |
| **POST** | `/reactivate`      | `reactivate_cliente`    | Reactiva un cliente previamente eliminado (soft delete).                                                       |

> **Nota sobre el Delete:** Se usa `POST` en lugar de `DELETE` para la ruta `/delete` porque la operación requiere un cuerpo JSON (`body`) con el `motivo` de la eliminación y el ID del usuario que elimina (`deleted_by`), lo cual es más estándar enviar en un POST.

---

## 3. Búsqueda y Filtros Avanzados

Herramientas para listados complejos y buscadores.

| Método   | Endpoint (Express) | Función Original (Rust)  | Descripción                                                                                        |
| :------- | :----------------- | :----------------------- | :------------------------------------------------------------------------------------------------- |
| **GET**  | `/search/query`    | `search_clientes`        | Búsqueda simple de texto (nombre, rut, correo). Usa query param `?term=...`.                       |
| **POST** | `/filter`          | `get_clientes_filtrados` | Filtro maestro unificado. Recibe un JSON con fechas, estados, arrays de ruts/correos, ciudad, etc. |

---

## 4. Listas Auxiliares (Filtros)

Endpoints que devuelven listas de valores únicos (`DISTINCT`) para poblar los selectores de filtros en el Frontend.

| Método  | Endpoint (Express) | Función Original (Rust) | Descripción                                     |
| :------ | :----------------- | :---------------------- | :---------------------------------------------- |
| **GET** | `/list/ruts`       | `get_ruts_clientes`     | Lista de todos los RUTs únicos registrados.     |
| **GET** | `/list/emails`     | `get_correos_clientes`  | Lista de todos los correos únicos registrados.  |
| **GET** | `/list/ciudades`   | `get_ciudades_clientes` | Lista de todas las direcciones/ciudades únicas. |

---

## ⚠️ Diferencias de Implementación y Notas

1. **Tipos de Datos:**

   - **Rust/SQL:** El campo `is_active` suele ser `1` o `0`.
   - **API Node:** Convierte automáticamente `is_active` a `true` o `false` (Booleano) en la respuesta JSON.

2. **Auditoría Automática:**

   - Todas las rutas de escritura (`create`, `update`, `delete`, `reactivate`) ejecutan internamente `logAction` para insertar en la tabla `AUDIT_LOG`, replicando la lógica exacta de Rust (incluyendo el registro de valores previos y nuevos en las actualizaciones).

3. **Validaciones:**

   - La API maneja internamente las validaciones de negocio:
     - No crear/actualizar si el RUT ya existe.
     - No eliminar si el cliente tiene equipos registrados (devuelve error 400).

4. **Funciones no migradas como endpoints directos:**
   - `get_clientes_by_created_by`: Esta funcionalidad se absorbe dentro de los filtros generales o no se expuso directamente por redundancia.
   - `count_clientes`: No se expuso un endpoint dedicado; se puede obtener la longitud del array en `get_clientes` o usar los metadatos de la respuesta si se implementa paginación futura.
   - `get_clientes_with_pagination`: La lógica de paginación se integró parcialmente en la estructura, pero el endpoint principal `/filter` permite flexibilidad similar.
