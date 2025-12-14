# API Mapping - PDF Generation Endpoints

Este documento detalla los endpoints diseñados específicamente para proveer los datos estructurados necesarios para la generación de documentos PDF (Cotizaciones, Informes Técnicos y Órdenes de Trabajo).

Estos endpoints devuelven un objeto JSON con toda la información consolidada (incluyendo relaciones con Cliente, Equipo, Piezas, Términos, etc.), lista para ser consumida por el generador de reportes.

## 1. Cotizaciones

### Obtener Datos para PDF de Cotización

Recupera todos los datos necesarios para generar el documento de cotización.

- **Método:** `GET`
- **Ruta:** `/api/cotizaciones/:id/pdf-data`
- **Parámetros:**
  - `id` (path): ID numérico de la cotización.

**Ejemplo de Respuesta (JSON):**

```json
{
  "cotizacion_codigo": "COT-2024-001",
  "fecha": "2024-05-10T14:30:00.000Z",
  "empresa": {
    "nombre": "Toscanini",
    "direccion": "Dirección de la empresa",
    "telefono": "+56 9 1234 5678",
    "email": "contacto@toscanini.cl",
    "website": "www.toscanini.cl"
  },
  "cliente": {
    "nombre": "Juan Pérez",
    "email": "juan@example.com",
    "telefono": "+56911112222",
    "direccion": "Calle Falsa 123"
  },
  "equipo": {
    "marca": "Yaesu",
    "modelo": "FT-857",
    "tipo": "Radio",
    "numero_serie": "ABC12345",
    "ubicacion": "Estante A1"
  },
  "informe_tecnico": "Descripción detallada del informe técnico asociado...",
  "costo_revision": 15000,
  "costo_reparacion": 45000,
  "costo_total": 60000,
  "piezas": [
    {
      "nombre": "Transistor Final",
      "marca": "Mitsubishi",
      "cantidad": 1,
      "precio_unitario": 20000,
      "subtotal": 20000
    }
  ],
  "is_aprobada": false,
  "orden_codigo": "OT-2024-045",
  "terminos_condiciones": [
    {
      "nombre": "Garantía",
      "descripcion": "3 meses de garantía sobre reparación..."
    }
  ]
}
```

---

## 2. Informes Técnicos

### Obtener Datos para PDF de Informe Técnico

Recupera los datos del informe técnico final, incluyendo diagnóstico, solución, piezas utilizadas y términos aplicables.

- **Método:** `GET`
- **Ruta:** `/api/informes/:id/pdf-data`
- **Parámetros:**
  - `id` (path): ID numérico del informe.

**Ejemplo de Respuesta (JSON):**

```json
{
  "informe_codigo": "INF-2024-012",
  "fecha": "2024-05-12T10:00:00.000Z",
  "empresa": { ... },
  "cliente": { ... },
  "equipo": { ... },
  "diagnostico": "Falla en etapa de potencia debido a sobrevoltaje...",
  "recomendaciones": "Revisar fuente de poder...",
  "solucion_aplicada": "Reemplazo de finales y ajuste de bias.",
  "tecnico_responsable": "Pedro Ingeniero",
  "piezas": [ ... ],
  "orden_codigo": "OT-2024-045",
  "tiene_garantia": true,
  "terminos_condiciones": [ ... ]
}
```

---

## 3. Órdenes de Trabajo (Ingreso)

### Obtener Datos para PDF de Ingreso (Orden de Trabajo)

Recupera los datos de la ficha de ingreso del equipo.

- **Método:** `GET`
- **Ruta:** `/api/ordenes-trabajo/:id/pdf-data`
- **Parámetros:**
  - `id` (path): ID numérico de la orden de trabajo.

**Ejemplo de Respuesta (JSON):**

```json
{
  "orden_codigo": "OT-2024-050",
  "fecha": "2024-06-01T09:15:00.000Z",
  "fecha_finalizacion": null,
  "empresa": { ... },
  "cliente": { ... },
  "equipo": { ... },
  "orden_desc": "Equipo no enciende, cliente reporta olor a quemado.",
  "pre_informe": "Consumo excesivo en fuente al conectar.",
  "prioridad": "Alta",
  "estado": "Pendiente",
  "has_garantia": false,
  "creador_nombre": "Recepcionista María",
  "cotizacion_codigo": null,
  "informe_codigo": null
}
```
