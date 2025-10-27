import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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
  Filter,
  Download,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { ViewTitle } from "@/components/ViewTitle";
import { usePermissions } from "@/hooks/use-permissions";
import { AccessDenied } from "@/components/AccessDenied";
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

interface FiltrosSalida {
  fechaDesde: string;
  fechaHasta: string;
  motivo: string;
  cliente: string;
  orden: string;
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
  const [filteredSalidas, setFilteredSalidas] = useState<SalidaEquipo[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedSalida, setSelectedSalida] = useState<SalidaEquipo | null>(
    null
  );
  const [showDetailDialog, setShowDetailDialog] = useState(false);
  const [filtros, setFiltros] = useState<FiltrosSalida>({
    fechaDesde: "",
    fechaHasta: "",
    motivo: "",
    cliente: "",
    orden: "",
  });
  const [showFilters, setShowFilters] = useState(false);

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
      setFilteredSalidas(result);
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

  // Aplicar filtros
  useEffect(() => {
    let filtered = [...salidas];

    if (filtros.fechaDesde) {
      filtered = filtered.filter(
        (s) => !s.fecha_salida || s.fecha_salida >= filtros.fechaDesde
      );
    }

    if (filtros.fechaHasta) {
      filtered = filtered.filter(
        (s) => !s.fecha_salida || s.fecha_salida <= filtros.fechaHasta
      );
    }

    if (filtros.motivo) {
      filtered = filtered.filter((s) => s.motivo_salida === filtros.motivo);
    }

    if (filtros.cliente) {
      filtered = filtered.filter((s) =>
        s.cliente_nombre?.toLowerCase().includes(filtros.cliente.toLowerCase())
      );
    }

    if (filtros.orden) {
      filtered = filtered.filter((s) =>
        s.orden_codigo?.toLowerCase().includes(filtros.orden.toLowerCase())
      );
    }

    setFilteredSalidas(filtered);
  }, [filtros, salidas]);

  const handleShowDetail = (salida: SalidaEquipo) => {
    setSelectedSalida(salida);
    setShowDetailDialog(true);
  };

  const clearFilters = () => {
    setFiltros({
      fechaDesde: "",
      fechaHasta: "",
      motivo: "",
      cliente: "",
      orden: "",
    });
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

  const exportSalidas = () => {
    const csvContent = [
      [
        "Fecha Salida",
        "Orden",
        "Equipo",
        "Cliente",
        "Motivo",
        "Usuario",
        "Observaciones",
      ].join(","),
      ...filteredSalidas.map((salida) =>
        [
          formatDate(salida.fecha_salida),
          salida.orden_codigo || "",
          salida.equipo_nombre || "",
          salida.cliente_nombre || "",
          motivosMap[salida.motivo_salida as keyof typeof motivosMap] ||
            salida.motivo_salida,
          salida.usuario_nombre || "",
          (salida.observaciones || "").replace(/,/g, ";"),
        ].join(",")
      ),
    ].join("\n");

    const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
    const link = document.createElement("a");
    const url = URL.createObjectURL(blob);
    link.setAttribute("href", url);
    link.setAttribute(
      "download",
      `salidas_equipos_${new Date().toISOString().split("T")[0]}.csv`
    );
    link.style.visibility = "hidden";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  return (
    <div className="space-y-6">
      <ViewTitle />

      {/* Título personalizado para esta vista */}
      <div className="border-b pb-4">
        <div className="flex items-center space-x-3">
          <Package className="h-8 w-8 text-primary" />
          <div>
            <h1 className="text-3xl font-bold tracking-tight">
              Salidas de Equipos
            </h1>
            <p className="text-muted-foreground">
              Historial completo de todas las salidas de equipos del sistema
            </p>
          </div>
        </div>
      </div>

      {/* Estadísticas rápidas */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Salidas</CardTitle>
            <Package className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{filteredSalidas.length}</div>
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
                filteredSalidas.filter(
                  (s) => s.motivo_salida === "entregado_cliente"
                ).length
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
              {
                filteredSalidas.filter((s) => s.motivo_salida === "abandonado")
                  .length
              }
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
                filteredSalidas.filter(
                  (s) => s.motivo_salida === "retirado_sin_reparacion"
                ).length
              }
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Controles y filtros */}
      <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
        <div className="flex gap-2">
          <Button
            onClick={() => setShowFilters(!showFilters)}
            variant="outline"
            size="sm"
          >
            <Filter className="h-4 w-4 mr-2" />
            {showFilters ? "Ocultar Filtros" : "Mostrar Filtros"}
          </Button>
          <Button
            onClick={exportSalidas}
            variant="outline"
            size="sm"
            disabled={filteredSalidas.length === 0}
          >
            <Download className="h-4 w-4 mr-2" />
            Exportar CSV
          </Button>
        </div>

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

      {/* Panel de filtros */}
      {showFilters && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Filtros de Búsqueda</CardTitle>
            <CardDescription>
              Filtra las salidas por fecha, motivo, cliente u orden de trabajo
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-4">
              <div className="space-y-2">
                <Label>Fecha Desde</Label>
                <Input
                  type="date"
                  value={filtros.fechaDesde}
                  onChange={(e) =>
                    setFiltros({ ...filtros, fechaDesde: e.target.value })
                  }
                />
              </div>

              <div className="space-y-2">
                <Label>Fecha Hasta</Label>
                <Input
                  type="date"
                  value={filtros.fechaHasta}
                  onChange={(e) =>
                    setFiltros({ ...filtros, fechaHasta: e.target.value })
                  }
                />
              </div>

              <div className="space-y-2">
                <Label>Motivo</Label>
                <Select
                  value={filtros.motivo}
                  onValueChange={(value) =>
                    setFiltros({ ...filtros, motivo: value })
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Todos los motivos" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">Todos los motivos</SelectItem>
                    <SelectItem value="entregado_cliente">
                      Entregado al Cliente
                    </SelectItem>
                    <SelectItem value="retirado_sin_reparacion">
                      Retirado sin Reparación
                    </SelectItem>
                    <SelectItem value="abandonado">Abandonado</SelectItem>
                    <SelectItem value="baja_definitiva">
                      Baja Definitiva
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Cliente</Label>
                <Input
                  placeholder="Buscar cliente..."
                  value={filtros.cliente}
                  onChange={(e) =>
                    setFiltros({ ...filtros, cliente: e.target.value })
                  }
                />
              </div>

              <div className="space-y-2">
                <Label>Orden</Label>
                <Input
                  placeholder="Buscar orden..."
                  value={filtros.orden}
                  onChange={(e) =>
                    setFiltros({ ...filtros, orden: e.target.value })
                  }
                />
              </div>
            </div>

            <div className="flex justify-end mt-4">
              <Button onClick={clearFilters} variant="outline" size="sm">
                Limpiar Filtros
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Tabla de salidas */}
      <Card>
        <CardHeader>
          <CardTitle>Registro de Salidas</CardTitle>
          <CardDescription>
            Mostrando {filteredSalidas.length} de {salidas.length} salidas
          </CardDescription>
        </CardHeader>
        <CardContent>
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
                ) : filteredSalidas.length === 0 ? (
                  <TableRow>
                    <TableCell
                      colSpan={7}
                      className="text-center py-8 text-muted-foreground"
                    >
                      No se encontraron salidas con los filtros aplicados
                    </TableCell>
                  </TableRow>
                ) : (
                  filteredSalidas.map((salida) => (
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
