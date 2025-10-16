import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ViewTitle } from "@/components/ViewTitle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Search, Eye } from "lucide-react";
import { useToastContext } from "@/contexts/ToastContext";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { UnificarFiltrosLogs } from "./UnificarFiltrosLogs";

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

export function LogsAuditoriaView() {
  const { error: showError } = useToastContext();
  const [logs, setLogs] = useState<AuditLogWithUser[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedLog, setSelectedLog] = useState<AuditLogWithUser | null>(null);
  const [showDetailDialog, setShowDetailDialog] = useState(false);

  const searchInputRef = useRef<HTMLInputElement>(null);

  // Cargar logs inicial
  useEffect(() => {
    const loadInitialLogs = async () => {
      try {
        setLoading(true);
        const logsData = await invoke<AuditLogWithUser[]>("get_audit_logs", {
          filters: null,
        });
        setLogs(logsData);
      } catch (error) {
        console.error("Error cargando logs:", error);
        showError(
          "Error al cargar logs",
          "No se pudieron cargar los registros de auditoría."
        );
      } finally {
        setLoading(false);
      }
    };
    loadInitialLogs();
  }, []);

  const handleClearSearch = () => setSearchTerm("");

  const handleLogsFiltrados = (logsFiltrados: AuditLogWithUser[]) => {
    setLogs(logsFiltrados);
    setLoading(false);
  };

  const handleViewDetails = (log: AuditLogWithUser) => {
    setSelectedLog(log);
    setShowDetailDialog(true);
  };

  const formatDate = (dateString: string | null) => {
    if (!dateString) return "N/A";
    try {
      const date = new Date(dateString);
      return date.toLocaleString("es-CL", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    } catch {
      return dateString;
    }
  };

  const getAccionLabel = (accion: string | null) => {
    if (!accion) return "N/A";
    const accionMap: { [key: string]: string } = {
      CREATE: "Crear",
      UPDATE: "Actualizar",
      DELETE: "Eliminar",
      LOGIN: "Inicio de sesión",
      LOGOUT: "Cierre de sesión",
      LOGIN_FAILED: "Inicio de sesión fallido",
    };
    return accionMap[accion.toUpperCase()] || accion;
  };

  const getTablaLabel = (tabla: string | null) => {
    if (!tabla) return "N/A";
    const tablaMap: { [key: string]: string } = {
      USUARIO: "Usuario",
      CLIENTE: "Cliente",
      EQUIPO: "Equipo",
      COTIZACION: "Cotización",
      INFORME: "Informe",
      ORDEN_TRABAJO: "Orden de Trabajo",
      PIEZA: "Pieza",
    };
    return tablaMap[tabla.toUpperCase()] || tabla;
  };

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando logs de auditoría...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
      </div>

      {/* Barra de búsqueda y filtros unificados */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative w-auto">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            ref={searchInputRef}
            placeholder="Buscar por acción..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
            title="Buscar por acción"
          />
        </div>
        <div className="flex-grow min-w-0">
          <UnificarFiltrosLogs
            searchTerm={searchTerm}
            onFiltrar={handleLogsFiltrados}
            onClearSearch={handleClearSearch}
          />
        </div>
      </div>

      {/* Tabla de logs */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>ID</TableHead>
              <TableHead>Fecha y Hora</TableHead>
              <TableHead>Usuario</TableHead>
              <TableHead>Acción</TableHead>
              <TableHead>Entidad</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {logs.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={6}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron registros"
                    : "No hay registros de auditoría"}
                </TableCell>
              </TableRow>
            ) : (
              logs.map((log) => (
                <TableRow key={log.log_id}>
                  <TableCell className="font-medium">{log.log_id}</TableCell>
                  <TableCell>{formatDate(log.created_at)}</TableCell>
                  <TableCell>{log.usuario_nombre || "Sistema"}</TableCell>
                  <TableCell>{getAccionLabel(log.log_accion)}</TableCell>
                  <TableCell>{getTablaLabel(log.log_entidad_tabla)}</TableCell>
                  <TableCell className="text-right">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleViewDetails(log)}
                      className="text-blue-600 hover:text-blue-700"
                      title="Ver detalles"
                    >
                      <Eye className="h-3 w-3" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Total de logs */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {logs.length} registro{logs.length !== 1 ? "s" : ""}
        {searchTerm && (
          <span className="ml-2 text-blue-600">
            (filtrado por: "{searchTerm}")
          </span>
        )}
      </div>

      {/* Dialog para ver detalles del log */}
      <Dialog open={showDetailDialog} onOpenChange={setShowDetailDialog}>
        <DialogContent className="max-w-3xl max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Detalles del Log de Auditoría</DialogTitle>
          </DialogHeader>
          {selectedLog && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <h3 className="font-semibold text-sm text-gray-600">
                    Fecha y Hora
                  </h3>
                  <p className="text-sm">
                    {formatDate(selectedLog.created_at)}
                  </p>
                </div>
                <div>
                  <h3 className="font-semibold text-sm text-gray-600">
                    Acción
                  </h3>
                  <p className="text-sm">
                    {getAccionLabel(selectedLog.log_accion)}
                  </p>
                </div>
                <div>
                  <h3 className="font-semibold text-sm text-gray-600">
                    Usuario
                  </h3>
                  <p className="text-sm">
                    {selectedLog.usuario_nombre || "Sistema"}
                  </p>
                  {selectedLog.usuario_correo && (
                    <p className="text-xs text-gray-500">
                      {selectedLog.usuario_correo}
                    </p>
                  )}
                </div>
                <div>
                  <h3 className="font-semibold text-sm text-gray-600">
                    Entidad
                  </h3>
                  <p className="text-sm">
                    {getTablaLabel(selectedLog.log_entidad_tabla)}
                  </p>
                </div>
              </div>

              {/* Valores anteriores y nuevos */}
              <div className="space-y-2">
                <div>
                  <h3 className="font-semibold text-sm text-gray-600 mb-1">
                    Resultado
                  </h3>
                  <div className="bg-gray-50 p-3 rounded-md max-h-48 overflow-auto">
                    <pre className="text-xs whitespace-pre-wrap">
                      {selectedLog.log_new_v || "N/A"}
                    </pre>
                  </div>
                </div>
              </div>

              <div className="flex justify-end">
                <Button onClick={() => setShowDetailDialog(false)}>
                  Cerrar
                </Button>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
