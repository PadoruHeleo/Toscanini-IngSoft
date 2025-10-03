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
import {
  Search,
  Plus,
  Eye,
  Trash2,
  Edit,
  CheckCircle,
  XCircle,
  FileText,
} from "lucide-react";
import CotizacionFormDialog from "./CotizacionFormDialog";
import { PdfViewer } from "@/components/PdfViewer";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";

interface Cotizacion {
  cotizacion_id: number;
  cotizacion_codigo?: string;
  costo_revision?: number;
  costo_reparacion?: number;
  costo_total?: number;
  is_aprobada?: boolean;
  is_borrador?: boolean;
  informe: string;
  created_by?: number;
  created_at?: string;
}

interface CotizacionDetallada extends Cotizacion {
  orden_codigo?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  cliente_nombre?: string;
  cantidad_piezas?: number;
}

const getEstadoStyles = (cotizacion: Cotizacion) => {
  if (cotizacion.is_aprobada) {
    return "bg-green-100 text-green-800 border border-green-200 hover:bg-green-200";
  } else if (cotizacion.is_borrador) {
    return "bg-gray-100 text-gray-800 border border-gray-200 hover:bg-gray-200";
  } else {
    return "bg-yellow-100 text-yellow-800 border border-yellow-200 hover:bg-yellow-200";
  }
};

const getEstadoText = (cotizacion: Cotizacion) => {
  if (cotizacion.is_aprobada) {
    return "✅ Aprobada";
  } else if (cotizacion.is_borrador) {
    return "📝 Borrador";
  } else {
    return "⏳ Pendiente";
  }
};

