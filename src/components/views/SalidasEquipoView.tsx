import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  RefreshCw,
  Calendar,
  FileText,
  User,
  Package,
  Eye,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { ViewTitle } from "@/components/layout/ViewTitle";
import { usePermissions } from "@/hooks/use-permissions";
import { AccessDenied } from "@/components/common/AccessDenied";
import { useToastContext } from "@/contexts/ToastContext";

interface SalidaEquipo {
  salida_id: number;
  orden_trabajo_id: number;
  motivo_salida: string;
  fecha_salida?: string;
  usuario_id?: number;
  observaciones?: string;
  created_at?: string;
  // Campos JOIN
  orden_codigo?: string;
  equipo_nombre?: string;
  cliente_nombre?: string;
  usuario_nombre?: string;
}

const motivosMap = {
  entregado_cliente: "Entregado al Cliente",
  retirado_sin_reparacion: "Retirado sin Reparación",
  abandonado: "Abandonado",
  baja_definitiva: "Baja Definitiva",
};

const motivoBadgeColors = {
  entregado_cliente: "bg-green-500",
  retirado_sin_reparacion: "bg-yellow-500",
  abandonado: "bg-red-500",
  baja_definitiva: "bg-gray-500",
};

export function SalidasEquipoView() {
  const [salidas, setSalidas] = useState<SalidaEquipo[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedSalida, setSelectedSalida] = useState<SalidaEquipo | null>(
    null
  );
  const [showDetailDialog, setShowDetailDialog] = useState(false);

  const { isAdmin } = usePermissions();
  const { error } = useToastContext();

  // Verificar permisos de administrador
  if (!isAdmin()) {
    return <AccessDenied />;
  }

  const loadSalidas = async () => {
    setLoading(true);
    try {
      const result = await invoke<SalidaEquipo[]>("get_salidas_equipo");
      setSalidas(result);
    } catch (err) {
      console.error("Error cargando salidas:", err);
      error("Error al cargar las salidas de equipos");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSalidas();
  }, []);

  const handleShowDetail = (salida: SalidaEquipo) => {
    setSelectedSalida(salida);
    setShowDetailDialog(true);
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return "N/A";
    try {
      return new Date(dateString).toLocaleDateString("es-ES", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return "N/A";
    }
  };

  return (
    <div className="space-y-6 px-6 pt-6">
      <ViewTitle />

      {/* Estadísticas rápidas */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Salidas</CardTitle>
            <Package className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{salidas.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Entregados</CardTitle>
            <Package className="h-4 w-4 text-green-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {
                salidas.filter((s) => s.motivo_salida === "entregado_cliente")
                  .length
              }
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Abandonados</CardTitle>
            <Package className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {salidas.filter((s) => s.motivo_salida === "abandonado").length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Sin Reparación
            </CardTitle>
            <Package className="h-4 w-4 text-yellow-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {
                salidas.filter(
                  (s) => s.motivo_salida === "retirado_sin_reparacion"
                ).length
              }
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Controles y filtros */}
      <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
        <div className="flex gap-2"></div>

        <Button
          onClick={loadSalidas}
          disabled={loading}
          size="sm"
          variant="outline"
        >
          <RefreshCw
            className={`h-4 w-4 mr-2 ${loading ? "animate-spin" : ""}`}
          />
          Actualizar
        </Button>
      </div>

      {/* Tabla de salidas */}
      <Card>
        <CardHeader>
          <CardTitle>Registro de Salidas</CardTitle>
          <CardDescription>Mostrando {salidas.length} salidas</CardDescription>
        </CardHeader>
        <CardContent className="px-6">
          <div className="rounded-md border overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Fecha Salida</TableHead>
                  <TableHead>Orden</TableHead>
                  <TableHead>Equipo</TableHead>
                  <TableHead>Cliente</TableHead>
                  <TableHead>Motivo</TableHead>
                  <TableHead>Usuario</TableHead>
                  <TableHead className="text-right">Acciones</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {loading ? (
                  <TableRow>
                    <TableCell colSpan={7} className="text-center py-8">
                      <RefreshCw className="h-4 w-4 animate-spin mx-auto mb-2" />
                      Cargando salidas...
                    </TableCell>
                  </TableRow>
                ) : salidas.length === 0 ? (
                  <TableRow>
                    <TableCell
                      colSpan={7}
                      className="text-center py-8 text-muted-foreground"
                    >
                      No se encontraron salidas
                    </TableCell>
                  </TableRow>
                ) : (
                  salidas.map((salida) => (
                    <TableRow key={salida.salida_id}>
                      <TableCell>
                        <div className="flex items-center space-x-2">
                          <Calendar className="h-4 w-4 text-muted-foreground" />
                          <span>{formatDate(salida.fecha_salida)}</span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge variant="outline">
                          {salida.orden_codigo ||
                            `ID: ${salida.orden_trabajo_id}`}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <div className="font-medium">
                          {salida.equipo_nombre || "N/A"}
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="font-medium">
                          {salida.cliente_nombre || "N/A"}
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge
                          className={`text-white ${
                            motivoBadgeColors[
                              salida.motivo_salida as keyof typeof motivoBadgeColors
                            ] || "bg-gray-500"
                          }`}
                        >
                          {motivosMap[
                            salida.motivo_salida as keyof typeof motivosMap
                          ] || salida.motivo_salida}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center space-x-2">
                          <User className="h-4 w-4 text-muted-foreground" />
                          <span>{salida.usuario_nombre || "Sistema"}</span>
                        </div>
                      </TableCell>
                      <TableCell className="text-right">
                        <Button
                          onClick={() => handleShowDetail(salida)}
                          variant="ghost"
                          size="sm"
                        >
                          <Eye className="h-4 w-4" />
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>

      {/* Dialog de detalles */}
      <Dialog open={showDetailDialog} onOpenChange={setShowDetailDialog}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle className="flex items-center space-x-2">
              <FileText className="h-5 w-5" />
              <span>Detalles de la Salida</span>
            </DialogTitle>
          </DialogHeader>

          {selectedSalida && (
            <div className="space-y-6">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    ID Salida
                  </Label>
                  <div className="text-sm">{selectedSalida.salida_id}</div>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Fecha de Salida
                  </Label>
                  <div className="text-sm">
                    {formatDate(selectedSalida.fecha_salida)}
                  </div>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Orden de Trabajo
                  </Label>
                  <Badge variant="outline">
                    {selectedSalida.orden_codigo ||
                      `ID: ${selectedSalida.orden_trabajo_id}`}
                  </Badge>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Motivo de Salida
                  </Label>
                  <Badge
                    className={`text-white ${
                      motivoBadgeColors[
                        selectedSalida.motivo_salida as keyof typeof motivoBadgeColors
                      ] || "bg-gray-500"
                    }`}
                  >
                    {motivosMap[
                      selectedSalida.motivo_salida as keyof typeof motivosMap
                    ] || selectedSalida.motivo_salida}
                  </Badge>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Equipo
                  </Label>
                  <div className="text-sm font-medium">
                    {selectedSalida.equipo_nombre || "N/A"}
                  </div>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Cliente
                  </Label>
                  <div className="text-sm font-medium">
                    {selectedSalida.cliente_nombre || "N/A"}
                  </div>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Usuario Responsable
                  </Label>
                  <div className="text-sm">
                    {selectedSalida.usuario_nombre || "Sistema"}
                  </div>
                </div>

                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Fecha de Registro
                  </Label>
                  <div className="text-sm">
                    {formatDate(selectedSalida.created_at)}
                  </div>
                </div>
              </div>

              {selectedSalida.observaciones && (
                <div className="space-y-2">
                  <Label className="text-sm font-medium text-muted-foreground">
                    Observaciones
                  </Label>
                  <div className="text-sm p-3 bg-muted rounded-md">
                    {selectedSalida.observaciones}
                  </div>
                </div>
              )}
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
