-- Agregar nuevos campos al informe para compatibilidad con el frontend
ALTER TABLE INFORME 
ADD COLUMN IF NOT EXISTS diagnostico TEXT,
ADD COLUMN IF NOT EXISTS recomendaciones TEXT,
ADD COLUMN IF NOT EXISTS solucion_aplicada TEXT,
ADD COLUMN IF NOT EXISTS tecnico_responsable VARCHAR(255);
