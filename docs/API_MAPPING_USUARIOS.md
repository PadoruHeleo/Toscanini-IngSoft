# Documentación de Mapeo API vs Rust (Usuarios)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `users.rs`.

## 📌 Base URL

Prefijo: **`/api/usuarios`**

---

## 1. CRUD Usuarios

| Endpoint (Express)  | Función Original (Rust) | Descripción                       |
| :------------------ | :---------------------- | :-------------------------------- |
| **GET** `/`         | `get_usuarios`          | Obtiene todos los usuarios.       |
| **GET** `/:id`      | `get_usuario_by_id`     | Obtiene un usuario por ID.        |
| **GET** `/rut/:rut` | `get_usuario_by_rut`    | Obtiene un usuario por RUT.       |
| **POST** `/`        | `create_usuario`        | Crea un nuevo usuario.            |
| **PUT** `/:id`      | `update_usuario`        | Actualiza datos de un usuario.    |
| **DELETE** `/:id`   | `delete_usuario`        | Elimina un usuario (Soft Delete). |

### Detalles de Endpoints

#### **GET** `/`

- **Parámetros:** Ninguno.
- **Respuesta:** `Array<Usuario>`
  - `usuario_id`, `usuario_rut`, `usuario_nombre`, `usuario_correo`, `usuario_rol`, `is_active`, `last_login_at`.

#### **GET** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `Usuario` o `null`.

#### **POST** `/`

- **Body:**
  - `usuario_rut`
  - `usuario_nombre`
  - `usuario_correo`
  - `usuario_contrasena` (Texto plano, se hashea en servidor)
  - `usuario_telefono`
  - `usuario_rol` ('admin', 'tecnico', 'recepcion')
- **Respuesta:** `Usuario` (Objeto creado).

#### **PUT** `/:id`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `usuario_rut`, `usuario_nombre`, `usuario_correo`, `usuario_telefono`, `usuario_rol` (Opcionales)
- **Respuesta:** `Usuario` (Objeto actualizado).

#### **DELETE** `/:id`

- **Parámetros:** `id` (URL param).
- **Respuesta:** `{ success: true, message: "..." }`

---

## 2. Autenticación y Sesión

| Endpoint (Express)           | Función Original (Rust) | Descripción                     |
| :--------------------------- | :---------------------- | :------------------------------ |
| **POST** `/login`            | `authenticate_usuario`  | Inicia sesión y devuelve token. |
| **POST** `/validate-session` | `validate_session`      | Valida un token de sesión.      |

### Detalles de Endpoints

#### **POST** `/login`

- **Body:**
  - `usuario_correo`
  - `usuario_contrasena`
- **Respuesta:** `Usuario` (Incluye `session_token` y `session_expires_at`).

#### **POST** `/validate-session`

- **Body:**
  - `session_token`
- **Respuesta:** `Usuario` (Si es válido) o `null`.

---

## 3. Gestión de Cuenta y Utilidades

| Endpoint (Express)             | Función Original (Rust)     | Descripción                         |
| :----------------------------- | :-------------------------- | :---------------------------------- |
| **GET** `/rut/:rut/exists`     | `verify_rut_in_use`         | Verifica si un RUT ya existe.       |
| **GET** `/email/:email/exists` | `verify_email_in_use`       | Verifica si un Email ya existe.     |
| **PUT** `/phone/:id`           | `change_user_phone`         | Cambia el teléfono de un usuario.   |
| **PUT** `/:id/email`           | `change_user_email`         | Cambia el email de un usuario.      |
| **PUT** `/:id/password`        | `change_user_password`      | Cambia la contraseña de un usuario. |
| **GET** `/admin-tech-emails`   | `get_admin_and_tech_emails` | Lista emails de admins y técnicos.  |

### Detalles de Endpoints

#### **GET** `/rut/:rut/exists`

- **Parámetros:** `rut` (URL param).
- **Respuesta:** `{ exists: boolean }`

#### **PUT** `/:id/password`

- **Parámetros:** `id` (URL param).
- **Body:**
  - `new_password` (Ya hasheada o texto plano según implementación frontend, API espera el valor final a guardar o lo hashea si es texto plano. _Nota: Controller actual guarda directo, asumir hasheado o ajustar controller_).
- **Respuesta:** `{ success: true }`

---

## 4. Recuperación de Contraseña

| Endpoint (Express)                 | Función Original (Rust)       | Descripción                      |
| :--------------------------------- | :---------------------------- | :------------------------------- |
| **POST** `/password-reset/request` | `request_password_reset`      | Solicita código de recuperación. |
| **POST** `/password-reset/verify`  | `verify_reset_code`           | Verifica código de recuperación. |
| **POST** `/password-reset/confirm` | `reset_password_with_code`    | Cambia contraseña usando código. |
| **DELETE** `/cleanup/reset-codes`  | `cleanup_expired_reset_codes` | Limpia códigos expirados.        |

### Detalles de Endpoints

#### **POST** `/password-reset/request`

- **Body:** `usuario_correo`.
- **Respuesta:** `{ success: true, message: "..." }`

#### **POST** `/password-reset/confirm`

- **Body:** `usuario_correo`, `reset_code`, `new_password`.
- **Respuesta:** `{ success: true }`
