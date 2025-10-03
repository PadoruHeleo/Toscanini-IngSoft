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
import { Search, Edit, History } from "lucide-react";
import { EquipoFormDialog } from "@/components/views/EquipoFormDialog";
import { EquipoHistorialDialog } from "@/components/views/EquipoHistorialDialog";
import { UnificarFiltrosEquipos } from "@/components/views/UnificarFiltrosEquipos";
import { Badge } from "@/components/ui/badge";
import { usePermissions } from "@/hooks/use-permissions";

interface EquipoConEstado {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_precio?: number;
  equipo_ubicacion?: string;
  cliente_id?: number;
  created_by?: number;
  created_at?: string;
  // Información del cliente
  cliente_nombre?: string;
  // Estado de la última orden de trabajo
  ultimo_estado_orden?: string;
  ultimo_codigo_orden?: string;
  fecha_ultima_orden?: string;
}

interface FiltrosEquipos {
  fecha_inicio?: string;
  fecha_fin?: string;
  marcas?: string[];
  modelos?: string[];
  tipos?: string[];
  clientes?: string[];
  ubicaciones?: string[];
  estados_orden?: string[];
  search?: string;
  ordenamiento?: string;
  precio_min?: number;
  precio_max?: number;
}

export function EquiposView() {
  const [equipos, setEquipos] = useState<EquipoConEstado[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingEquipo, setEditingEquipo] = useState<EquipoConEstado | null>(
    null
  );
  const [historialEquipo, setHistorialEquipo] =
    useState<EquipoConEstado | null>(null);
  const [refreshFilters, setRefreshFilters] = useState(0);

  // Hook para verificar permisos de administrador
  const { isAdmin } = usePermissions();

  const searchInputRef = useRef<HTMLInputElement>(null);
  const searchTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Validación de texto (permitir letras, números, espacios y algunos caracteres especiales para equipos)
  const isValidText = (text: string) =>
    /^[a-zA-Z0-9áéíóúÁÉÍÓÚñÑ\s'\-._]*$/.test(text);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (isValidText(value)) setSearchTerm(value);
  };

  const handleClearSearch = () => setSearchTerm("");

  // Cargar equipos inicial
  useEffect(() => {
    const loadInitialEquipos = async () => {
      try {
        setLoading(true);
        const equiposData = await invoke<EquipoConEstado[]>(
          "get_equipos_con_estado"
        );
        setEquipos(equiposData);
      } catch (error) {
        console.error("Error cargando equipos:", error);
      } finally {
        setLoading(false);
      }
    };
    loadInitialEquipos();
  }, []);

  // Debounce búsqueda
  useEffect(() => {
    if (searchTimeoutRef.current) clearTimeout(searchTimeoutRef.current);

    searchTimeoutRef.current = setTimeout(() => {
      // UnificarFiltrosEquipos se encarga de filtrar
    }, 150);

    return () => {
      if (searchTimeoutRef.current) clearTimeout(searchTimeoutRef.current);
    };
  }, [searchTerm]);

  const handleEquipoAdded = () => {
    setShowAddForm(false);
    setRefreshFilters((prev) => prev + 1);
  };

  const handleEquipoUpdated = () => {
    setEditingEquipo(null);
    setRefreshFilters((prev) => prev + 1);
  };

  const handleEditEquipo = (equipo: EquipoConEstado) =>
    setEditingEquipo(equipo);
  const handleVerHistorial = (equipo: EquipoConEstado) =>
    setHistorialEquipo(equipo);

  const formatDate = (dateString?: string) =>
    dateString ? new Date(dateString).toLocaleDateString("es-CL") : "N/A";

  const handleEquiposFiltrados = (equiposFiltrados: EquipoConEstado[]) => {
    setEquipos(equiposFiltrados);
    setLoading(false);
  };

  const getEstadoBadge = (estado?: string) => {
    if (!estado) {
      return <Badge variant="secondary">Sin órdenes</Badge>;
    }

    const colorMap: { [key: string]: string } = {
      recibido: "bg-blue-100 text-blue-800",
      cotizacion_enviada: "bg-yellow-100 text-yellow-800",
      aprobacion_pendiente: "bg-orange-100 text-orange-800",
      en_reparacion: "bg-purple-100 text-purple-800",
      espera_de_retiro: "bg-green-100 text-green-800",
      entregado: "bg-gray-100 text-gray-800",
      abandonado: "bg-red-100 text-red-800",
      equipo_no_reparable: "bg-red-100 text-red-800",
    };

    const estadoTexto: { [key: string]: string } = {
      recibido: "Recibido",
      cotizacion_enviada: "Cotización Enviada",
      aprobacion_pendiente: "Aprobación Pendiente",
      en_reparacion: "En Reparación",
      espera_de_retiro: "Esperando Retiro",
      entregado: "Entregado",
      abandonado: "Abandonado",
      equipo_no_reparable: "No Reparable",
    };

    return (
      <Badge className={colorMap[estado] || "bg-gray-100 text-gray-800"}>
        {estadoTexto[estado] || estado}
      </Badge>
    );
  };

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando equipos...</div>
      </div>
    );
  }
  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>Agregar Equipo</Button>
      </div>

      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            ref={searchInputRef}
            placeholder="Buscar equipos..."
            value={searchTerm}
            onChange={handleSearchChange}
            className="pl-8"
          />
        </div>
        {searchTerm && (
          <Button variant="outline" onClick={handleClearSearch}>
            Limpiar búsqueda
          </Button>
        )}
      </div>

      {/* Panel de filtros unificado */}
      <div className="mb-4">
        <UnificarFiltrosEquipos
          key={refreshFilters}
          searchTerm={searchTerm}
          onFiltrar={handleEquiposFiltrados}
          onClearSearch={handleClearSearch}
        />
      </div>
      {/* Tabla de equipos */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[120px]">Marca</TableHead>
              <TableHead>Modelo</TableHead>
              <TableHead>Número de Serie</TableHead>
              <TableHead>Tipo</TableHead>
              <TableHead>Cliente</TableHead>
              <TableHead>Ubicación</TableHead>
              {isAdmin() && <TableHead className="w-[160px]">Estado</TableHead>}
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {equipos.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={isAdmin() ? 8 : 7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron equipos con los filtros aplicados"
                    : "No hay equipos registrados"}
                </TableCell>
              </TableRow>
            ) : (
              equipos.map((equipo) => (
                <TableRow key={equipo.equipo_id}>
                  <TableCell className="font-medium">
                    {equipo.equipo_marca || "N/A"}
                  </TableCell>
                  <TableCell>{equipo.equipo_modelo || "N/A"}</TableCell>
                  <TableCell>{equipo.numero_serie || "N/A"}</TableCell>
                  <TableCell>{equipo.equipo_tipo || "N/A"}</TableCell>
                  <TableCell>
                    {equipo.cliente_nombre || "Sin cliente"}
                  </TableCell>
                  <TableCell>{equipo.equipo_ubicacion || "N/A"}</TableCell>
                  {isAdmin() && (
                    <TableCell>
                      <div className="flex flex-col gap-1">
                        {getEstadoBadge(equipo.ultimo_estado_orden)}
                        {equipo.ultimo_codigo_orden && (
                          <div className="text-xs text-gray-500">
                            {equipo.ultimo_codigo_orden}
                          </div>
                        )}
                      </div>
                    </TableCell>
                  )}
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditEquipo(equipo)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar equipo"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>
                      {isAdmin() && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setHistorialEquipo(equipo)}
                          className="text-blue-600 hover:text-blue-700"
                          title="Ver historial del equipo"
                        >
                          <History className="h-3 w-3" />
                        </Button>
                      )}
                    </div>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Total de equipos */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {equipos.length} equipo{equipos.length !== 1 ? "s" : ""}
      </div>
      <EquipoFormDialog
        open={showAddForm || editingEquipo !== null}
        onOpenChange={(open) => {
          if (!open) {
            setShowAddForm(false);
            setEditingEquipo(null);
          }
        }}
        onEquipoAdded={handleEquipoAdded}
        equipo={editingEquipo || undefined}
        isEditing={editingEquipo !== null}
      />
      {isAdmin() && (
        <EquipoHistorialDialog
          open={historialEquipo !== null}
          onOpenChange={(open) => !open && setHistorialEquipo(null)}
          equipo={historialEquipo}
        />
      )}
    </div>
  );
}
