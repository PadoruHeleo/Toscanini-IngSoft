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
  DialogFooter,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Plus,
  Minus,
  RefreshCw,
  Radio,
  Wrench,
  AlertTriangle,
  CheckCircle,
  Edit,
  Trash2,
  Package,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { ViewTitle } from "@/components/layout/ViewTitle";
import { useInventarioEquipoPermissions } from "@/hooks/use-permissions";
import { AccessDenied } from "@/components/common/AccessDenied";
import { useToastContext } from "@/contexts/ToastContext";

interface InventarioEquipo {
  inventario_equipo_id: number;
  equipo_codigo?: string;
  equipo_nombre?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_descripcion?: string;
  equipo_precio?: number;
  equipo_stock?: number;
  equipo_estado?: string;
  equipo_ubicacion?: string;
  fecha_adquisicion?: string;
  proveedor?: string;
  numero_serie?: string;
  garantia_vencimiento?: string;
  observaciones?: string;
  created_at?: string;
}

interface StockUpdateData {
  inventario_equipo_id: number;
  cantidad: number;
  tipo: "entrada" | "salida";
}

interface EquipoFormData {
  equipo_codigo: string;
  equipo_nombre: string;
  equipo_marca: string;
  equipo_modelo: string;
  equipo_tipo: string;
  equipo_descripcion: string;
  equipo_precio: string;
  equipo_stock: string;
  equipo_ubicacion: string;
  proveedor: string;
  numero_serie: string;
  observaciones: string;
}

