import { useState, useEffect } from "react";
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
import { Search, Edit, History, Filter } from "lucide-react";
import { EquipoFormDialog } from "@/components/views/EquipoFormDialog";
import { EquipoHistorialDialog } from "@/components/views/EquipoHistorialDialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";

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

  // Estados para filtros
  const [filtros, setFiltros] = useState<FiltrosEquipos>({});
  const [marcasDisponibles, setMarcasDisponibles] = useState<string[]>([]);
  const [tiposDisponibles, setTiposDisponibles] = useState<string[]>([]);
  const [clientesDisponibles, setClientesDisponibles] = useState<string[]>([]);
  const [estadosDisponibles, setEstadosDisponibles] = useState<string[]>([]);
  const [showFilters, setShowFilters] = useState(false);

  const loadEquipos = async () => {
    try {
      setLoading(true);
      const filtrosActuales = {
        ...filtros,
        search: searchTerm || undefined,
      };
      const equiposData = await invoke<EquipoConEstado[]>(
        "get_equipos_filtrados",
        {
          filtros: filtrosActuales,
        }
      );
      setEquipos(equiposData);
    } catch (error) {
      console.error("Error cargando equipos:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadFilterData = async () => {
    try {
      const [marcas, tipos, clientes, estados] = await Promise.all([
        invoke<string[]>("get_equipos_marcas"),
        invoke<string[]>("get_tipos_equipos"),
        invoke<string[]>("get_clientes_con_equipos"),
        invoke<string[]>("get_estados_ordenes_trabajo"),
      ]);

      setMarcasDisponibles(marcas);
      setTiposDisponibles(tipos);
      setClientesDisponibles(clientes);
      setEstadosDisponibles(estados);
    } catch (error) {
      console.error("Error cargando datos de filtros:", error);
    }
  };

  useEffect(() => {
    loadEquipos();
  }, [filtros, searchTerm]);

  useEffect(() => {
    loadFilterData();
  }, []);

  const handleEquipoAdded = () => {
    loadEquipos();
    setShowAddForm(false);
    setEditingEquipo(null);
  };

  const handleEditEquipo = (equipo: EquipoConEstado) => {
    setEditingEquipo(equipo);
    setShowAddForm(true);
  };

  const clearFilters = () => {
    setFiltros({});
    setSearchTerm("");
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
        <div className="flex gap-2">
          <Button
            variant="outline"
            onClick={() => setShowFilters(!showFilters)}
            className="flex items-center gap-2"
          >
            <Filter className="h-4 w-4" />
            Filtros
          </Button>
          <Button onClick={() => setShowAddForm(true)}>Agregar Equipo</Button>
        </div>
      </div>

      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Buscar equipos..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
          />
        </div>
        {(searchTerm || Object.keys(filtros).length > 0) && (
          <Button variant="outline" onClick={clearFilters}>
            Limpiar Filtros
          </Button>
        )}
      </div>

      {/* Panel de filtros avanzados */}
      {showFilters && (
        <div className="bg-gray-50 p-4 rounded-lg mb-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Marca</label>
            <Select
              value={filtros.marcas?.[0] || ""}
              onValueChange={(value) =>
                setFiltros((prev) => ({
                  ...prev,
                  marcas: value ? [value] : undefined,
                }))
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Todas las marcas" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">Todas las marcas</SelectItem>
                {marcasDisponibles.map((marca) => (
                  <SelectItem key={marca} value={marca}>
                    {marca}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Tipo</label>
            <Select
              value={filtros.tipos?.[0] || ""}
              onValueChange={(value) =>
                setFiltros((prev) => ({
                  ...prev,
                  tipos: value ? [value] : undefined,
                }))
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Todos los tipos" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">Todos los tipos</SelectItem>
                {tiposDisponibles.map((tipo) => (
                  <SelectItem key={tipo} value={tipo}>
                    {tipo}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Cliente</label>
            <Select
              value={filtros.clientes?.[0] || ""}
              onValueChange={(value) =>
                setFiltros((prev) => ({
                  ...prev,
                  clientes: value ? [value] : undefined,
                }))
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Todos los clientes" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">Todos los clientes</SelectItem>
                {clientesDisponibles.map((cliente) => (
                  <SelectItem key={cliente} value={cliente}>
                    {cliente}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">
              Estado de Orden
            </label>
            <Select
              value={filtros.estados_orden?.[0] || ""}
              onValueChange={(value) =>
                setFiltros((prev) => ({
                  ...prev,
                  estados_orden: value ? [value] : undefined,
                }))
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Todos los estados" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">Todos los estados</SelectItem>
                <SelectItem value="null">Sin órdenes</SelectItem>
                {estadosDisponibles.map((estado) => (
                  <SelectItem key={estado} value={estado}>
                    {estado === "recibido"
                      ? "Recibido"
                      : estado === "cotizacion_enviada"
                      ? "Cotización Enviada"
                      : estado === "aprobacion_pendiente"
                      ? "Aprobación Pendiente"
                      : estado === "en_reparacion"
                      ? "En Reparación"
                      : estado === "espera_de_retiro"
                      ? "Esperando Retiro"
                      : estado === "entregado"
                      ? "Entregado"
                      : estado === "abandonado"
                      ? "Abandonado"
                      : estado === "equipo_no_reparable"
                      ? "No Reparable"
                      : estado}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">
              Ordenar por
            </label>
            <Select
              value={filtros.ordenamiento || ""}
              onValueChange={(value) =>
                setFiltros((prev) => ({
                  ...prev,
                  ordenamiento: value || undefined,
                }))
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Fecha desc." />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="fecha_desc">Fecha (más reciente)</SelectItem>
                <SelectItem value="fecha_asc">Fecha (más antigua)</SelectItem>
                <SelectItem value="marca_asc">Marca (A-Z)</SelectItem>
                <SelectItem value="marca_desc">Marca (Z-A)</SelectItem>
                <SelectItem value="cliente_asc">Cliente (A-Z)</SelectItem>
                <SelectItem value="cliente_desc">Cliente (Z-A)</SelectItem>
                <SelectItem value="estado_asc">Estado (A-Z)</SelectItem>
                <SelectItem value="estado_desc">Estado (Z-A)</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      )}
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
              <TableHead className="w-[160px]">Estado</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {equipos.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={8}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm || Object.keys(filtros).length > 0
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
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => setHistorialEquipo(equipo)}
                        className="text-blue-600 hover:text-blue-700"
                        title="Ver historial del equipo"
                      >
                        <History className="h-3 w-3" />
                      </Button>
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
      <EquipoHistorialDialog
        open={historialEquipo !== null}
        onOpenChange={(open) => !open && setHistorialEquipo(null)}
        equipo={historialEquipo}
      />
    </div>
  );
}
