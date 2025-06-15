import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";
import { Plus, Trash2 } from "lucide-react";
import { Textarea } from "@/components/ui/textarea";

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

interface Pieza {
  pieza_id: number;
  pieza_nombre?: string;
  pieza_marca?: string;
  pieza_desc?: string;
  pieza_precio?: number;
  created_at?: string;
}

interface PiezaCotizacion {
  pieza_id: number;
  cotizacion_id: number;
  cantidad: number;
  pieza_nombre?: string;
  pieza_marca?: string;
  pieza_precio?: number;
}

interface CotizacionFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCotizacionAdded: () => void;
  cotizacion?: Cotizacion;
  isEditing?: boolean;
  ordenTrabajoId?: number; // Para asociar la cotización a una orden de trabajo
  onSendToClient?: (cotizacionId: number) => void; // Nueva función para enviar al cliente
}

interface FormData {
  costo_revision: string;
  costo_reparacion: string;
  is_aprobada: boolean;
  informe: string;
}

interface FormErrors {
  costo_revision?: string;
  costo_reparacion?: string;
  informe?: string;
}

interface SelectedPieza extends PiezaCotizacion {
  pieza_nombre: string;
  pieza_marca?: string;
  pieza_precio: number;
}

export default function CotizacionFormDialog({
  open,
  onOpenChange,
  onCotizacionAdded,
  cotizacion,
  isEditing = false,
  ordenTrabajoId,
  onSendToClient,
}: CotizacionFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [loading, setLoading] = useState(false);
  const [piezas, setPiezas] = useState<Pieza[]>([]);
  const [loadingPiezas, setLoadingPiezas] = useState(false);
  const [selectedPiezas, setSelectedPiezas] = useState<SelectedPieza[]>([]);
  const [selectedPiezaId, setSelectedPiezaId] = useState<string>("");
  const [cantidad, setCantidad] = useState<string>("1");
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  const [formData, setFormData] = useState<FormData>({
    costo_revision: "25000",
    costo_reparacion: "0",
    is_aprobada: false,
    informe: "",
  });

  const [errors, setErrors] = useState<FormErrors>({});

  // Calcular costo total automáticamente
  const calculateTotal = () => {
    const costoRevision = parseInt(formData.costo_revision) || 0;
    const costoReparacion = parseInt(formData.costo_reparacion) || 0;
    const costoPiezas = selectedPiezas.reduce(
      (total, pieza) => total + pieza.pieza_precio * pieza.cantidad,
      0
    );
    return costoRevision + costoReparacion + costoPiezas;
  };

  // Cargar piezas al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadPiezas();
      if (isEditing && cotizacion) {
        loadCotizacionPiezas();
      }
    }
  }, [open]);

  // Inicializar formulario cuando se pasa una cotización para editar
  useEffect(() => {
    if (isEditing && cotizacion && open) {
      setFormData({
        costo_revision: cotizacion.costo_revision?.toString() || "0",
        costo_reparacion: cotizacion.costo_reparacion?.toString() || "0",
        is_aprobada: cotizacion.is_aprobada || false,
        informe: cotizacion.informe || "",
      });
    } else if (!isEditing && open) {
      // Resetear formulario para crear nueva cotización
      setFormData({
        costo_revision: "25000",
        costo_reparacion: "0",
        is_aprobada: false,
        informe: "",
      });
      setSelectedPiezas([]);
    }
    setErrors({});
  }, [isEditing, cotizacion, open]);

  const loadPiezas = async () => {
    try {
      setLoadingPiezas(true);
      const piezasData = await invoke<Pieza[]>("get_piezas");
      setPiezas(piezasData);
    } catch (error) {
      console.error("Error cargando piezas:", error);
      showError("Error", "No se pudieron cargar las piezas.");
    } finally {
      setLoadingPiezas(false);
    }
  };

  const loadCotizacionPiezas = async () => {
    if (!cotizacion?.cotizacion_id) return;

    try {
      const piezasCotizacion = await invoke<PiezaCotizacion[]>(
        "get_piezas_cotizacion",
        {
          cotizacionId: cotizacion.cotizacion_id,
        }
      );

      const selectedPiezasWithDetails: SelectedPieza[] = piezasCotizacion.map(
        (pc) => ({
          pieza_id: pc.pieza_id,
          cotizacion_id: pc.cotizacion_id,
          cantidad: pc.cantidad,
          pieza_nombre: pc.pieza_nombre || "Nombre no disponible",
          pieza_marca: pc.pieza_marca,
          pieza_precio: pc.pieza_precio || 0,
        })
      );

      setSelectedPiezas(selectedPiezasWithDetails);
    } catch (error) {
      console.error("Error cargando piezas de cotización:", error);
      let errorMsg = "No se pudieron cargar las piezas de la cotización.";
      if (error instanceof Error) {
        errorMsg += `\n${error.message}`;
      } else if (typeof error === "string") {
        errorMsg += `\n${error}`;
      } else if (error && typeof error === "object" && "message" in error) {
        errorMsg += `\n${(error as any).message}`;
      }
      showError("Error", errorMsg);
    }
  };

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    const costoRevision = parseInt(formData.costo_revision);
    if (isNaN(costoRevision) || costoRevision < 0) {
      newErrors.costo_revision =
        "El costo de revisión debe ser un número válido mayor o igual a 0";
    }

    const costoReparacion = parseInt(formData.costo_reparacion);
    if (isNaN(costoReparacion) || costoReparacion < 0) {
      newErrors.costo_reparacion =
        "El costo de reparación debe ser un número válido mayor o igual a 0";
    }

    if (!formData.informe.trim()) {
      newErrors.informe = "El informe es requerido";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleInputChange = (
    field: keyof FormData,
    value: string | boolean
  ) => {
    setFormData((prev) => ({ ...prev, [field]: value }));

    // Limpiar error del campo cuando el usuario empiece a escribir
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const handleAddPieza = () => {
    if (!selectedPiezaId || !cantidad) return;

    const pieza = piezas.find((p) => p.pieza_id.toString() === selectedPiezaId);
    if (!pieza) return;

    const cantidadNum = parseInt(cantidad);
    if (isNaN(cantidadNum) || cantidadNum <= 0) {
      showError("Error", "La cantidad debe ser un número mayor a 0");
      return;
    }

    // Verificar si la pieza ya está seleccionada
    const existingIndex = selectedPiezas.findIndex(
      (sp) => sp.pieza_id === pieza.pieza_id
    );

    if (existingIndex >= 0) {
      // Actualizar cantidad si ya existe
      const updated = [...selectedPiezas];
      updated[existingIndex].cantidad += cantidadNum;
      setSelectedPiezas(updated);
    } else {
      // Agregar nueva pieza
      const newSelectedPieza: SelectedPieza = {
        pieza_id: pieza.pieza_id,
        cotizacion_id: cotizacion?.cotizacion_id || 0,
        cantidad: cantidadNum,
        pieza_nombre: pieza.pieza_nombre || "Nombre no disponible",
        pieza_marca: pieza.pieza_marca,
        pieza_precio: pieza.pieza_precio || 0,
      };
      setSelectedPiezas([...selectedPiezas, newSelectedPieza]);
    }

    setSelectedPiezaId("");
    setCantidad("1");
  };

  const handleRemovePieza = (piezaId: number) => {
    setSelectedPiezas(selectedPiezas.filter((sp) => sp.pieza_id !== piezaId));
  };

  const handleUpdateCantidad = (piezaId: number, newCantidad: string) => {
    const cantidadNum = parseInt(newCantidad);
    if (isNaN(cantidadNum) || cantidadNum <= 0) return;

    setSelectedPiezas(
      selectedPiezas.map((sp) =>
        sp.pieza_id === piezaId ? { ...sp, cantidad: cantidadNum } : sp
      )
    );
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }

    if (!validateForm()) {
      return;
    }

    // Mostrar modal de confirmación en lugar de enviar directamente
    setShowConfirmationDialog(true);
  };

  const handleConfirmSubmit = async () => {
    if (!user) {
      showError("Error de autenticación", "Usuario no autenticado");
      setShowConfirmationDialog(false);
      return;
    }

    try {
      setLoading(true);

      const costoTotal = calculateTotal();

      if (isEditing && cotizacion) {
        // Actualizar cotización existente
        const updateData = {
          costo_revision:
            parseInt(formData.costo_revision) !== cotizacion.costo_revision
              ? parseInt(formData.costo_revision)
              : undefined,
          costo_reparacion:
            parseInt(formData.costo_reparacion) !== cotizacion.costo_reparacion
              ? parseInt(formData.costo_reparacion)
              : undefined,
          costo_total:
            costoTotal !== cotizacion.costo_total ? costoTotal : undefined,
          is_aprobada:
            formData.is_aprobada !== cotizacion.is_aprobada
              ? formData.is_aprobada
              : undefined,
          informe:
            formData.informe !== cotizacion.informe
              ? formData.informe
              : undefined,
        };

        const result = await invoke<boolean>("update_cotizacion", {
          cotizacionId: cotizacion.cotizacion_id,
          request: updateData,
        });

        if (result) {
          // Actualizar piezas
          await updateCotizacionPiezas(cotizacion.cotizacion_id);

          success(
            "Cotización actualizada",
            `La cotización ha sido actualizada exitosamente.`
          );
          onCotizacionAdded();
        } else {
          showError("Error", "No se pudo actualizar la cotización.");
        }
      } else {
        // Crear nueva cotización
        const createData = {
          costo_revision: parseInt(formData.costo_revision),
          costo_reparacion: parseInt(formData.costo_reparacion),
          costo_total: costoTotal,
          is_aprobada: formData.is_aprobada,
          is_borrador: true, // Siempre crear como borrador
          created_by: user.usuario_id,
          informe: formData.informe,
          piezas:
            selectedPiezas.length > 0
              ? selectedPiezas.map((pieza) => ({
                  pieza_id: pieza.pieza_id,
                  cantidad: pieza.cantidad,
                }))
              : undefined,
        };

        const cotizacionResult = await invoke<any>("create_cotizacion", {
          request: createData,
        });
        const cotizacionId =
          cotizacionResult?.cotizacion_id ?? cotizacionResult;

        if (!cotizacionId || isNaN(Number(cotizacionId)) || cotizacionId <= 0) {
          showError(
            "Error",
            `No se pudo crear la cotización. ID inválido: ${cotizacionId}`
          );
          setLoading(false);
          return;
        }

        // Agregar piezas a la cotización
        await updateCotizacionPiezas(cotizacionId);

        let asociadaAOrden = false;
        // Si se proporciona ordenTrabajoId, asociar la cotización a la orden
        if (ordenTrabajoId) {
          try {
            const asociada = await invoke<boolean>("update_orden_trabajo", {
              ordenId: ordenTrabajoId,
              request: { cotizacion_id: cotizacionId },
              updatedBy: user.usuario_id,
            });
            asociadaAOrden = !!asociada;
            if (!asociadaAOrden) {
              showError(
                "Advertencia",
                "La cotización se creó pero no se pudo asociar a la orden de trabajo."
              );
            }
          } catch (error) {
            console.error(
              "Error asociando cotización a orden de trabajo:",
              error
            );
            showError(
              "Advertencia",
              "La cotización se creó pero no se pudo asociar a la orden de trabajo."
            );
          }
        }

        success(
          "Cotización creada",
          `La cotización ha sido creada exitosamente.` +
            (ordenTrabajoId
              ? asociadaAOrden
                ? " (Asociada a la orden de trabajo)"
                : " (No se pudo asociar a la orden de trabajo)"
              : "")
        );
        onCotizacionAdded();
      }
    } catch (error) {
      showError(
        `Error al ${isEditing ? "actualizar" : "crear"} cotización`,
        error instanceof Error ? error.message : JSON.stringify(error)
      );
    } finally {
      setLoading(false);
      setShowConfirmationDialog(false);
    }
  };

  const updateCotizacionPiezas = async (cotizacionId: number) => {
    if (!cotizacionId || isNaN(Number(cotizacionId))) {
      throw new Error(
        "cotizacionId inválido al agregar piezas a la cotización"
      );
    }
    // Solo soportado para creación, no para edición
    if (!isEditing) {
      // Ya se envían las piezas en create_cotizacion
      return;
    } else {
      // Si se desea soportar edición de piezas, implementar en backend y aquí
      showError(
        "No soportado",
        "La edición de piezas en cotizaciones existentes no está soportada."
      );
      return;
    }
  };

  const getPiezaDisplayName = (pieza: Pieza) => {
    const parts = [];
    if (pieza.pieza_nombre) parts.push(pieza.pieza_nombre);
    if (pieza.pieza_marca) parts.push(`(${pieza.pieza_marca})`);
    if (pieza.pieza_precio) parts.push(`- $${pieza.pieza_precio}`);
    return parts.length > 0 ? parts.join(" ") : `Pieza ${pieza.pieza_id}`;
  };

  const handleSendToClient = async () => {
    if (!cotizacion?.cotizacion_id || !user) {
      showError("Error", "No se puede enviar la cotización al cliente.");
      return;
    }

    try {
      setLoading(true);

      // Actualizar is_borrador a false para marcar como enviada
      const result = await invoke<boolean>("update_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        request: { is_borrador: false },
      });

      if (result) {
        success(
          "Cotización enviada",
          "La cotización ha sido enviada al cliente exitosamente."
        );

        if (onSendToClient) {
          onSendToClient(cotizacion.cotizacion_id);
        }

        onCotizacionAdded(); // Refrescar la lista
        onOpenChange(false); // Cerrar el diálogo
      } else {
        showError("Error", "No se pudo enviar la cotización al cliente.");
      }
    } catch (error) {
      console.error("Error enviando cotización al cliente:", error);
      showError(
        "Error al enviar cotización",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        style={{ maxWidth: "38vw", width: "38vw", minWidth: "340px" }}
        className="max-h-[90vh] overflow-y-auto"
      >
        <DialogHeader>
          <DialogTitle>
            {isEditing ? "Editar Cotización" : "Crear Nueva Cotización"}
          </DialogTitle>
          <DialogDescription>
            {isEditing
              ? "Modifica los datos de la cotización"
              : "Completa los datos para crear una nueva cotización"}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Datos básicos */}
          <div className="grid grid-cols-2 gap-4">
            {/* Código de cotización (solo lectura si existe) */}
            <div className="space-y-2">
              <Label htmlFor="cotizacion_codigo">Código de Cotización</Label>
              <Input
                id="cotizacion_codigo"
                type="text"
                value={
                  cotizacion?.cotizacion_codigo ||
                  "(Se generará automáticamente)"
                }
                readOnly
                className="bg-gray-50 font-semibold"
              />
            </div>

            {/* Costo total (solo lectura) */}
            <div className="space-y-2">
              <Label htmlFor="costo_total">Costo Total</Label>
              <Input
                id="costo_total"
                type="text"
                value={`$${calculateTotal()}`}
                readOnly
                className="bg-gray-50 font-semibold"
              />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            {/* Costo de revisión */}
            <div className="space-y-2">
              <Label htmlFor="costo_revision">Costo de Revisión *</Label>
              <Input
                id="costo_revision"
                type="number"
                min="0"
                value={formData.costo_revision}
                onChange={(e) =>
                  handleInputChange("costo_revision", e.target.value)
                }
                placeholder="25000"
                className={errors.costo_revision ? "border-red-500" : ""}
              />
              {errors.costo_revision && (
                <p className="text-sm text-red-500">{errors.costo_revision}</p>
              )}
            </div>

            {/* Costo de reparación */}
            <div className="space-y-2">
              <Label htmlFor="costo_reparacion">Costo de Reparación *</Label>
              <Input
                id="costo_reparacion"
                type="number"
                min="0"
                value={formData.costo_reparacion}
                onChange={(e) =>
                  handleInputChange("costo_reparacion", e.target.value)
                }
                placeholder="0"
                className={errors.costo_reparacion ? "border-red-500" : ""}
              />
              {errors.costo_reparacion && (
                <p className="text-sm text-red-500">
                  {errors.costo_reparacion}
                </p>
              )}
            </div>
          </div>{" "}
          {/* Checkboxes */}
          <div className="flex gap-6">
            <div className="flex items-center space-x-2">
              <Checkbox
                id="is_aprobada"
                checked={formData.is_aprobada}
                onCheckedChange={(checked) =>
                  handleInputChange("is_aprobada", !!checked)
                }
              />
              <Label htmlFor="is_aprobada">Está aprobada</Label>
            </div>
          </div>
          {/* Informe */}
          <div className="col-span-2 space-y-2">
            <Label htmlFor="informe">Informe *</Label>
            <Textarea
              id="informe"
              value={formData.informe}
              onChange={(e) => handleInputChange("informe", e.target.value)}
              placeholder="Redacte aquí el informe técnico de la cotización"
              className={errors.informe ? "border-red-500" : ""}
              rows={5}
              required
            />
            {errors.informe && (
              <p className="text-sm text-red-500">{errors.informe}</p>
            )}
          </div>
          {/* Gestión de piezas */}
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <Label className="text-base font-semibold">Piezas</Label>
            </div>

            {/* Agregar pieza */}
            <div className="flex gap-2 items-end">
              <div className="flex-1">
                <Label htmlFor="pieza_select">Seleccionar Pieza</Label>
                <Select
                  value={selectedPiezaId}
                  onValueChange={setSelectedPiezaId}
                >
                  <SelectTrigger>
                    <SelectValue
                      placeholder={
                        loadingPiezas
                          ? "Cargando piezas..."
                          : "Seleccionar pieza"
                      }
                    />
                  </SelectTrigger>
                  <SelectContent>
                    {piezas.map((pieza) => (
                      <SelectItem
                        key={pieza.pieza_id}
                        value={pieza.pieza_id.toString()}
                      >
                        {getPiezaDisplayName(pieza)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="w-24">
                <Label htmlFor="cantidad">Cantidad</Label>
                <Input
                  id="cantidad"
                  type="number"
                  min="1"
                  value={cantidad}
                  onChange={(e) => setCantidad(e.target.value)}
                />
              </div>

              <Button
                type="button"
                onClick={handleAddPieza}
                disabled={!selectedPiezaId || !cantidad}
                className="mb-0"
              >
                <Plus className="h-4 w-4" />
              </Button>
            </div>

            {/* Tabla de piezas seleccionadas */}
            {selectedPiezas.length > 0 && (
              <div className="border rounded-md">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Pieza</TableHead>
                      <TableHead>Marca</TableHead>
                      <TableHead>Precio Unit.</TableHead>
                      <TableHead>Cantidad</TableHead>
                      <TableHead>Subtotal</TableHead>
                      <TableHead className="w-12"></TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {selectedPiezas.map((pieza) => (
                      <TableRow key={pieza.pieza_id}>
                        <TableCell>{pieza.pieza_nombre}</TableCell>
                        <TableCell>{pieza.pieza_marca || "N/A"}</TableCell>
                        <TableCell>${pieza.pieza_precio}</TableCell>
                        <TableCell>
                          <Input
                            type="number"
                            min="1"
                            value={pieza.cantidad}
                            onChange={(e) =>
                              handleUpdateCantidad(
                                pieza.pieza_id,
                                e.target.value
                              )
                            }
                            className="w-16"
                          />
                        </TableCell>
                        <TableCell>
                          ${pieza.pieza_precio * pieza.cantidad}
                        </TableCell>
                        <TableCell>
                          <Button
                            type="button"
                            variant="ghost"
                            size="sm"
                            onClick={() => handleRemovePieza(pieza.pieza_id)}
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            )}
          </div>
          {Object.keys(errors).length > 0 && (
            <div className="text-sm text-red-500 bg-red-50 p-3 rounded-md">
              Por favor, corrija los errores antes de continuar.
            </div>
          )}{" "}
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={loading}
            >
              Cancelar
            </Button>

            {isEditing && cotizacion?.is_borrador && (
              <Button
                type="button"
                variant="default"
                onClick={handleSendToClient}
                disabled={loading}
                className="bg-blue-600 hover:bg-blue-700"
              >
                {loading ? "Enviando..." : "Enviar al Cliente"}
              </Button>
            )}

            <Button type="submit" disabled={loading}>
              {loading ? "Guardando..." : isEditing ? "Actualizar" : "Crear"}
            </Button>
          </DialogFooter>{" "}
        </form>
      </DialogContent>

      {/* Modal de confirmación */}
      <Dialog
        open={showConfirmationDialog}
        onOpenChange={setShowConfirmationDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>
              {isEditing
                ? "Confirmar Actualización"
                : "Confirmar Creación de Cotización"}
            </DialogTitle>
            <DialogDescription>
              {isEditing
                ? "¿Está seguro que desea actualizar esta cotización con los cambios realizados?"
                : "¿Está seguro que desea crear esta cotización con la siguiente información?"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2 text-sm">
            <div>
              <strong>Costo Revisión:</strong> $
              {parseInt(formData.costo_revision || "0").toLocaleString()}
            </div>
            <div>
              <strong>Costo Reparación:</strong> $
              {parseInt(formData.costo_reparacion || "0").toLocaleString()}
            </div>
            <div>
              <strong>Piezas:</strong> {selectedPiezas.length} pieza(s)
              seleccionada(s)
            </div>
            <div>
              <strong>Costo Total:</strong> ${calculateTotal().toLocaleString()}
            </div>
            <div>
              <strong>Estado:</strong>{" "}
              {formData.is_aprobada ? "Aprobada" : "Pendiente"}
            </div>
            {formData.informe && (
              <div>
                <strong>Informe:</strong>{" "}
                {formData.informe.length > 50
                  ? formData.informe.substring(0, 50) + "..."
                  : formData.informe}
              </div>
            )}
          </div>

          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmationDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmSubmit} disabled={loading}>
              {loading
                ? "Procesando..."
                : isEditing
                ? "Confirmar Actualización"
                : "Confirmar Creación"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}
