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
  Plus,
  Minus,
  RefreshCw,
  Package,
  AlertTriangle,
  Edit,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { ViewTitle } from "@/components/ViewTitle";
import { usePiezasPermissions } from "@/hooks/use-permissions";
import { AccessDenied } from "@/components/AccessDenied";
import { useToastContext } from "@/contexts/ToastContext";

interface PiezaInventario {
  pieza_id: number;
  pieza_nombre?: string;
  pieza_marca?: string;
  pieza_desc?: string;
  pieza_precio?: number;
  pieza_stock?: number;
  created_at?: string;
}

interface StockUpdateData {
  pieza_id: number;
  cantidad: number;
  tipo: "entrada" | "salida";
}

interface PiezaFormData {
  pieza_nombre: string;
  pieza_marca: string;
  pieza_desc: string;
  pieza_precio: string;
}

export function InventarioPiezasView() {
  const { canViewPiezas } = usePiezasPermissions();
  const { success, error } = useToastContext();
  const [piezas, setPiezas] = useState<PiezaInventario[]>([]);
  const [loading, setLoading] = useState(true);
  const [showStockDialog, setShowStockDialog] = useState(false);
  const [showEditDialog, setShowEditDialog] = useState(false);
  const [editingPieza, setEditingPieza] = useState<PiezaInventario | null>(
    null
  );
  const [selectedPieza, setSelectedPieza] = useState<PiezaInventario | null>(
    null
  );
  const [stockUpdate, setStockUpdate] = useState<StockUpdateData>({
    pieza_id: 0,
    cantidad: 0,
    tipo: "entrada",
  });
  const [formData, setFormData] = useState<PiezaFormData>({
    pieza_nombre: "",
    pieza_marca: "",
    pieza_desc: "",
    pieza_precio: "",
  });
  const [errors, setErrors] = useState<Partial<PiezaFormData>>({});
  const [searchTerm, setSearchTerm] = useState("");

  const loadPiezas = async () => {
    setLoading(true);
    try {
      const data = await invoke<PiezaInventario[]>("get_piezas_inventario");
      setPiezas(data);
    } catch (e) {
      console.error("Error loading piezas inventario:", e);
      // Fallback: usar get_piezas si no existe get_piezas_inventario
      try {
        const fallbackData = await invoke<PiezaInventario[]>("get_piezas");
        // Asignar stock 0 si no existe la columna
        const dataWithStock = fallbackData.map((pieza) => ({
          ...pieza,
          pieza_stock: pieza.pieza_stock || 0,
        }));
        setPiezas(dataWithStock);
      } catch (fallbackError) {
        console.error("Error loading piezas fallback:", fallbackError);
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadPiezas();
  }, []);

  const handleOpenStockDialog = (
    pieza: PiezaInventario,
    tipo: "entrada" | "salida"
  ) => {
    setSelectedPieza(pieza);
    setStockUpdate({
      pieza_id: pieza.pieza_id,
      cantidad: 1,
      tipo: tipo,
    });
    setShowStockDialog(true);
  };

  const handleCloseStockDialog = () => {
    setShowStockDialog(false);
    setSelectedPieza(null);
    setStockUpdate({ pieza_id: 0, cantidad: 0, tipo: "entrada" });
  };

  const handleOpenEditDialog = (pieza: PiezaInventario) => {
    setEditingPieza(pieza);
    setFormData({
      pieza_nombre: pieza.pieza_nombre || "",
      pieza_marca: pieza.pieza_marca || "",
      pieza_desc: pieza.pieza_desc || "",
      pieza_precio: pieza.pieza_precio?.toString() || "",
    });
    setErrors({});
    setShowEditDialog(true);
  };

  const handleCloseEditDialog = () => {
    setShowEditDialog(false);
    setEditingPieza(null);
    setFormData({
      pieza_nombre: "",
      pieza_marca: "",
      pieza_desc: "",
      pieza_precio: "",
    });
    setErrors({});
  };

  const validateForm = () => {
    const newErrors: Partial<PiezaFormData> = {};
    if (!formData.pieza_nombre.trim())
      newErrors.pieza_nombre = "El nombre es requerido";
    if (formData.pieza_precio && isNaN(Number(formData.pieza_precio)))
      newErrors.pieza_precio = "Debe ser un número";
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleEditSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm() || !editingPieza) return;

    try {
      await invoke("update_pieza", {
        piezaId: editingPieza.pieza_id,
        request: {
          pieza_nombre: formData.pieza_nombre,
          pieza_marca: formData.pieza_marca,
          pieza_desc: formData.pieza_desc,
          pieza_precio: formData.pieza_precio
            ? Number(formData.pieza_precio)
            : null,
        },
      });

      // Toast de éxito
      success(
        "Pieza actualizada exitosamente",
        `Se actualizó la pieza "${formData.pieza_nombre}"`
      );

      loadPiezas();
      handleCloseEditDialog();
    } catch (e) {
      console.error("Error updating pieza:", e);

      // Toast de error
      error(
        "Error al actualizar pieza",
        "No se pudo actualizar la información de la pieza"
      );
    }
  };

  const handleStockUpdate = async () => {
    if (!selectedPieza) return;

    try {
      await invoke("update_pieza_stock", {
        piezaId: stockUpdate.pieza_id,
        cantidad: stockUpdate.cantidad,
        tipo: stockUpdate.tipo,
      });

      // Toast de éxito
      const accion = stockUpdate.tipo === "entrada" ? "agregado" : "reducido";
      const cantidad = stockUpdate.cantidad;
      const nombre = selectedPieza.pieza_nombre || "Pieza";

      success(
        `Stock ${accion} exitosamente`,
        `Se ${
          stockUpdate.tipo === "entrada" ? "agregaron" : "redujeron"
        } ${cantidad} unidad${cantidad > 1 ? "es" : ""} de ${nombre}`
      );

      loadPiezas();
      handleCloseStockDialog();
    } catch (e) {
      console.error("Error updating stock:", e);

      // Toast de error
      error(
        "Error al actualizar stock",
        "No se pudo actualizar el stock de la pieza"
      );

      // Si no existe el comando, simular actualización local
      setPiezas((prevPiezas) =>
        prevPiezas.map((pieza) => {
          if (pieza.pieza_id === stockUpdate.pieza_id) {
            const currentStock = pieza.pieza_stock || 0;
            const newStock =
              stockUpdate.tipo === "entrada"
                ? currentStock + stockUpdate.cantidad
                : Math.max(0, currentStock - stockUpdate.cantidad);
            return { ...pieza, pieza_stock: newStock };
          }
          return pieza;
        })
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
    if (stock <= 5) {
      return {
        label: "Stock Bajo",
        className:
          "bg-amber-100 text-amber-800 border border-amber-200 hover:bg-amber-200",
        icon: AlertTriangle,
        textColor: "text-amber-600",
      };
    }
    if (stock <= 20) {
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

  const filteredPiezas = piezas.filter(
    (pieza) =>
      pieza.pieza_nombre?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      pieza.pieza_marca?.toLowerCase().includes(searchTerm.toLowerCase()) ||
      pieza.pieza_desc?.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleRefresh = () => {
    loadPiezas();
  };

  // Verificar permisos antes de renderizar la vista
  if (!canViewPiezas) {
    return <AccessDenied />;
  }

  return (
    <div className="p-6">
      <ViewTitle onRefresh={handleRefresh} />

      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">
          Gestión de Inventario de Piezas
        </h3>
        <p className="text-muted-foreground mb-4">
          Visualiza y gestiona el stock de todas las piezas en el inventario
        </p>

        {/* Resumen del inventario */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <div className="bg-emerald-50 border border-emerald-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <Package className="h-4 w-4 text-emerald-600" />
              <span className="text-sm font-medium text-emerald-800">
                Stock Alto
              </span>
            </div>
            <div className="text-2xl font-bold text-emerald-600">
              {filteredPiezas.filter((p) => (p.pieza_stock || 0) > 20).length}
            </div>
          </div>

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
            <div className="flex items-center gap-2">
              <Package className="h-4 w-4 text-blue-600" />
              <span className="text-sm font-medium text-blue-800">
                Stock Medio
              </span>
            </div>
            <div className="text-2xl font-bold text-blue-600">
              {
                filteredPiezas.filter(
                  (p) => (p.pieza_stock || 0) > 5 && (p.pieza_stock || 0) <= 20
                ).length
              }
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
                filteredPiezas.filter(
                  (p) => (p.pieza_stock || 0) > 0 && (p.pieza_stock || 0) <= 5
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
              {filteredPiezas.filter((p) => (p.pieza_stock || 0) === 0).length}
            </div>
          </div>
        </div>

        <div className="flex gap-4 mb-4">
          <Input
            placeholder="Buscar por nombre, marca o descripción..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="max-w-md"
          />
        </div>
      </div>

      <div className="rounded-md border bg-white">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Nombre</TableHead>
              <TableHead>Marca</TableHead>
              <TableHead>Descripción</TableHead>
              <TableHead>Precio</TableHead>
              <TableHead>Stock</TableHead>
              <TableHead>Estado</TableHead>
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
            ) : filteredPiezas.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron piezas que coincidan con la búsqueda"
                    : "No hay piezas en el inventario"}
                </TableCell>
              </TableRow>
            ) : (
              filteredPiezas.map((pieza) => {
                const stockStatus = getStockStatus(pieza.pieza_stock || 0);
                const StockIcon = stockStatus.icon;

                return (
                  <TableRow key={pieza.pieza_id}>
                    <TableCell className="font-medium">
                      {pieza.pieza_nombre}
                    </TableCell>
                    <TableCell>{pieza.pieza_marca}</TableCell>
                    <TableCell>{pieza.pieza_desc}</TableCell>
                    <TableCell>
                      {pieza.pieza_precio != null
                        ? `$${pieza.pieza_precio.toLocaleString()}`
                        : "-"}
                    </TableCell>
                    <TableCell className="text-center">
                      <div className="flex flex-col items-center gap-1">
                        <span
                          className={`font-mono text-lg font-semibold ${stockStatus.textColor}`}
                        >
                          {pieza.pieza_stock || 0}
                        </span>
                        <span className="text-xs text-gray-500">unidades</span>
                      </div>
                    </TableCell>
                    <TableCell>
                      <span
                        className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors gap-1 ${stockStatus.className}`}
                      >
                        <StockIcon className="h-3 w-3" />
                        {stockStatus.label}
                      </span>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1 justify-end">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleOpenEditDialog(pieza)}
                          className="text-blue-600 hover:text-blue-700"
                          title="Editar información de la pieza"
                        >
                          <Edit className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() =>
                            handleOpenStockDialog(pieza, "entrada")
                          }
                          className="text-green-600 hover:text-green-700"
                          title="Agregar stock"
                        >
                          <Plus className="h-3 w-3" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleOpenStockDialog(pieza, "salida")}
                          className="text-red-600 hover:text-red-700"
                          title="Reducir stock"
                          disabled={(pieza.pieza_stock || 0) === 0}
                        >
                          <Minus className="h-3 w-3" />
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
          {selectedPieza && (
            <div className="space-y-4">
              <div className="p-3 bg-gray-50 rounded-md">
                <p className="font-medium">{selectedPieza.pieza_nombre}</p>
                <p className="text-sm text-gray-600">
                  {selectedPieza.pieza_marca}
                </p>
                <p className="text-sm text-gray-500">
                  Stock actual: {selectedPieza.pieza_stock || 0}
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
                      ? selectedPieza.pieza_stock || 0
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
                    ? (selectedPieza.pieza_stock || 0) + stockUpdate.cantidad
                    : Math.max(
                        0,
                        (selectedPieza.pieza_stock || 0) - stockUpdate.cantidad
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

      {/* Dialog para editar información de la pieza */}
      <Dialog open={showEditDialog} onOpenChange={setShowEditDialog}>
        <DialogContent style={{ maxWidth: 400 }}>
          <DialogHeader>
            <DialogTitle>Editar Información de la Pieza</DialogTitle>
          </DialogHeader>
          {editingPieza && (
            <form onSubmit={handleEditSubmit} className="space-y-4">
              <div>
                <Label htmlFor="edit_pieza_nombre">Nombre *</Label>
                <Input
                  id="edit_pieza_nombre"
                  value={formData.pieza_nombre}
                  onChange={(e) =>
                    setFormData({ ...formData, pieza_nombre: e.target.value })
                  }
                  className={errors.pieza_nombre ? "border-red-500" : ""}
                />
                {errors.pieza_nombre && (
                  <p className="text-sm text-red-500">{errors.pieza_nombre}</p>
                )}
              </div>

              <div>
                <Label htmlFor="edit_pieza_marca">Marca</Label>
                <Input
                  id="edit_pieza_marca"
                  value={formData.pieza_marca}
                  onChange={(e) =>
                    setFormData({ ...formData, pieza_marca: e.target.value })
                  }
                />
              </div>

              <div>
                <Label htmlFor="edit_pieza_desc">Descripción</Label>
                <Input
                  id="edit_pieza_desc"
                  value={formData.pieza_desc}
                  onChange={(e) =>
                    setFormData({ ...formData, pieza_desc: e.target.value })
                  }
                />
              </div>

              <div>
                <Label htmlFor="edit_pieza_precio">Precio</Label>
                <Input
                  id="edit_pieza_precio"
                  type="number"
                  min="0"
                  value={formData.pieza_precio}
                  onChange={(e) =>
                    setFormData({ ...formData, pieza_precio: e.target.value })
                  }
                  className={errors.pieza_precio ? "border-red-500" : ""}
                />
                {errors.pieza_precio && (
                  <p className="text-sm text-red-500">{errors.pieza_precio}</p>
                )}
              </div>

              <DialogFooter className="gap-2">
                <Button
                  type="button"
                  variant="outline"
                  onClick={handleCloseEditDialog}
                >
                  Cancelar
                </Button>
                <Button type="submit">Actualizar</Button>
              </DialogFooter>
            </form>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
