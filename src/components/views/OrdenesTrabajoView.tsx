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
import { Search, Plus, Eye, Trash2, Edit } from "lucide-react";
import OrdenTrabajoFormDialog from "./OrdenTrabajoFormDialog";
import CotizacionFormDialog from "./CotizacionFormDialog";
import InformeFormDialog from "./InformeFormDialog";
import { FiltrarOrdenesPorFecha } from "./FiltrarOrdenesPorFecha";
import { FiltrarOrdenesPorPrioridad } from "./FiltrarOrdenesPorPrioridad";
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

const getEstadoStyles = (estado?: string) => {
  switch (estado) {
    case "recibido":
      return "bg-blue-100 text-blue-800 border border-blue-200 hover:bg-blue-200";
    case "cotizacion_enviada":
      return "bg-purple-100 text-purple-800 border border-purple-200 hover:bg-purple-200";
    case "aprobacion_pendiente":
      return "bg-amber-100 text-amber-800 border border-amber-200 hover:bg-amber-200";
    case "en_reparacion":
      return "bg-indigo-100 text-indigo-800 border border-indigo-200 hover:bg-indigo-200";
    case "espera_de_retiro":
      return "bg-orange-100 text-orange-800 border border-orange-200 hover:bg-orange-200";
    case "entregado":
      return "bg-green-100 text-green-800 border border-green-200 hover:bg-green-200";
    case "abandonado":
      return "bg-gray-100 text-gray-800 border border-gray-200 hover:bg-gray-200";
    case "equipo_no_reparable":
      return "bg-red-100 text-red-800 border border-red-200 hover:bg-red-200";
    default:
      return "bg-gray-100 text-gray-600 border border-gray-200";
  }
};

const getPrioridadStyles = (prioridad?: string) => {
  switch (prioridad) {
    case "alta":
      return "bg-red-100 text-red-800 border border-red-200 hover:bg-red-200 font-semibold";
    case "media":
      return "bg-yellow-100 text-yellow-800 border border-yellow-200 hover:bg-yellow-200";
    case "baja":
      return "bg-green-100 text-green-800 border border-green-200 hover:bg-green-200";
    default:
      return "bg-gray-100 text-gray-600 border border-gray-200";
  }
};

const formatEstadoText = (estado?: string) => {
  const estadoMap: { [key: string]: string } = {
    recibido: "Recibido",
    cotizacion_enviada: "Cotización Enviada",
    aprobacion_pendiente: "Aprobación Pendiente",
    en_reparacion: "En Reparación",
    espera_de_retiro: "Espera de Retiro",
    entregado: "Entregado",
    abandonado: "Abandonado",
    equipo_no_reparable: "Equipo No Reparable",
  };
  return estadoMap[estado || ""] || "N/A";
};

const formatPrioridadText = (prioridad?: string) => {
  const prioridadMap: { [key: string]: string } = {
    alta: "🔴 Alta",
    media: "🟡 Media",
    baja: "🟢 Baja",
  };
  return prioridadMap[prioridad || ""] || "N/A";
};

const getCotizacionButtonInfo = (orden: OrdenTrabajo) => {
  if (!orden.cotizacion_id) {
    return {
      hasQuote: false,
      text: "Crear Cotización",
      icon: "plus",
      className: "text-green-600 hover:text-green-700",
      title: "Crear nueva cotización para esta orden",
    };
  }

  const isSent = orden.estado === "cotizacion_enviada";
  return {
    hasQuote: true,
    text: isSent ? "Ver Cotización ✓" : "Ver Cotización",
    icon: "eye",
    className: isSent
      ? "text-purple-600 hover:text-purple-700"
      : "text-blue-600 hover:text-blue-700",
    title: isSent
      ? "Ver cotización enviada al cliente"
      : "Ver cotización (borrador)",
  };
};

