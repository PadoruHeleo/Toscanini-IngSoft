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
import { ViewTitle } from "@/components/layout/ViewTitle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Search,
  Plus,
  Eye,
  Trash2,
  Edit,
  FileText,
  LogOut,
} from "lucide-react";
import OrdenTrabajoFormDialog from "./dialogs/OrdenTrabajoFormDialog";
import CotizacionFormDialog from "./dialogs/CotizacionFormDialog";
import InformeFormDialog from "./dialogs/InformeFormDialog";
import { PdfViewer } from "@/components/features/documents/PdfViewer";
import { UnificarFiltros } from "./filters/UnificarFiltros";
import { RegistrarSalidaDialog } from "./dialogs/RegistrarSalidaDialog";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";
import { useOrdenTrabajoPermissions } from "@/hooks/use-permissions";

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
  estado_updated_at?: string;
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
    case "cotizacion_rechazada":
      return "bg-red-100 text-red-800 border border-red-200 hover:bg-red-200";
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
    cotizacion_rechazada: "Cotización Rechazada",
  };
  return estadoMap[estado || ""] || "N/A";
};

const formatPrioridadText = (prioridad?: string) => {
  const prioridadMap: { [key: string]: string } = {
    alta: "Alta",
    media: "Media",
    baja: "Baja",
  };
  return prioridadMap[prioridad || ""] || "N/A";
};

const getCotizacionButtonInfo = (orden: OrdenTrabajo) => {
  if (!orden.cotizacion_id) {
    return {
      hasQuote: false,
      text: "Crear Cotización",
      className: "text-green-600 hover:text-green-700",
      title: "Crear nueva cotización para esta orden",
    };
  }

  const isSent = orden.estado === "cotizacion_enviada";
  return {
    hasQuote: true,
    text: isSent ? "Ver Cotización" : "Ver Cotización",
    icon: "eye",
    className: isSent
      ? "text-purple-600 hover:text-purple-700 font-medium"
      : "text-blue-600 hover:text-blue-700",
    title: isSent
      ? "Cotización enviada - Pendiente de aprobación/rechazo"
      : "Ver cotización (borrador)",
  };
};

const getTiempoEnEstado = (createdAt?: string) => {
  if (!createdAt) return "N/A";
  const created = new Date(createdAt);
  const now = new Date();
  let diffMs = now.getTime() - created.getTime();

  const dias = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  diffMs -= dias * (1000 * 60 * 60 * 24);
  const horas = Math.floor(diffMs / (1000 * 60 * 60));
  diffMs -= horas * (1000 * 60 * 60);
  const minutos = Math.floor(diffMs / (1000 * 60));

  return `${dias}d ${horas}h ${minutos}m`;
};

