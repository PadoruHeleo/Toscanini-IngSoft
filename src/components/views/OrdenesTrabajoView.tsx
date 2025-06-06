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
import { Badge } from "@/components/ui/badge";
import { Search, Plus, Eye, Trash2, Edit } from "lucide-react";
import OrdenTrabajoFormDialog from "./OrdenTrabajoFormDialog";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  has_garantia?: boolean;
  equipo_id?: number;
  created_by?: number;
  cotizacion_id?: number;
  informe_id?: number;
  pre_informe?: string;
  created_at?: string;
  finished_at?: string;
}

const getEstadoBadgeVariant = (estado?: string) => {
  switch (estado) {
    case "recibido":
      return "default";
    case "cotizacion_enviada":
      return "secondary";
    case "aprobacion_pendiente":
      return "destructive";
    case "en_reparacion":
      return "default";
    case "espera_de_retiro":
      return "secondary";
    case "entregado":
      return "default";
    case "abandonado":
      return "destructive";
    case "equipo_no_reparable":
      return "destructive";
    default:
      return "outline";
  }
};

const getPrioridadBadgeVariant = (prioridad?: string) => {
  switch (prioridad) {
    case "alta":
      return "destructive";
    case "media":
      return "default";
    case "baja":
      return "secondary";
    default:
      return "outline";
  }
};

