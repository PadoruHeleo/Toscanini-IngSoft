# Documentación de Mapeo API vs Rust (Usuarios)

Este documento detalla la relación entre los endpoints de la API REST (Express) y las funciones originales implementadas en el backend de Rust (`users.rs`).

El objetivo de esta API es actuar como una capa de acceso a datos, delegando la lógica de negocio compleja a la aplicación cliente, pero manteniendo la integridad de la base de datos y la auditoría.

## 📌 Base URL

Todas las rutas listadas a continuación tienen el prefijo configurado en tu `index.js` (ejemplo: `/api/usuarios`).

---

## 1. CRUD de Usuarios

Gestión básica de la entidad Usuario.

| Método     | Endpoint (Express) | Función Original (Rust) | Descripción                                      |
| :--------- | :----------------- | :---------------------- | :----------------------------------------------- |
| **GET**    | `/`                | `get_usuarios`          | Obtiene la lista de todos los usuarios.          |
| **GET**    | `/:id`             | `get_usuario_by_id`     | Obtiene un usuario específico por su ID.         |
| **GET**    | `/rut/:rut`        | `get_usuario_by_rut`    | Obtiene un usuario buscando por su RUT.          |
| **POST**   | `/`                | `create_usuario`        | Crea un nuevo usuario y registra auditoría.      |
| **PUT**    | `/:id`             | `update_usuario`        | Actualiza datos parciales de un usuario.         |
| **DELETE** | `/:id`             | `delete_usuario`        | Realiza un borrado lógico (`is_active = false`). |

---

## 2. Autenticación y Sesiones

Manejo de acceso y validación de tokens.

| Método   | Endpoint (Express)  | Función Original (Rust)    | Descripción                                      |
| :------- | :------------------ | :------------------------- | :----------------------------------------------- |
| **POST** | `/login`            | `authenticate_usuario`     | Verifica credenciales y genera `session_token`.  |
| **POST** | `/validate-session` | `validate_session`         | Verifica si un token es válido y no ha expirado. |
| **POST** | `/cleanup/sessions` | `cleanup_expired_sessions` | Limpia tokens de sesiones vencidas.              |

---

## 3. Validaciones

Endpoints rápidos para validaciones en tiempo real (formularios).

| Método  | Endpoint (Express)     | Función Original (Rust) | Descripción                                  |
| :------ | :--------------------- | :---------------------- | :------------------------------------------- |
| **GET** | `/rut/:rut/exists`     | `verify_rut_in_use`     | Retorna `true/false` si el RUT ya existe.    |
| **GET** | `/email/:email/exists` | `verify_email_in_use`   | Retorna `true/false` si el correo ya existe. |

---

## 4. Gestión de Cuenta

Modificaciones sensibles del perfil de usuario.

| Método  | Endpoint (Express) | Función Original (Rust) | Descripción                                    |
| :------ | :----------------- | :---------------------- | :--------------------------------------------- |
| **PUT** | `/phone/:id`       | `change_user_phone`     | Actualiza el teléfono.                         |
| **PUT** | `/:id/email`       | `change_user_email`     | Actualiza el correo (valida unicidad).         |
| **PUT** | `/:id/password`    | `change_user_password`  | Actualiza la contraseña (recibe hash o texto). |

---

## 5. Recuperación de Contraseña

Flujo completo de "Olvidé mi contraseña".

| Método     | Endpoint (Express)        | Función Original (Rust)       | Descripción                                        |
| :--------- | :------------------------ | :---------------------------- | :------------------------------------------------- |
| **POST**   | `/password-reset/request` | `request_password_reset`      | Genera código de recuperación (simulado/guardado). |
| **POST**   | `/password-reset/verify`  | `verify_reset_code`           | Valida si el código ingresado es correcto.         |
| **POST**   | `/password-reset/confirm` | `reset_password_with_code`    | Cambia la contraseña usando el código validado.    |
| **DELETE** | `/cleanup/reset-codes`    | `cleanup_expired_reset_codes` | Elimina códigos de recuperación viejos.            |

---

## 6. Utilidades y Configuración

Herramientas administrativas y de sistema.

| Método   | Endpoint (Express)   | Función Original (Rust)     | Descripción                                             |
| :------- | :------------------- | :-------------------------- | :------------------------------------------------------ |
| **GET**  | `/admin-tech-emails` | `get_admin_and_tech_emails` | Lista correos de admins y técnicos para notificaciones. |
| **POST** | `/setup/admin`       | `create_admin_user`         | Script inicial para crear el primer admin si no existe. |

---

## ⚠️ Notas de Implementación

1. **Contraseñas:** A diferencia de Rust, donde `hash_password` ocurre en el backend, esta API está diseñada para recibir las contraseñas ya procesadas o delegar la encriptación, actuando principalmente como interfaz de base de datos.
2. **Auditoría:** Todas las acciones de escritura (`POST`, `PUT`, `DELETE`) ejecutan automáticamente la función `logAction` (equivalente a `log_action` de Rust) para mantener la trazabilidad en la tabla `AUDIT_LOG`.
3. **Tipos de Datos:** Se asegura que los booleanos (`is_active`, `used`) se devuelvan siempre como `true/false` y no como `1/0` para mantener compatibilidad con el frontend existente.
