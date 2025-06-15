-- Agregar nuevos campos al informe para compatibilidad con el frontend
ALTER TABLE INFORME 
ADD COLUMN diagnostico TEXT,
ADD COLUMN recomendaciones TEXT,
ADD COLUMN solucion_aplicada TEXT,
ADD COLUMN tecnico_responsable VARCHAR(255);