export function InventarioEquiposView() {
  const { canViewEquipment } = useInventarioEquipoPermissions();
  const { success, error } = useToastContext();
  const [equipos, setEquipos] = useState<InventarioEquipo[]>([]);
  const [loading, setLoading] = useState(true);
  const [showStockDialog, setShowStockDialog] = useState(false);
  const [showEditDialog, setShowEditDialog] = useState(false);
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [editingEquipo, setEditingEquipo] = useState<InventarioEquipo | null>(
    null
  );
  const [selectedEquipo, setSelectedEquipo] = useState<InventarioEquipo | null>(
    null
  );
  const [stockUpdate, setStockUpdate] = useState<StockUpdateData>({
    inventario_equipo_id: 0,
    cantidad: 0,
    tipo: "entrada",
  });
  const [formData, setFormData] = useState<EquipoFormData>({
    equipo_codigo: "",
    equipo_nombre: "",
    equipo_marca: "",
    equipo_modelo: "",
    equipo_tipo: "",
    equipo_descripcion: "",
    equipo_precio: "",
    equipo_stock: "",
    equipo_ubicacion: "",
    proveedor: "",
    numero_serie: "",
    observaciones: "",
  });
  const [errors, setErrors] = useState<Partial<EquipoFormData>>({});
  const [searchTerm, setSearchTerm] = useState("");

  const loadEquipos = async () => {
    setLoading(true);
    try {
      const data = await invoke<InventarioEquipo[]>("get_inventario_equipos");
      console.log("Datos de inventario equipos cargados:", data);
      setEquipos(data);
    } catch (e) {
      console.error("Error loading equipos inventario:", e);
      // Fallback con datos simulados para desarrollo
      setEquipos([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadEquipos();
  }, []);

  const handleOpenStockDialog = (
    equipo: InventarioEquipo,
    tipo: "entrada" | "salida"
  ) => {
    setSelectedEquipo(equipo);
    setStockUpdate({
      inventario_equipo_id: equipo.inventario_equipo_id,
      cantidad: 1,
      tipo: tipo,
    });
    setShowStockDialog(true);
  };

  const handleCloseStockDialog = () => {
    setShowStockDialog(false);
    setSelectedEquipo(null);
    setStockUpdate({ inventario_equipo_id: 0, cantidad: 0, tipo: "entrada" });
  };

  const handleOpenCreateDialog = () => {
    setFormData({
      equipo_codigo: "",
      equipo_nombre: "",
      equipo_marca: "",
      equipo_modelo: "",
      equipo_tipo: "",
      equipo_descripcion: "",
      equipo_precio: "",
      equipo_stock: "0",
      equipo_ubicacion: "",
      proveedor: "",
      numero_serie: "",
      observaciones: "",
    });
    setErrors({});
    setShowCreateDialog(true);
  };

  const handleOpenEditDialog = (equipo: InventarioEquipo) => {
    setEditingEquipo(equipo);
    setFormData({
      equipo_codigo: equipo.equipo_codigo || "",
      equipo_nombre: equipo.equipo_nombre || "",
      equipo_marca: equipo.equipo_marca || "",
      equipo_modelo: equipo.equipo_modelo || "",
      equipo_tipo: equipo.equipo_tipo || "",
      equipo_descripcion: equipo.equipo_descripcion || "",
      equipo_precio: equipo.equipo_precio?.toString() || "",
      equipo_stock: equipo.equipo_stock?.toString() || "",
      equipo_ubicacion: equipo.equipo_ubicacion || "",
      proveedor: equipo.proveedor || "",
      numero_serie: equipo.numero_serie || "",
      observaciones: equipo.observaciones || "",
    });
    setErrors({});
    setShowEditDialog(true);
  };

  const handleCloseEditDialog = () => {
    setShowEditDialog(false);
    setShowCreateDialog(false);
    setEditingEquipo(null);
    setErrors({});
  };

  const validateForm = () => {
    const newErrors: Partial<EquipoFormData> = {};
    if (!formData.equipo_codigo.trim())
      newErrors.equipo_codigo = "El código es requerido";
    if (!formData.equipo_nombre.trim())
      newErrors.equipo_nombre = "El nombre es requerido";
    if (!formData.equipo_tipo.trim())
      newErrors.equipo_tipo = "El tipo es requerido";
    if (formData.equipo_precio && isNaN(Number(formData.equipo_precio)))
      newErrors.equipo_precio = "Debe ser un número";
    if (formData.equipo_stock && isNaN(Number(formData.equipo_stock)))
      newErrors.equipo_stock = "Debe ser un número";
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    try {
      if (editingEquipo) {
        await invoke("update_inventario_equipo", {
          equipoId: editingEquipo.inventario_equipo_id,
          request: {
            equipo_codigo: formData.equipo_codigo,
            equipo_nombre: formData.equipo_nombre,
            equipo_marca: formData.equipo_marca,
            equipo_modelo: formData.equipo_modelo,
            equipo_tipo: formData.equipo_tipo,
            equipo_descripcion: formData.equipo_descripcion,
            equipo_precio: formData.equipo_precio
              ? Number(formData.equipo_precio)
              : null,
            equipo_stock: formData.equipo_stock
              ? Number(formData.equipo_stock)
              : 0,
            equipo_ubicacion: formData.equipo_ubicacion,
            proveedor: formData.proveedor,
            numero_serie: formData.numero_serie,
            observaciones: formData.observaciones,
          },
        });

        // Toast de éxito para actualizar
        success(
          "Equipo actualizado exitosamente",
          `Se actualizó el equipo "${formData.equipo_nombre}"`
        );
      } else {
        await invoke("create_inventario_equipo", {
          request: {
            equipo_codigo: formData.equipo_codigo,
            equipo_nombre: formData.equipo_nombre,
            equipo_marca: formData.equipo_marca,
            equipo_modelo: formData.equipo_modelo,
            equipo_tipo: formData.equipo_tipo,
            equipo_descripcion: formData.equipo_descripcion,
            equipo_precio: formData.equipo_precio
              ? Number(formData.equipo_precio)
              : null,
            equipo_stock: formData.equipo_stock
              ? Number(formData.equipo_stock)
              : 0,
            equipo_ubicacion: formData.equipo_ubicacion,
            proveedor: formData.proveedor,
            numero_serie: formData.numero_serie,
            observaciones: formData.observaciones,
          },
        });

        // Toast de éxito para crear
        success(
          "Equipo creado exitosamente",
          `Se agregó el equipo "${formData.equipo_nombre}" al inventario`
        );
      }
      loadEquipos();
      handleCloseEditDialog();
    } catch (e) {
      console.error("Error saving equipo:", e);

      // Toast de error
      const accion = editingEquipo ? "actualizar" : "crear";
      error(
        `Error al ${accion} equipo`,
        `No se pudo ${accion} el equipo en el inventario`
      );
    }
  };

  const handleDelete = async (equipo: InventarioEquipo) => {
    if (!window.confirm(`¿Eliminar el equipo "${equipo.equipo_nombre}"?`))
      return;
    try {
      await invoke("delete_inventario_equipo", {
        equipoId: equipo.inventario_equipo_id,
      });

      // Toast de éxito
      success(
        "Equipo eliminado exitosamente",
        `Se eliminó el equipo "${equipo.equipo_nombre}" del inventario`
      );

      loadEquipos();
    } catch (e) {
      console.error("Error deleting equipo:", e);

      // Toast de error
      error(
        "Error al eliminar equipo",
        "No se pudo eliminar el equipo del inventario"
      );
    }
  };

  const handleStockUpdate = async () => {
    if (!selectedEquipo) return;

    try {
      await invoke("update_inventario_equipo_stock", {
        equipoId: stockUpdate.inventario_equipo_id,
        cantidad: stockUpdate.cantidad,
        tipo: stockUpdate.tipo,
      });

      // Toast de éxito
      const accion = stockUpdate.tipo === "entrada" ? "agregado" : "reducido";
      const cantidad = stockUpdate.cantidad;
      const nombre = selectedEquipo.equipo_nombre || "Equipo";

      success(
        `Stock ${accion} exitosamente`,
        `Se ${
          stockUpdate.tipo === "entrada" ? "agregaron" : "redujeron"
        } ${cantidad} unidad${cantidad > 1 ? "es" : ""} de ${nombre}`
      );

      loadEquipos();
      handleCloseStockDialog();
    } catch (e) {
      console.error("Error updating stock:", e);

      // Toast de error
      error(
        "Error al actualizar stock",
        "No se pudo actualizar el stock del equipo"
      );

      handleCloseStockDialog();
    }
  };

  const getStockStatus = (stock: number) => {
    if (stock === 0) {
      return {
        label: "Sin Stock",
        className:
          "bg-red-100 text-red-800 border border-red-200 hover:bg-red-200",
        icon: AlertTriangle,
        textColor: "text-red-600",
      };
    }
    if (stock <= 2) {
      return {
        label: "Stock Bajo",
        className:
          "bg-amber-100 text-amber-800 border border-amber-200 hover:bg-amber-200",
        icon: AlertTriangle,
        textColor: "text-amber-600",
      };
    }
    if (stock <= 10) {
      return {
        label: "Stock Medio",
        className:
          "bg-blue-100 text-blue-800 border border-blue-200 hover:bg-blue-200",
        icon: Package,
        textColor: "text-blue-600",
      };
    }
    return {
      label: "Stock Alto",
      className:
        "bg-emerald-100 text-emerald-800 border border-emerald-200 hover:bg-emerald-200",
      icon: Package,
      textColor: "text-emerald-600",
    };
  };

  const getTipoIcon = (tipo: string) => {
    switch (tipo) {
      case "radio":
        return Radio;
      case "herramienta":
        return Wrench;
      default:
        return Package;
    }
  };

  const filteredEquipos = equipos.filter(
    (equipo) =>
      equipo.equipo_nombre?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      equipo.equipo_marca?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      equipo.equipo_modelo?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      equipo.equipo_codigo?.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleRefresh = () => {
    loadEquipos();
  };

  // Verificar permisos antes de renderizar la vista
  if (!canViewEquipment) {
    return <AccessDenied />;
  }

  return (
    <div className="px-6 pt-6 space-y-6">
      <ViewTitle onRefresh={handleRefresh} />

      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">
          Inventario de Equipos de la Empresa
        </h3>
        <p className="text-muted-foreground mb-4">
          Gestiona el inventario de equipos, herramientas y accesorios propios
          de la empresa
        </p>

        {/* Resumen del inventario */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <Package className="h-4 w-4 text-blue-600" />
              <span className="text-sm font-medium text-blue-800">
                Total Equipos
              </span>
            </div>
            <div className="text-2xl font-bold text-blue-600">
              {filteredEquipos.length}
            </div>
          </div>

          <div className="bg-emerald-50 border border-emerald-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <CheckCircle className="h-4 w-4 text-emerald-600" />
              <span className="text-sm font-medium text-emerald-800">
                Con Stock
              </span>
            </div>
            <div className="text-2xl font-bold text-emerald-600">
              {filteredEquipos.filter((e) => (e.equipo_stock || 0) > 0).length}
            </div>
          </div>

          <div className="bg-amber-50 border border-amber-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <AlertTriangle className="h-4 w-4 text-amber-600" />
              <span className="text-sm font-medium text-amber-800">
                Stock Bajo
              </span>
            </div>
            <div className="text-2xl font-bold text-amber-600">
              {
                filteredEquipos.filter(
                  (e) => (e.equipo_stock || 0) > 0 && (e.equipo_stock || 0) <= 2
                ).length
              }
            </div>
          </div>

          <div className="bg-red-50 border border-red-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <AlertTriangle className="h-4 w-4 text-red-600" />
              <span className="text-sm font-medium text-red-800">
                Sin Stock
              </span>
            </div>
            <div className="text-2xl font-bold text-red-600">
              {
                filteredEquipos.filter((e) => (e.equipo_stock || 0) === 0)
                  .length
              }
            </div>
          </div>
        </div>

        <div className="flex gap-4 mb-4">
          <Input
            placeholder="Buscar por código, nombre, marca o modelo..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="max-w-md"
          />
          <Button onClick={handleOpenCreateDialog}>
            <Plus className="h-4 w-4 mr-2" />
            Agregar Equipo
          </Button>
        </div>
      </div>

      <div className="rounded-md border bg-white">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Código</TableHead>
              <TableHead>Nombre</TableHead>
              <TableHead>Marca/Modelo</TableHead>
              <TableHead>Tipo</TableHead>
              <TableHead>Stock</TableHead>
              <TableHead>Ubicación</TableHead>
              <TableHead>Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loading ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  <RefreshCw className="h-4 w-4 animate-spin inline-block mr-2" />
                  Cargando inventario...
                </TableCell>
              </TableRow>
            ) : filteredEquipos.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron equipos que coincidan con la búsqueda"
                    : "No hay equipos en el inventario"}
                </TableCell>
              </TableRow>
            ) : (
              filteredEquipos.map((equipo) => {
                const stockStatus = getStockStatus(equipo.equipo_stock || 0);
                const TipoIcon = getTipoIcon(equipo.equipo_tipo || "");

                return (
                  <TableRow key={equipo.inventario_equipo_id}>
                    <TableCell className="font-mono text-sm font-medium">
                      {equipo.equipo_codigo}
                    </TableCell>
                    <TableCell className="font-medium">
                      {equipo.equipo_nombre}
                    </TableCell>
                    <TableCell>
                      <div className="flex flex-col">
                        <span className="font-medium">
                          {equipo.equipo_marca}
                        </span>
                        <span className="text-sm text-gray-500">
                          {equipo.equipo_modelo}
                        </span>
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <TipoIcon className="h-4 w-4 text-gray-600" />
                        <span className="capitalize">{equipo.equipo_tipo}</span>
                      </div>
                    </TableCell>
                    <TableCell className="text-center">
                      <div className="flex flex-col items-center gap-1">
                        <span
                          className={`font-mono text-lg font-semibold ${stockStatus.textColor}`}
                        >
                          {equipo.equipo_stock || 0}
                        </span>
                        <span className="text-xs text-gray-500">unidades</span>
                      </div>
                    </TableCell>
                    <TableCell className="text-sm">
                      {equipo.equipo_ubicacion}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1 justify-end">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleOpenEditDialog(equipo)}
                          className="text-blue-600 hover:text-blue-700"
                          title="Editar equipo"
                        >
                          <Edit className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() =>
                            handleOpenStockDialog(equipo, "entrada")
                          }
                          className="text-green-600 hover:text-green-700"
                          title="Agregar stock"
                        >
                          <Plus className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() =>
                            handleOpenStockDialog(equipo, "salida")
                          }
                          className="text-red-600 hover:text-red-700"
                          title="Reducir stock"
                          disabled={(equipo.equipo_stock || 0) === 0}
                        >
                          <Minus className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleDelete(equipo)}
                          className="text-red-600 hover:text-red-700"
                          title="Eliminar equipo"
                        >
                          <Trash2 className="h-3 w-3" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                );
              })
            )}
          </TableBody>
        </Table>
      </div>

      {/* Dialog para actualizar stock */}
      <Dialog open={showStockDialog} onOpenChange={setShowStockDialog}>
        <DialogContent style={{ maxWidth: 400 }}>
          <DialogHeader>
            <DialogTitle>
              {stockUpdate.tipo === "entrada"
                ? "Agregar Stock"
                : "Reducir Stock"}
            </DialogTitle>
          </DialogHeader>
          {selectedEquipo && (
            <div className="space-y-4">
              <div className="p-3 bg-gray-50 rounded-md">
                <p className="font-medium">{selectedEquipo.equipo_nombre}</p>
                <p className="text-sm text-gray-600">
                  {selectedEquipo.equipo_codigo}
                </p>
                <p className="text-sm text-gray-500">
                  Stock actual: {selectedEquipo.equipo_stock || 0}
                </p>
              </div>

              <div>
                <Label htmlFor="cantidad">
                  Cantidad a{" "}
                  {stockUpdate.tipo === "entrada" ? "agregar" : "reducir"}
                </Label>
                <Input
                  id="cantidad"
                  type="number"
                  min="1"
                  max={
                    stockUpdate.tipo === "salida"
                      ? selectedEquipo.equipo_stock || 0
                      : undefined
                  }
                  value={stockUpdate.cantidad}
                  onChange={(e) =>
                    setStockUpdate({
                      ...stockUpdate,
                      cantidad: parseInt(e.target.value) || 0,
                    })
                  }
                />
              </div>

              <div className="p-3 bg-blue-50 rounded-md">
                <p className="text-sm">
                  <strong>Nuevo stock:</strong>{" "}
                  {stockUpdate.tipo === "entrada"
                    ? (selectedEquipo.equipo_stock || 0) + stockUpdate.cantidad
                    : Math.max(
                        0,
                        (selectedEquipo.equipo_stock || 0) -
                          stockUpdate.cantidad
                      )}
                </p>
              </div>
            </div>
          )}
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={handleCloseStockDialog}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={handleStockUpdate}
              variant={
                stockUpdate.tipo === "entrada" ? "default" : "destructive"
              }
            >
              {stockUpdate.tipo === "entrada" ? "Agregar" : "Reducir"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Dialog para crear/editar equipo */}
      <Dialog
        open={showEditDialog || showCreateDialog}
        onOpenChange={handleCloseEditDialog}
      >
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>
              {editingEquipo ? "Editar Equipo" : "Agregar Nuevo Equipo"}
            </DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* Información básica */}
              <div className="space-y-4">
                <h4 className="font-medium text-lg">Información Básica</h4>

                <div>
                  <Label htmlFor="equipo_codigo">Código *</Label>
                  <Input
                    id="equipo_codigo"
                    value={formData.equipo_codigo}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_codigo: e.target.value,
                      })
                    }
                    className={errors.equipo_codigo ? "border-red-500" : ""}
                  />
                  {errors.equipo_codigo && (
                    <p className="text-sm text-red-500">
                      {errors.equipo_codigo}
                    </p>
                  )}
                </div>

                <div>
                  <Label htmlFor="equipo_nombre">Nombre *</Label>
                  <Input
                    id="equipo_nombre"
                    value={formData.equipo_nombre}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_nombre: e.target.value,
                      })
                    }
                    className={errors.equipo_nombre ? "border-red-500" : ""}
                  />
                  {errors.equipo_nombre && (
                    <p className="text-sm text-red-500">
                      {errors.equipo_nombre}
                    </p>
                  )}
                </div>

                <div>
                  <Label htmlFor="equipo_marca">Marca</Label>
                  <Input
                    id="equipo_marca"
                    value={formData.equipo_marca}
                    onChange={(e) =>
                      setFormData({ ...formData, equipo_marca: e.target.value })
                    }
                  />
                </div>

                <div>
                  <Label htmlFor="equipo_modelo">Modelo</Label>
                  <Input
                    id="equipo_modelo"
                    value={formData.equipo_modelo}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_modelo: e.target.value,
                      })
                    }
                  />
                </div>

                <div>
                  <Label htmlFor="equipo_tipo">Tipo *</Label>
                  <Select
                    value={formData.equipo_tipo}
                    onValueChange={(value) =>
                      setFormData({ ...formData, equipo_tipo: value })
                    }
                  >
                    <SelectTrigger
                      className={errors.equipo_tipo ? "border-red-500" : ""}
                    >
                      <SelectValue placeholder="Seleccionar tipo" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="radio">Radio</SelectItem>
                      <SelectItem value="antena">Antena</SelectItem>
                      <SelectItem value="repetidor">Repetidor</SelectItem>
                      <SelectItem value="herramienta">Herramienta</SelectItem>
                      <SelectItem value="accesorio">Accesorio</SelectItem>
                      <SelectItem value="otro">Otro</SelectItem>
                    </SelectContent>
                  </Select>
                  {errors.equipo_tipo && (
                    <p className="text-sm text-red-500">{errors.equipo_tipo}</p>
                  )}
                </div>

                <div>
                  <Label htmlFor="equipo_descripcion">Descripción</Label>
                  <Input
                    id="equipo_descripcion"
                    value={formData.equipo_descripcion}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_descripcion: e.target.value,
                      })
                    }
                  />
                </div>
              </div>

              {/* Información de inventario y estado */}
              <div className="space-y-4">
                <h4 className="font-medium text-lg">Inventario y Estado</h4>

                <div>
                  <Label htmlFor="equipo_precio">Precio</Label>
                  <Input
                    id="equipo_precio"
                    type="number"
                    min="0"
                    value={formData.equipo_precio}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_precio: e.target.value,
                      })
                    }
                    className={errors.equipo_precio ? "border-red-500" : ""}
                  />
                  {errors.equipo_precio && (
                    <p className="text-sm text-red-500">
                      {errors.equipo_precio}
                    </p>
                  )}
                </div>

                <div>
                  <Label htmlFor="equipo_stock">Stock Inicial</Label>
                  <Input
                    id="equipo_stock"
                    type="number"
                    min="0"
                    value={formData.equipo_stock}
                    onChange={(e) =>
                      setFormData({ ...formData, equipo_stock: e.target.value })
                    }
                    className={errors.equipo_stock ? "border-red-500" : ""}
                  />
                  {errors.equipo_stock && (
                    <p className="text-sm text-red-500">
                      {errors.equipo_stock}
                    </p>
                  )}
                </div>

                <div>
                  <Label htmlFor="equipo_ubicacion">Ubicación</Label>
                  <Input
                    id="equipo_ubicacion"
                    value={formData.equipo_ubicacion}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        equipo_ubicacion: e.target.value,
                      })
                    }
                  />
                </div>

                <div>
                  <Label htmlFor="proveedor">Proveedor</Label>
                  <Input
                    id="proveedor"
                    value={formData.proveedor}
                    onChange={(e) =>
                      setFormData({ ...formData, proveedor: e.target.value })
                    }
                  />
                </div>

                <div>
                  <Label htmlFor="numero_serie">Número de Serie</Label>
                  <Input
                    id="numero_serie"
                    value={formData.numero_serie}
                    onChange={(e) =>
                      setFormData({ ...formData, numero_serie: e.target.value })
                    }
                  />
                </div>

                <div>
                  <Label htmlFor="observaciones">Observaciones</Label>
                  <Input
                    id="observaciones"
                    value={formData.observaciones}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        observaciones: e.target.value,
                      })
                    }
                  />
                </div>
              </div>
            </div>

            <DialogFooter className="gap-2">
              <Button
                type="button"
                variant="outline"
                onClick={handleCloseEditDialog}
              >
                Cancelar
              </Button>
              <Button type="submit">
                {editingEquipo ? "Actualizar" : "Crear"}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}
