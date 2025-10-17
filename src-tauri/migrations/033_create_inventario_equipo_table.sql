-- Crear tabla para inventario de equipos de la empresa
CREATE TABLE IF NOT EXISTS INVENTARIO_EQUIPO (
    inventario_equipo_id INT PRIMARY KEY AUTO_INCREMENT,
    equipo_codigo VARCHAR(32) UNIQUE NOT NULL,
    equipo_nombre VARCHAR(64) NOT NULL,
    equipo_marca VARCHAR(32),
    equipo_modelo VARCHAR(64),
    equipo_tipo SET('radio', 'antena', 'repetidor', 'herramienta', 'accesorio', 'otro') NOT NULL,
    equipo_descripcion TEXT,
    equipo_precio INT,
    equipo_stock INT DEFAULT 0,
    equipo_estado SET('disponible', 'en_uso', 'mantenimiento', 'fuera_servicio', 'prestado') DEFAULT 'disponible',
    equipo_ubicacion VARCHAR(256),
    fecha_adquisicion DATE,
    proveedor VARCHAR(128),
    numero_serie VARCHAR(64),
    garantia_vencimiento DATE,
    observaciones TEXT,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id)
);