export function OrdenesTrabajoView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [ordenes, setOrdenes] = useState<OrdenTrabajo[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingOrden, setEditingOrden] = useState<OrdenTrabajo | null>(null);

  const loadOrdenes = async () => {
    try {
      setLoading(true);
      const ordenesData = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      setOrdenes(ordenesData);
    } catch (error) {
      console.error("Error cargando órdenes de trabajo:", error);
      showError(
        "Error al cargar órdenes",
        "No se pudieron cargar las órdenes de trabajo."
      );
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadOrdenes();
  }, []);

  const filteredOrdenes = ordenes.filter((orden) => {
    if (!searchTerm.trim()) return true;
    const searchLower = searchTerm.toLowerCase();
    return (
      orden.orden_codigo?.toLowerCase().includes(searchLower) ||
      orden.orden_desc?.toLowerCase().includes(searchLower) ||
      orden.estado?.toLowerCase().includes(searchLower) ||
      orden.prioridad?.toLowerCase().includes(searchLower)
    );
  });

  const handleOrdenAdded = () => {
    loadOrdenes();
    setShowAddForm(false);
  };

  const handleOrdenUpdated = () => {
    loadOrdenes();
    setEditingOrden(null);
  };

  const handleEditOrden = (orden: OrdenTrabajo) => {
    setEditingOrden(orden);
  };

  const handleDeleteOrden = async (orden: OrdenTrabajo) => {
    if (!user) return;

    const confirmDelete = window.confirm(
      `¿Está seguro que desea eliminar la orden de trabajo "${orden.orden_codigo}"?\n\nEsta acción no se puede deshacer.`
    );

    if (!confirmDelete) return;

    try {
      const result = await invoke<boolean>("delete_orden_trabajo", {
        ordenId: orden.orden_id,
        deletedBy: user.usuario_id,
      });

      if (result) {
        success(
          "Orden eliminada",
          `La orden ${orden.orden_codigo} ha sido eliminada exitosamente.`
        );
        loadOrdenes();
      } else {
        showError("Error", "No se pudo eliminar la orden de trabajo.");
      }
    } catch (error) {
      console.error("Error eliminando orden:", error);
      showError(
        "Error al eliminar orden",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    }
  };

  const handleVerCotizacion = async (orden: OrdenTrabajo) => {
    if (!orden.cotizacion_id) {
      showError(
        "Sin cotización",
        "Esta orden no tiene una cotización asociada."
      );
      return;
    }

    try {
      // TODO: Implementar vista de cotización
      showError(
        "Función no implementada",
        "La vista de cotización estará disponible próximamente."
      );
    } catch (error) {
      showError("Error", "No se pudo abrir la cotización.");
    }
  };

  const handleCrearCotizacion = async (orden: OrdenTrabajo) => {
    if (orden.cotizacion_id) {
      showError(
        "Cotización existente",
        "Esta orden ya tiene una cotización asociada."
      );
      return;
    }

    try {
      // TODO: Implementar creación de cotización
      showError(
        "Función no implementada",
        "La creación de cotización estará disponible próximamente."
      );
    } catch (error) {
      showError("Error", "No se pudo crear la cotización.");
    }
  };

  const handleVerInforme = async (orden: OrdenTrabajo) => {
    if (!orden.informe_id) {
      showError("Sin informe", "Esta orden no tiene un informe asociado.");
      return;
    }

    try {
      // TODO: Implementar vista de informe
      showError(
        "Función no implementada",
        "La vista de informe estará disponible próximamente."
      );
    } catch (error) {
      showError("Error", "No se pudo abrir el informe.");
    }
  };

  const handleCrearInforme = async (orden: OrdenTrabajo) => {
    if (orden.informe_id) {
      showError(
        "Informe existente",
        "Esta orden ya tiene un informe asociado."
      );
      return;
    }

    try {
      // TODO: Implementar creación de informe
      showError(
        "Función no implementada",
        "La creación de informe estará disponible próximamente."
      );
    } catch (error) {
      showError("Error", "No se pudo crear el informe.");
    }
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return "N/A";
    return new Date(dateString).toLocaleDateString("es-CL");
  };

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando órdenes de trabajo...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>
          <Plus className="h-4 w-4 mr-2" />
          Crear Orden de Trabajo
        </Button>
      </div>

      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Buscar órdenes..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
          />
        </div>
        {searchTerm && (
          <Button variant="outline" onClick={() => setSearchTerm("")}>
            Limpiar
          </Button>
        )}
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Código</TableHead>
              <TableHead>Descripción</TableHead>
              <TableHead>Estado</TableHead>
              <TableHead>Prioridad</TableHead>
              <TableHead>Garantía</TableHead>
              <TableHead>Fecha Creación</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredOrdenes.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron órdenes de trabajo"
                    : "No hay órdenes de trabajo registradas"}
                </TableCell>
              </TableRow>
            ) : (
              filteredOrdenes.map((orden) => (
                <TableRow key={orden.orden_id}>
                  <TableCell className="font-medium">
                    {orden.orden_codigo || "N/A"}
                  </TableCell>
                  <TableCell className="max-w-xs truncate">
                    {orden.orden_desc || "N/A"}
                  </TableCell>
                  <TableCell>
                    <Badge variant={getEstadoBadgeVariant(orden.estado)}>
                      {orden.estado?.replace(/_/g, " ") || "N/A"}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <Badge variant={getPrioridadBadgeVariant(orden.prioridad)}>
                      {orden.prioridad || "N/A"}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={orden.has_garantia ? "default" : "secondary"}
                    >
                      {orden.has_garantia ? "Sí" : "No"}
                    </Badge>
                  </TableCell>
                  <TableCell>{formatDate(orden.created_at)}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end flex-wrap">
                      {/* Botones de cotización */}
                      {orden.cotizacion_id ? (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleVerCotizacion(orden)}
                          className="text-blue-600 hover:text-blue-700"
                          title="Ver cotización"
                        >
                          <Eye className="h-3 w-3" />
                          Cotización
                        </Button>
                      ) : (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleCrearCotizacion(orden)}
                          className="text-green-600 hover:text-green-700"
                          title="Crear cotización"
                        >
                          <Plus className="h-3 w-3" />
                          Cotización
                        </Button>
                      )}

                      {/* Botones de informe */}
                      {orden.informe_id ? (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleVerInforme(orden)}
                          className="text-blue-600 hover:text-blue-700"
                          title="Ver informe"
                        >
                          <Eye className="h-3 w-3" />
                          Informe
                        </Button>
                      ) : (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleCrearInforme(orden)}
                          className="text-green-600 hover:text-green-700"
                          title="Crear informe"
                        >
                          <Plus className="h-3 w-3" />
                          Informe
                        </Button>
                      )}

                      {/* Botón editar */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditOrden(orden)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar orden"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>

                      {/* Botón eliminar */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleDeleteOrden(orden)}
                        className="text-red-600 hover:text-red-700"
                        title="Eliminar orden"
                      >
                        <Trash2 className="h-3 w-3" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Total de órdenes */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {filteredOrdenes.length} orden
        {filteredOrdenes.length !== 1 ? "es" : ""} de trabajo
        {searchTerm && ` (filtrado de ${ordenes.length})`}
      </div>

      {/* Dialog para agregar orden */}
      <OrdenTrabajoFormDialog
        open={showAddForm}
        onOpenChange={setShowAddForm}
        onOrdenAdded={handleOrdenAdded}
      />

      {/* Dialog para editar orden */}
      {editingOrden && (
        <OrdenTrabajoFormDialog
          open={!!editingOrden}
          onOpenChange={(open: boolean) => !open && setEditingOrden(null)}
          onOrdenAdded={handleOrdenUpdated}
          orden={editingOrden}
          isEditing={true}
        />
      )}
    </div>
  );
}
