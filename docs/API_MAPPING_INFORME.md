# Documentación de Mapeo API vs Rust (Informes Técnicos)

Este documento detalla la relación entre los endpoints de la API REST y las funciones originales de `informe.rs`.

## 📌 Base URL
Prefijo: **`/api/informes`**

---

## 1. CRUD Informe

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/` | `get_informes` | Lista informes activos. |
| **GET** `/:id` | `get_informe_by_id` | Detalle de un informe. |
| **POST** `/` | `create_informe` | Crea informe y copia términos por defecto (`is_borrador=1`). |
| **PUT** `/:id` | `update_informe` | Actualiza diagnóstico, recomendaciones, etc. |
| **POST** `/delete` | `delete_informe` | Soft delete. |

---

## 2. Piezas y Estado

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **GET** `/:id/piezas` | `get_piezas_informe` | Lista piezas usadas en el informe. |
| **PUT** `/:id/piezas` | `update_informe_piezas` | Actualiza listado de piezas (Transaction). |
| **POST** `/finalizar` | `finalizar_informe` (aprox) | Cambia estado `is_borrador` a `false`. |

---

## 3. Acciones y Relaciones

| Endpoint (Express) | Función Original (Rust) | Descripción |
| :--- | :--- | :--- |
| **POST** `/registrar-envio` | `send_informe_to_client` | Registra en auditoría que se envió el informe. |
| **GET** `/cliente/:id` | `get_informes_by_cliente` | Historial por cliente. |
| **GET** `/equipo/:id` | `get_informes_by_equipo` | Historial por equipo. |
| **GET** `/:id/pdf-data` | `get_informe_pdf_data` | Datos completos para generar el PDF. |

---

## ⚠️ Notas Técnicas

1. **Envío de Emails:** La función original `send_informe_to_client` en Rust enviaba el correo directamente. El endpoint `/registrar-envio` implementado aquí solo registra la acción en la base de datos (log). Para enviar el correo real, deberías implementar `nodemailer` dentro del controlador `registrarEnvioEmail` o mantener el envío en el cliente (sidecar) y solo llamar a la API para el registro.
2. **Transacciones:** Al igual que en Cotizaciones, la creación de informes y la actualización de piezas usan transacciones SQL para garantizar la consistencia.