export function OrdenesTrabajoView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [ordenes, setOrdenes] = useState<OrdenTrabajo[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingOrden, setEditingOrden] = useState<OrdenTrabajo | null>(null);
  const [showCotizacionForm, setShowCotizacionForm] = useState(false);
  const [selectedOrdenForCotizacion, setSelectedOrdenForCotizacion] =
    useState<OrdenTrabajo | null>(null);
  const [editingCotizacion, setEditingCotizacion] = useState<any>(null);
  const [loadingCotizacion, setLoadingCotizacion] = useState<number | null>(
    null
  );
  const [showInformeForm, setShowInformeForm] = useState(false);
  const [selectedOrdenForInforme, setSelectedOrdenForInforme] =
    useState<OrdenTrabajo | null>(null);
  const [editingInforme, setEditingInforme] = useState<any>(null);

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
  const handleCotizacionAdded = () => {
    // Recargar las órdenes para ver los cambios
    loadOrdenes();
    // Cerrar el formulario
    setShowCotizacionForm(false);
    setSelectedOrdenForCotizacion(null);
    setEditingCotizacion(null);
    // Limpiar estado de loading
    setLoadingCotizacion(null);

    // Mostrar mensaje de éxito
    success(
      "Cotización procesada",
      editingCotizacion
        ? "La cotización ha sido actualizada exitosamente."
        : "La cotización ha sido creada exitosamente."
    );
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
      setLoadingCotizacion(orden.orden_id);

      // Obtener los detalles de la cotización
      const cotizacion = await invoke("get_cotizacion_by_id", {
        cotizacionId: orden.cotizacion_id,
      });

      // Abrir el formulario de cotización en modo edición
      setSelectedOrdenForCotizacion(orden);
      setEditingCotizacion(cotizacion);
      setShowCotizacionForm(true);
    } catch (error) {
      console.error("Error obteniendo cotización:", error);
      showError("Error", "No se pudo obtener la cotización.");
    } finally {
      setLoadingCotizacion(null);
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

    // Abrir el formulario para crear nueva cotización directamente, sin confirmación
    try {
      setLoadingCotizacion(orden.orden_id);

      setSelectedOrdenForCotizacion(orden);
      setEditingCotizacion(null);
      setShowCotizacionForm(true);
    } catch (error) {
      showError("Error", "No se pudo abrir el formulario de cotización.");
    } finally {
      setLoadingCotizacion(null);
    }
  };
  const handleVerInforme = async (orden: OrdenTrabajo) => {
    if (!orden.informe_id) {
      showError("Sin informe", "Esta orden no tiene un informe asociado.");
      return;
    }
    try {
      // Cargar el informe desde el backend
      const informeData = await invoke<any>("get_informe_by_id", {
        informeId: orden.informe_id,
      });

      if (informeData) {
        setEditingInforme(informeData);
        setSelectedOrdenForInforme(orden);
        setShowInformeForm(true);
      } else {
        showError("Error", "No se pudo cargar el informe.");
      }
    } catch (error) {
      console.error("Error cargando informe:", error);
      showError(
        "Error",
        `No se pudo abrir el informe.\n${
          error instanceof Error ? error.message : JSON.stringify(error)
        }`
      );
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
      // Abrir diálogo de creación de informe
      setEditingInforme(null);
      setSelectedOrdenForInforme(orden);
      setShowInformeForm(true);
    } catch (error) {
      console.error("Error preparando creación de informe:", error);
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
        {/* boton para las fechas */}
        <FiltrarOrdenesPorFecha
          onFiltrar={(ordenesFiltradas) => setOrdenes(ordenesFiltradas)}
        />
        <FiltrarOrdenesPorPrioridad
          onFiltrar={(ordenesFiltradas) => setOrdenes(ordenesFiltradas)}
        />
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
                  </TableCell>{" "}
                  <TableCell>
                    <span
                      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors ${getEstadoStyles(
                        orden.estado
                      )}`}
                    >
                      {formatEstadoText(orden.estado)}
                    </span>
                  </TableCell>
                  <TableCell>
                    <span
                      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs transition-colors ${getPrioridadStyles(
                        orden.prioridad
                      )}`}
                    >
                      {formatPrioridadText(orden.prioridad)}
                    </span>
                  </TableCell>{" "}
                  <TableCell>
                    <span
                      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors ${
                        orden.has_garantia
                          ? "bg-emerald-100 text-emerald-800 border border-emerald-200 hover:bg-emerald-200"
                          : "bg-slate-100 text-slate-800 border border-slate-200 hover:bg-slate-200"
                      }`}
                    >
                      {orden.has_garantia ? "✅ Sí" : "❌ No"}
                    </span>
                  </TableCell>
                  <TableCell>{formatDate(orden.created_at)}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end flex-wrap">
                      {" "}
                      {/* Botones de cotización */}
                      {(() => {
                        const buttonInfo = getCotizacionButtonInfo(orden);
                        return (
                          <Button
                            variant="outline"
                            size="sm"
                            onClick={() =>
                              buttonInfo.hasQuote
                                ? handleVerCotizacion(orden)
                                : handleCrearCotizacion(orden)
                            }
                            disabled={loadingCotizacion === orden.orden_id}
                            className={`${buttonInfo.className} disabled:opacity-50`}
                            title={buttonInfo.title}
                          >
                            {loadingCotizacion === orden.orden_id ? (
                              <div className="w-3 h-3 border border-current border-t-transparent rounded-full animate-spin" />
                            ) : buttonInfo.icon === "eye" ? (
                              <Eye className="h-3 w-3" />
                            ) : (
                              <Plus className="h-3 w-3" />
                            )}
                            {buttonInfo.text}
                          </Button>
                        );
                      })()}
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
      )}{" "}
      {/* Dialog para crear/editar cotización */}
      <CotizacionFormDialog
        open={showCotizacionForm}
        onOpenChange={(open) => {
          setShowCotizacionForm(open);
          if (!open) {
            // Limpiar estados cuando se cierre el diálogo
            setSelectedOrdenForCotizacion(null);
            setEditingCotizacion(null);
            setLoadingCotizacion(null);
          }
        }}
        onCotizacionAdded={handleCotizacionAdded}
        cotizacion={editingCotizacion}
        isEditing={!!editingCotizacion}
        ordenTrabajoId={selectedOrdenForCotizacion?.orden_id}
        onSendToClient={async (cotizacionId) => {
          try {
            if (!user || !selectedOrdenForCotizacion) return;

            // Actualizar el estado de la orden a "cotizacion_enviada" cuando se envíe la cotización
            await invoke("cambiar_estado_orden_trabajo", {
              ordenId: selectedOrdenForCotizacion.orden_id,
              nuevoEstado: "cotizacion_enviada",
              updatedBy: user.usuario_id,
            });

            success(
              "Cotización enviada",
              "La cotización ha sido enviada al cliente exitosamente."
            );

            console.log(`Cotización ${cotizacionId} enviada al cliente`);
            loadOrdenes(); // Recargar la lista para ver el cambio de estado
          } catch (error) {
            console.error("Error actualizando estado de orden:", error);
            showError(
              "Advertencia",
              "La cotización se envió pero no se pudo actualizar el estado de la orden."
            );
            loadOrdenes(); // Recargar de todas formas
          }
        }}
      />
      {/* Dialog para crear/editar informe */}
      <InformeFormDialog
        open={showInformeForm}
        onOpenChange={(open) => {
          setShowInformeForm(open);
          if (!open) {
            // Limpiar estados cuando se cierre el diálogo
            setSelectedOrdenForInforme(null);
            setEditingInforme(null);
          }
        }}
        onInformeAdded={() => {
          loadOrdenes(); // Recargar la lista de órdenes
        }}
        informe={editingInforme}
        isEditing={!!editingInforme}
        ordenTrabajoId={selectedOrdenForInforme?.orden_id}
      />
    </div>
  );
}