export function CotizacionesView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [cotizaciones, setCotizaciones] = useState<CotizacionDetallada[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingCotizacion, setEditingCotizacion] = useState<Cotizacion | null>(
    null
  );

  // Estados para PDF Viewer
  const [showPdfViewer, setShowPdfViewer] = useState(false);
  const [pdfCotizacionId, setPdfCotizacionId] = useState<number | null>(null);
  const [pdfCotizacionCodigo, setPdfCotizacionCodigo] = useState<string>("");

  const loadCotizaciones = async () => {
    try {
      setLoading(true);
      const cotizacionesData = await invoke<CotizacionDetallada[]>(
        "get_cotizaciones_detalladas"
      );
      setCotizaciones(cotizacionesData);
    } catch (error) {
      console.error("Error cargando cotizaciones:", error);
      showError(
        "Error al cargar cotizaciones",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCotizaciones();
  }, []);

  const handleCotizacionAdded = () => {
    loadCotizaciones();
    setShowAddForm(false);
  };

  const handleCotizacionUpdated = () => {
    loadCotizaciones();
    setEditingCotizacion(null);
  };

  const handleEditCotizacion = (cotizacion: Cotizacion) => {
    setEditingCotizacion(cotizacion);
  };

  const handleVerPdf = (cotizacion: CotizacionDetallada) => {
    setPdfCotizacionId(cotizacion.cotizacion_id);
    setPdfCotizacionCodigo(cotizacion.cotizacion_codigo || "Sin código");
    setShowPdfViewer(true);
  };

  const handleDeleteCotizacion = async (cotizacion: Cotizacion) => {
    if (!user) return;

    const confirmed = window.confirm(
      `¿Está seguro de que desea eliminar la cotización ${cotizacion.cotizacion_codigo}?`
    );

    if (!confirmed) return;

    try {
      const result = await invoke<boolean>("delete_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
      });

      if (result) {
        success(
          "Cotización eliminada",
          `La cotización ${cotizacion.cotizacion_codigo} ha sido eliminada exitosamente.`
        );
        loadCotizaciones();
      } else {
        showError("Error", "No se pudo eliminar la cotización.");
      }
    } catch (error) {
      console.error("Error eliminando cotización:", error);
      showError(
        "Error al eliminar cotización",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    }
  };

  const handleToggleAprobacion = async (cotizacion: Cotizacion) => {
    if (!user) return;

    try {
      const updateData = {
        is_aprobada: !cotizacion.is_aprobada,
        is_borrador: false, // Si se aprueba o rechaza, ya no es borrador
      };

      const result = await invoke<boolean>("update_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        request: updateData,
      });

      if (result) {
        const newStatus = updateData.is_aprobada ? "aprobada" : "rechazada";
        success(
          "Estado actualizado",
          `La cotización ${cotizacion.cotizacion_codigo} ha sido ${newStatus}.`
        );
        loadCotizaciones();
      } else {
        showError("Error", "No se pudo actualizar el estado de la cotización.");
      }
    } catch (error) {
      console.error("Error actualizando estado:", error);
      showError(
        "Error al actualizar estado",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    }
  };

  const handleVerDetalles = async (cotizacion: Cotizacion) => {
    try {
      // Abrir el formulario en modo solo lectura/vista
      setEditingCotizacion(cotizacion);
    } catch (error) {
      showError("Error", "No se pudo abrir el detalle de la cotización.");
    }
  };

  // Filtrar cotizaciones según el término de búsqueda
  const filteredCotizaciones = cotizaciones.filter((cotizacion) => {
    const searchLower = searchTerm.toLowerCase();
    return (
      cotizacion.cotizacion_codigo?.toLowerCase().includes(searchLower) ||
      cotizacion.orden_codigo?.toLowerCase().includes(searchLower) ||
      cotizacion.cliente_nombre?.toLowerCase().includes(searchLower) ||
      cotizacion.equipo_marca?.toLowerCase().includes(searchLower) ||
      cotizacion.equipo_modelo?.toLowerCase().includes(searchLower)
    );
  });

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando cotizaciones...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>
          <Plus className="h-4 w-4 mr-2" />
          Crear Cotización
        </Button>
      </div>
      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Buscar cotizaciones..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
          />
        </div>
      </div>
      {/* Tabla de cotizaciones */}
      <div className="border rounded-md">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Código</TableHead>
              <TableHead>Orden de Trabajo</TableHead>
              <TableHead>Cliente</TableHead>
              <TableHead>Equipo</TableHead>
              <TableHead>Costo Total</TableHead>
              <TableHead>Estado</TableHead>
              <TableHead>Piezas</TableHead>
              <TableHead>Fecha Creación</TableHead>
              <TableHead className="w-32">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredCotizaciones.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={9}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron cotizaciones que coincidan con la búsqueda"
                    : "No hay cotizaciones registradas"}
                </TableCell>
              </TableRow>
            ) : (
              filteredCotizaciones.map((cotizacion) => (
                <TableRow key={cotizacion.cotizacion_id}>
                  <TableCell className="font-medium">
                    {cotizacion.cotizacion_codigo || "N/A"}
                  </TableCell>
                  <TableCell>{cotizacion.orden_codigo || "N/A"}</TableCell>
                  <TableCell>{cotizacion.cliente_nombre || "N/A"}</TableCell>
                  <TableCell>
                    {cotizacion.equipo_marca && cotizacion.equipo_modelo
                      ? `${cotizacion.equipo_marca} ${cotizacion.equipo_modelo}`
                      : "N/A"}
                  </TableCell>
                  <TableCell className="font-semibold">
                    ${cotizacion.costo_total || 0}
                  </TableCell>
                  <TableCell>
                    <span
                      className={`px-2 py-1 rounded-full text-xs font-medium ${getEstadoStyles(
                        cotizacion
                      )}`}
                    >
                      {getEstadoText(cotizacion)}
                    </span>
                  </TableCell>
                  <TableCell className="text-center">
                    {cotizacion.cantidad_piezas || 0}
                  </TableCell>
                  <TableCell>
                    {cotizacion.created_at
                      ? new Date(cotizacion.created_at).toLocaleDateString()
                      : "N/A"}
                  </TableCell>{" "}
                  <TableCell>
                    <div className="flex gap-1 justify-end">
                      {/* Ver PDF */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleVerPdf(cotizacion)}
                        className="text-purple-600 hover:text-purple-700 hover:bg-purple-50"
                        title="Ver PDF"
                      >
                        <FileText className="h-3 w-3" />
                      </Button>

                      {/* Ver detalles */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleVerDetalles(cotizacion)}
                        className="text-blue-600 hover:text-blue-700"
                        title="Ver detalles"
                      >
                        <Eye className="h-3 w-3" />
                      </Button>

                      {/* Editar */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditCotizacion(cotizacion)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar cotización"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>

                      {/* Toggle aprobación */}
                      {!cotizacion.is_borrador && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleToggleAprobacion(cotizacion)}
                          className={
                            cotizacion.is_aprobada
                              ? "text-red-600 hover:text-red-700"
                              : "text-green-600 hover:text-green-700"
                          }
                          title={
                            cotizacion.is_aprobada ? "Rechazar" : "Aprobar"
                          }
                        >
                          {cotizacion.is_aprobada ? (
                            <XCircle className="h-3 w-3" />
                          ) : (
                            <CheckCircle className="h-3 w-3" />
                          )}
                        </Button>
                      )}

                      {/* Eliminar */}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleDeleteCotizacion(cotizacion)}
                        className="text-red-600 hover:text-red-700"
                        title="Eliminar cotización"
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
      {/* Total de cotizaciones */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {filteredCotizaciones.length} cotización
        {filteredCotizaciones.length !== 1 ? "es" : ""}
        {searchTerm && ` (filtrado de ${cotizaciones.length})`}
      </div>
      {/* Dialog para agregar cotización */}
      <CotizacionFormDialog
        open={showAddForm}
        onOpenChange={setShowAddForm}
        onCotizacionAdded={handleCotizacionAdded}
      />
      {/* Dialog para editar cotización */}
      {editingCotizacion && (
        <CotizacionFormDialog
          open={!!editingCotizacion}
          onOpenChange={(open: boolean) => !open && setEditingCotizacion(null)}
          onCotizacionAdded={handleCotizacionUpdated}
          cotizacion={editingCotizacion}
          isEditing={true}
          onSendToClient={(cotizacionId) => {
            console.log(`Cotización ${cotizacionId} enviada al cliente`);
            loadCotizaciones(); // Recargar la lista para ver el cambio de estado
          }}
        />
      )}

      {/* PDF Viewer */}
      {pdfCotizacionId && (
        <PdfViewer
          open={showPdfViewer}
          onOpenChange={setShowPdfViewer}
          title={`Documentos - Cotización ${pdfCotizacionCodigo}`}
          cotizacionId={pdfCotizacionId}
        />
      )}
    </div>
  );
}
