import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import { FiltrarLogsPorEntidad } from "./FiltrarLogsPorEntidad";
import { FiltrarLogsPorAccion } from "./FiltrarLogsPorAccion";

interface AuditLogWithUser {
  log_id: number;
  log_accion: string | null;
  log_usuario_id: number | null;
  log_entidad_tabla: string | null;
  log_entidad_id: number | null;
  log_prev_v: string | null;
  log_new_v: string | null;
  created_at: string | null;
  usuario_nombre: string | null;
  usuario_correo: string | null;
}

interface Props {
  onFiltrar: (logs: AuditLogWithUser[]) => void;
  searchTerm?: string;
  onClearSearch?: () => void;
}

export function UnificarFiltrosLogs({
  onFiltrar,
  searchTerm,
  onClearSearch,
}: Props) {
  const filtrosIniciales = {
    entidad_tabla: null as string[] | null,
    accion: null as string[] | null,
    search: null as string | null,
  };

  const [filtros, setFiltros] = useState(filtrosIniciales);
  const [resetKey, setResetKey] = useState(0);

  // Sincronizar searchTerm con filtros de forma más eficiente
  useEffect(() => {
    const searchValue = searchTerm?.trim() || null;

    // Solo actualizar si realmente cambió para evitar re-renders innecesarios
    setFiltros((prev) => {
      if (prev.search !== searchValue) {
        return { ...prev, search: searchValue };
      }
      return prev;
    });
  }, [searchTerm]);

  // Función para actualizar filtros (sin afectar search)
  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({
      ...prev,
      ...nuevoFiltro,
    }));
  };

  // Aplicar filtros al backend
  const aplicarFiltros = async () => {
    try {
      console.log("🔍 Aplicando filtros de logs:", filtros);

      // Si no hay ningún filtro activo, obtener todos los logs
      const hayFiltrosActivos =
        (filtros.entidad_tabla !== null && filtros.entidad_tabla.length > 0) ||
        (filtros.accion !== null && filtros.accion.length > 0) ||
        filtros.search !== null;

      let logs: AuditLogWithUser[];

      if (!hayFiltrosActivos) {
        // Sin filtros, obtener todos los logs
        logs = await invoke<AuditLogWithUser[]>("get_audit_logs", {
          filters: null,
        });
      } else {
        // Con filtros, usar el endpoint de filtrado
        const filtrosParaBackend = {
          entidad_tabla: filtros.entidad_tabla,
          accion: filtros.accion,
          search: filtros.search,
          limit: 100,
        };

        console.log("📤 Enviando al backend:", filtrosParaBackend);
        logs = await invoke<AuditLogWithUser[]>("get_audit_logs", {
          filters: filtrosParaBackend,
        });
      }

      console.log("📨 Logs recibidos:", logs.length);
      onFiltrar(logs);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
      // En caso de error, intentar cargar todos los logs
      try {
        const logsBackup = await invoke<AuditLogWithUser[]>("get_audit_logs", {
          filters: null,
        });
        onFiltrar(logsBackup);
      } catch (backupErr) {
        console.error("❌ Error en carga de respaldo:", backupErr);
        onFiltrar([]);
      }
    }
  };

  // Aplicar filtros cuando cambien
  useEffect(() => {
    aplicarFiltros();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [filtros]);

  // Verificar si hay filtros activos (incluyendo búsqueda)
  const hayFiltrosActivos =
    (filtros.entidad_tabla !== null && filtros.entidad_tabla.length > 0) ||
    (filtros.accion !== null && filtros.accion.length > 0) ||
    filtros.search !== null;

  // Limpiar todos los filtros (incluyendo búsqueda)
  const limpiarFiltros = () => {
    setFiltros(filtrosIniciales);
    setResetKey((prev) => prev + 1);

    // Limpiar la búsqueda en el componente padre
    if (onClearSearch) {
      onClearSearch();
    }
  };

  return (
    <div className="flex gap-2 flex-wrap items-center">
      <FiltrarLogsPorEntidad
        resetKey={resetKey}
        onChange={(entidades) => actualizarFiltro({ entidad_tabla: entidades })}
      />

      <FiltrarLogsPorAccion
        resetKey={resetKey}
        onChange={(acciones) => actualizarFiltro({ accion: acciones })}
      />

      {hayFiltrosActivos && (
        <Button variant="outline" onClick={limpiarFiltros} className="text-sm">
          Limpiar
        </Button>
      )}
    </div>
  );
}