export function OrdenesTrabajoView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const { getVisibleActions } = useOrdenTrabajoPermissions();
  const [ordenes, setOrdenes] = useState<OrdenTrabajo[]>([]);
  const [loading, setLoading] = useState(true);
  const [refreshFilters, setRefreshFilters] = useState(0);
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

  // Estados para PDF Viewer de Informes y Cotizaciones
  const [showInformePdfViewer, setShowInformePdfViewer] = useState(false);
  const [pdfInformeId, setPdfInformeId] = useState<number | null>(null);
  const [pdfCotizacionId, setPdfCotizacionId] = useState<number | null>(null);
  const [pdfOrdenCodigo, setPdfOrdenCodigo] = useState<string>("");

  // Estados para Registro de Salida
  const [showRegistrarSalidaDialog, setShowRegistrarSalidaDialog] =
    useState(false);
  const [selectedEquipoForSalida, setSelectedEquipoForSalida] =
    useState<any>(null);
  const [selectedOrdenForSalida, setSelectedOrdenForSalida] =
    useState<OrdenTrabajo | null>(null);

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

  const prioridadOrder = { alta: 0, media: 1, baja: 2 };
  const estadoOrder = {
    recibido: 0,
    cotizacion_enviada: 1,
    aprobacion_pendiente: 2,
    en_reparacion: 3,
    espera_de_retiro: 4,
    entregado: 5,
    abandonado: 6,
    equipo_no_reparable: 7,
  };

  const isTiempoEnEstadoCritico = (orden: OrdenTrabajo) => {
    if (orden.estado !== "en_reparacion" && orden.estado !== "recibido")
      return false;
    const base = orden.estado_updated_at || orden.created_at;
    if (!base) return false;
    const baseDate = new Date(base);
    const now = new Date();
    const diffMs = now.getTime() - baseDate.getTime();
    return diffMs > 60 * 60 * 1000;
  };

  const ordenesOrdenadas = [...filteredOrdenes].sort((a, b) => {
    const critA = isTiempoEnEstadoCritico(a) ? 0 : 1;
    const critB = isTiempoEnEstadoCritico(b) ? 0 : 1;
    if (critA !== critB) return critA - critB;

    const pa = prioridadOrder[a.prioridad as keyof typeof prioridadOrder] ?? 3;
    const pb = prioridadOrder[b.prioridad as keyof typeof prioridadOrder] ?? 3;
    if (pa !== pb) return pa - pb;

    const ea = estadoOrder[a.estado as keyof typeof estadoOrder] ?? 99;
    const eb = estadoOrder[b.estado as keyof typeof estadoOrder] ?? 99;
    return ea - eb;
  });

  const handleOrdenAdded = () => {
    loadOrdenes();
    setShowAddForm(false);
    setRefreshFilters((prev) => prev + 1);
  };

  const handleOrdenUpdated = () => {
    loadOrdenes();
    setEditingOrden(null);
    setRefreshFilters((prev) => prev + 1);
  };
  const handleEditOrden = (orden: OrdenTrabajo) => {
    setEditingOrden(orden);
    setRefreshFilters((prev) => prev + 1);
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
        setRefreshFilters((prev) => prev + 1);
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

      // Log para depuración
      console.log(
        "Abriendo CotizacionFormDialog, ordenTrabajoId:",
        orden.orden_id
      );

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
      // Log para depuración
      console.log(
        "Abriendo CotizacionFormDialog, ordenTrabajoId:",
        orden.orden_id
      );

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

  const handleVerInformePdf = (orden: OrdenTrabajo) => {
    if (!orden.informe_id && !orden.cotizacion_id) {
      showError(
        "Sin documentos",
        "Esta orden no tiene informe ni cotización asociados."
      );
      return;
    }
    setPdfInformeId(orden.informe_id || null);
    setPdfCotizacionId(orden.cotizacion_id || null);
    setPdfOrdenCodigo(orden.orden_codigo || "Sin código");
    setShowInformePdfViewer(true);
  };

  const handleCrearInforme = async (orden: OrdenTrabajo) => {
    if (orden.informe_id) {
      showError(
        "Informe existente",
        "Esta orden ya tiene un informe asociado."
      );
      return;
    }

    // NUEVA VALIDACIÓN: Verificar que la orden esté en estado "en_reparacion"
    if (orden.estado !== "en_reparacion") {
      showError(
        "Estado inválido",
        `No se puede crear un informe para una orden en estado "${formatEstadoText(
          orden.estado
        )}". Solo se pueden crear informes cuando la orden está en estado "En Reparación".`
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

  const handleRegistrarSalida = async (orden: OrdenTrabajo) => {
    if (!orden.equipo_id) {
      showError("Error", "Esta orden no tiene un equipo asociado.");
      return;
    }

    try {
      // Obtener información del equipo
      const equipoData = await invoke<any>("get_equipo_by_id", {
        equipoId: orden.equipo_id,
      });

      if (equipoData) {
        setSelectedEquipoForSalida(equipoData);
        setSelectedOrdenForSalida(orden);
        setShowRegistrarSalidaDialog(true);
      } else {
        showError("Error", "No se pudo cargar la información del equipo.");
      }
    } catch (error) {
      console.error("Error cargando equipo:", error);
      showError("Error", `No se pudo cargar el equipo: ${error}`);
    }
  };

  const handleSalidaRegistrada = () => {
    // Recargar las órdenes después de registrar la salida
    loadOrdenes();
    // Limpiar estados
    setSelectedEquipoForSalida(null);
    setSelectedOrdenForSalida(null);
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
        <ViewTitle onRefresh={loadOrdenes} />
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
        {/* boton para los filtos */}
        <UnificarFiltros
          key={refreshFilters}
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
              <TableHead>Duración</TableHead>
              <TableHead>Prioridad</TableHead>
              <TableHead>Garantía</TableHead>
              <TableHead>Fecha Creación</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {ordenesOrdenadas.length === 0 ? (
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
              ordenesOrdenadas.map((orden) => (
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
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-gray-700 w-[5.5rem] inline-block text-right">
                        {getTiempoEnEstado(
                          orden.estado_updated_at || orden.created_at
                        )}
                      </span>
                      {isTiempoEnEstadoCritico(orden) && (
                        <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-red-100 text-red-800 border border-red-300 animate-pulse">
                          <svg
                            className="w-3 h-3"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                          </svg>
                          Atrasado
                        </span>
                      )}
                    </div>
                  </TableCell>
                  <TableCell>
                    <span
                      className={`inline-flex items-center justify-center px-2.5 py-0.5 rounded-full text-xs transition-colors min-w-[4.5rem] ${getPrioridadStyles(
                        orden.prioridad
                      )}`}
                    >
                      {formatPrioridadText(orden.prioridad)}
                    </span>
                  </TableCell>{" "}
                  <TableCell>
                    <span
                      className={`inline-flex items-center justify-center px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors min-w-[3rem] ${
                        orden.has_garantia
                          ? "bg-emerald-100 text-emerald-800 border border-emerald-200 hover:bg-emerald-200"
                          : "bg-slate-100 text-slate-800 border border-slate-200 hover:bg-slate-200"
                      }`}
                    >
                      {orden.has_garantia ? "Sí" : "No"}
                    </span>
                  </TableCell>
                  <TableCell>{formatDate(orden.created_at)}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-between items-center flex-wrap w-full">
                      {(() => {
                        const actions = getVisibleActions(orden);

                        return (
                          <>
                            {/* Grupo de botones principales (izquierda) */}
                            <div className="flex gap-1 flex-wrap">
                              {/* Botones de cotización */}
                              {actions.showCreateCotizacion &&
                                (() => {
                                  const buttonInfo =
                                    getCotizacionButtonInfo(orden);
                                  return (
                                    <Button
                                      variant="outline"
                                      size="sm"
                                      onClick={() =>
                                        handleCrearCotizacion(orden)
                                      }
                                      disabled={
                                        loadingCotizacion === orden.orden_id
                                      }
                                      className={`${buttonInfo.className} disabled:opacity-50`}
                                      title={buttonInfo.title}
                                    >
                                      {loadingCotizacion === orden.orden_id ? (
                                        <div className="w-3 h-3 border border-current border-t-transparent rounded-full animate-spin" />
                                      ) : (
                                        <Plus className="h-3 w-3" />
                                      )}
                                      {buttonInfo.text}
                                    </Button>
                                  );
                                })()}

                              {actions.showViewCotizacion &&
                                (() => {
                                  const buttonInfo =
                                    getCotizacionButtonInfo(orden);
                                  return (
                                    <Button
                                      variant="outline"
                                      size="sm"
                                      onClick={() => handleVerCotizacion(orden)}
                                      disabled={
                                        loadingCotizacion === orden.orden_id
                                      }
                                      className={`${buttonInfo.className} disabled:opacity-50`}
                                      title={buttonInfo.title}
                                    >
                                      {loadingCotizacion === orden.orden_id ? (
                                        <div className="w-3 h-3 border border-current border-t-transparent rounded-full animate-spin" />
                                      ) : (
                                        <Eye className="h-3 w-3" />
                                      )}
                                      {buttonInfo.text}
                                    </Button>
                                  );
                                })()}

                              {/* Botones de informe */}
                              {actions.showCreateInforme && (
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => handleCrearInforme(orden)}
                                  className="text-green-600 hover:text-green-700"
                                  title="Crear nuevo informe"
                                >
                                  <Plus className="h-3 w-3" />
                                  Crear Informe
                                </Button>
                              )}

                              {actions.showViewInforme && (
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => handleVerInforme(orden)}
                                  className="text-blue-600 hover:text-blue-700"
                                  title="Ver informe existente"
                                >
                                  <Eye className="h-3 w-3" />
                                  Ver Informe
                                </Button>
                              )}

                              {/* Botón registrar salida - visible en estados compatibles */}
                              {actions.showRegistrarSalida && (
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => handleRegistrarSalida(orden)}
                                  className="text-orange-600 hover:text-orange-700"
                                  title="Registrar salida del equipo"
                                >
                                  <LogOut className="h-3 w-3" />
                                  Registrar Salida
                                </Button>
                              )}
                            </div>

                            {/* Grupo de botones de acción (derecha) - Editar, Ver PDF, Borrar */}
                            <div className="flex gap-1 flex-wrap">
                              {/* Botón PDF - siempre visible, deshabilitado si no hay cotización ni informe */}
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => handleVerInformePdf(orden)}
                                disabled={
                                  !orden.cotizacion_id && !orden.informe_id
                                }
                                className="text-purple-600 hover:text-purple-700 hover:bg-purple-50 disabled:opacity-50 disabled:cursor-not-allowed"
                                title={
                                  orden.cotizacion_id || orden.informe_id
                                    ? "Ver PDF (Cotización e Informe)"
                                    : "No hay documentos disponibles"
                                }
                              >
                                <FileText className="h-3 w-3" />
                              </Button>

                              {/* Botón editar - siempre visible */}
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => handleEditOrden(orden)}
                                className="text-gray-600 hover:text-gray-700"
                                title="Editar orden"
                              >
                                <Edit className="h-3 w-3" />
                              </Button>

                              {/* Botón eliminar - solo para admin y técnico */}
                              {actions.showDeleteOrden && (
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => handleDeleteOrden(orden)}
                                  className="text-red-600 hover:text-red-700"
                                  title="Eliminar orden"
                                >
                                  <Trash2 className="h-3 w-3" />
                                </Button>
                              )}
                            </div>
                          </>
                        );
                      })()}
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
        onSendToClient={async () => {
          // El backend ya maneja el cambio de estado automáticamente
          // Solo recargamos las órdenes para reflejar los cambios
          loadOrdenes();
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
      {/* PDF Viewer para Informes */}
      {(pdfInformeId || pdfCotizacionId) && (
        <PdfViewer
          open={showInformePdfViewer}
          onOpenChange={setShowInformePdfViewer}
          title={`Documentos - Orden ${pdfOrdenCodigo}`}
          informeId={pdfInformeId || undefined}
          cotizacionId={pdfCotizacionId || undefined}
        />
      )}
      {/* Diálogo para Registrar Salida de Equipo */}
      <RegistrarSalidaDialog
        open={showRegistrarSalidaDialog}
        onOpenChange={setShowRegistrarSalidaDialog}
        equipo={selectedEquipoForSalida}
        ordenTrabajo={selectedOrdenForSalida}
        onSalidaRegistrada={handleSalidaRegistrada}
      />
    </div>
  );
}
