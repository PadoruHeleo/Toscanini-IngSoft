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

interface Informe {
  informe_id: number;
  informe_codigo?: string;
  diagnostico?: string;
  recomendaciones?: string;
  solucion_aplicada?: string;
  tecnico_responsable?: string;
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

interface PiezaInforme {
  pieza_id: number;
  informe_id: number;
  cantidad: number;
  pieza_nombre?: string;
  pieza_marca?: string;
  pieza_precio?: number;
}

interface InformeFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onInformeAdded: () => void;
  informe?: Informe;
  isEditing?: boolean;
  ordenTrabajoId?: number; // Para asociar el informe a una orden de trabajo
}

interface FormData {
  diagnostico: string;
  recomendaciones: string;
  solucion_aplicada: string;
  tecnico_responsable: string;
}

interface FormErrors {
  diagnostico?: string;
  recomendaciones?: string;
  solucion_aplicada?: string;
  tecnico_responsable?: string;
}

interface SelectedPieza extends PiezaInforme {
  pieza_nombre: string;
  pieza_marca?: string;
  pieza_precio: number;
}

export default function InformeFormDialog({
  open,
  onOpenChange,
  onInformeAdded,
  informe,
  isEditing = false,
  ordenTrabajoId,
}: InformeFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [loading, setLoading] = useState(false);
  const [loadingSendToClient, setLoadingSendToClient] = useState(false);
  const [piezas, setPiezas] = useState<Pieza[]>([]);
  const [loadingPiezas, setLoadingPiezas] = useState(false);
  const [selectedPiezas, setSelectedPiezas] = useState<SelectedPieza[]>([]);
  const [selectedPiezaId, setSelectedPiezaId] = useState<string>("");
  const [cantidad, setCantidad] = useState<string>("1");
  const [formData, setFormData] = useState<FormData>({
    diagnostico: "",
    recomendaciones: "",
    solucion_aplicada: "",
    tecnico_responsable: "",
  });

  const [errors, setErrors] = useState<FormErrors>({});
  // Cargar piezas al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadPiezas();
      if (isEditing && informe) {
        loadInformePiezas();
      } else if (!isEditing && ordenTrabajoId) {
        // Cargar piezas de la cotización asociada a la orden de trabajo
        loadPiezasFromOrdenTrabajo();
      }
    }
  }, [open]);

  // Inicializar formulario cuando se pasa un informe para editar
  useEffect(() => {
    if (isEditing && informe && open) {
      setFormData({
        diagnostico: informe.diagnostico || "",
        recomendaciones: informe.recomendaciones || "",
        solucion_aplicada: informe.solucion_aplicada || "",
        tecnico_responsable: informe.tecnico_responsable || "",
      });
    } else if (!isEditing && open) {
      // Resetear formulario para crear nuevo informe
      setFormData({
        diagnostico: "",
        recomendaciones: "",
        solucion_aplicada: "",
        tecnico_responsable: user?.usuario_nombre || "",
      });
      // No resetear selectedPiezas aquí si venimos de una orden de trabajo
      // porque se cargarán automáticamente desde loadPiezasFromOrdenTrabajo
      if (!ordenTrabajoId) {
        setSelectedPiezas([]);
      }
    }
    setErrors({});
  }, [isEditing, informe, open, user]);

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
  const loadInformePiezas = async () => {
    if (!informe?.informe_id) return;

    try {
      const piezasInforme = await invoke<PiezaInforme[]>("get_piezas_informe", {
        informeId: informe.informe_id,
      });

      const selectedPiezasWithDetails: SelectedPieza[] = piezasInforme.map(
        (pi) => ({
          pieza_id: pi.pieza_id,
          informe_id: pi.informe_id,
          cantidad: pi.cantidad,
          pieza_nombre: pi.pieza_nombre || "Nombre no disponible",
          pieza_marca: pi.pieza_marca,
          pieza_precio: pi.pieza_precio || 0,
        })
      );

      setSelectedPiezas(selectedPiezasWithDetails);
    } catch (error) {
      console.error("Error cargando piezas de informe:", error);
      let errorMsg = "No se pudieron cargar las piezas del informe.";
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

  const loadPiezasFromOrdenTrabajo = async () => {
    if (!ordenTrabajoId) return;

    try {
      // Primero obtener la orden de trabajo para conseguir la cotización asociada
      const ordenTrabajo = await invoke<any>("get_orden_trabajo_by_id", {
        ordenId: ordenTrabajoId,
      });

      if (!ordenTrabajo?.cotizacion_id) {
        console.log("La orden de trabajo no tiene una cotización asociada");
        return;
      }

      // Obtener las piezas de la cotización
      const piezasCotizacion = await invoke<any[]>("get_piezas_cotizacion", {
        cotizacionId: ordenTrabajo.cotizacion_id,
      });

      // Convertir las piezas de cotización a formato de piezas de informe
      const selectedPiezasWithDetails: SelectedPieza[] = piezasCotizacion.map(
        (pc) => ({
          pieza_id: pc.pieza_id,
          informe_id: 0, // Se asignará cuando se cree el informe
          cantidad: pc.cantidad || 1,
          pieza_nombre: pc.pieza_nombre || "Nombre no disponible",
          pieza_marca: pc.pieza_marca,
          pieza_precio: pc.pieza_precio || 0,
        })
      );

      setSelectedPiezas(selectedPiezasWithDetails);

      if (selectedPiezasWithDetails.length > 0) {
        console.log(
          `Se cargaron ${selectedPiezasWithDetails.length} piezas de la cotización asociada`
        );
      }
    } catch (error) {
      console.error("Error cargando piezas de la orden de trabajo:", error);
      // No mostrar error al usuario ya que esto es una funcionalidad de conveniencia
      // Si no se pueden cargar las piezas, simplemente no se precargan
    }
  };

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.diagnostico.trim()) {
      newErrors.diagnostico = "El diagnóstico es requerido";
    }

    if (!formData.tecnico_responsable.trim()) {
      newErrors.tecnico_responsable = "El técnico responsable es requerido";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleInputChange = (field: keyof FormData, value: string) => {
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
        informe_id: informe?.informe_id || 0,
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

    try {
      setLoading(true);

      if (isEditing && informe) {
        // Actualizar informe existente
        const updateData = {
          diagnostico:
            formData.diagnostico !== informe.diagnostico
              ? formData.diagnostico
              : undefined,
          recomendaciones:
            formData.recomendaciones !== informe.recomendaciones
              ? formData.recomendaciones
              : undefined,
          solucion_aplicada:
            formData.solucion_aplicada !== informe.solucion_aplicada
              ? formData.solucion_aplicada
              : undefined,
          tecnico_responsable:
            formData.tecnico_responsable !== informe.tecnico_responsable
              ? formData.tecnico_responsable
              : undefined,
        };

        const result = await invoke<boolean>("update_informe", {
          informeId: informe.informe_id,
          request: updateData,
        });

        if (result) {
          success(
            "Informe actualizado",
            `El informe ha sido actualizado exitosamente.`
          );
          onInformeAdded();
        } else {
          showError("Error", "No se pudo actualizar el informe.");
        }
      } else {
        // Crear nuevo informe
        const createData = {
          diagnostico: formData.diagnostico,
          recomendaciones: formData.recomendaciones.trim() || undefined,
          solucion_aplicada: formData.solucion_aplicada.trim() || undefined,
          tecnico_responsable: formData.tecnico_responsable,
          created_by: user.usuario_id,
          piezas:
            selectedPiezas.length > 0
              ? selectedPiezas.map((pieza) => ({
                  pieza_id: pieza.pieza_id,
                  cantidad: pieza.cantidad,
                }))
              : undefined,
        };

        const informeResult = await invoke<any>("create_informe", {
          request: createData,
        });
        const informeId = informeResult?.informe_id ?? informeResult;

        if (!informeId || isNaN(Number(informeId)) || informeId <= 0) {
          showError(
            "Error",
            `No se pudo crear el informe. ID inválido: ${informeId}`
          );
          setLoading(false);
          return;
        }

        let asociadoAOrden = false;
        // Si se proporciona ordenTrabajoId, asociar el informe a la orden
        if (ordenTrabajoId) {
          try {
            const asociado = await invoke<boolean>(
              "asignar_informe_orden_trabajo",
              {
                ordenId: ordenTrabajoId,
                informeId: informeId,
                updatedBy: user.usuario_id,
              }
            );
            asociadoAOrden = !!asociado;
            if (!asociadoAOrden) {
              showError(
                "Advertencia",
                "El informe se creó pero no se pudo asociar a la orden de trabajo."
              );
            }
          } catch (error) {
            console.error("Error asociando informe a orden de trabajo:", error);
            showError(
              "Advertencia",
              "El informe se creó pero no se pudo asociar a la orden de trabajo."
            );
          }
        }

        success(
          "Informe creado",
          `El informe ha sido creado exitosamente.` +
            (ordenTrabajoId
              ? asociadoAOrden
                ? " (Asociado a la orden de trabajo)"
                : " (No se pudo asociar a la orden de trabajo)"
              : "")
        );
        onInformeAdded();
      }

      onOpenChange(false);
    } catch (error) {
      showError(
        `Error al ${isEditing ? "actualizar" : "crear"} informe`,
        error instanceof Error ? error.message : JSON.stringify(error)
      );
    } finally {
      setLoading(false);
    }
  };

  const handleSubmitAndSendToClient = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }

    if (!validateForm()) {
      return;
    }

    try {
      setLoadingSendToClient(true);

      // Crear el informe primero (solo para creación nueva)
      if (!isEditing) {
        const createData = {
          diagnostico: formData.diagnostico,
          recomendaciones: formData.recomendaciones.trim() || undefined,
          solucion_aplicada: formData.solucion_aplicada.trim() || undefined,
          tecnico_responsable: formData.tecnico_responsable,
          created_by: user.usuario_id,
          piezas:
            selectedPiezas.length > 0
              ? selectedPiezas.map((pieza) => ({
                  pieza_id: pieza.pieza_id,
                  cantidad: pieza.cantidad,
                }))
              : undefined,
        };

        const informeResult = await invoke<any>("create_informe", {
          request: createData,
        });
        const informeId = informeResult?.informe_id ?? informeResult;

        if (!informeId || isNaN(Number(informeId)) || informeId <= 0) {
          showError(
            "Error",
            `No se pudo crear el informe. ID inválido: ${informeId}`
          );
          setLoadingSendToClient(false);
          return;
        }

        // Asociar el informe a la orden de trabajo si corresponde
        let asociadoAOrden = false;
        if (ordenTrabajoId) {
          try {
            const asociado = await invoke<boolean>(
              "asignar_informe_orden_trabajo",
              {
                ordenId: ordenTrabajoId,
                informeId: informeId,
                updatedBy: user.usuario_id,
              }
            );
            asociadoAOrden = !!asociado;
          } catch (error) {
            console.error("Error asociando informe a orden de trabajo:", error);
            showError(
              "Advertencia",
              "El informe se creó pero no se pudo asociar a la orden de trabajo."
            );
          }
        }

        // Enviar el informe al cliente
        try {
          await invoke<boolean>("send_informe_to_client", {
            informeId: informeId,
            sentBy: user.usuario_id,
          });

          success(
            "Informe creado y enviado",
            `El informe ha sido creado y enviado al cliente exitosamente.` +
              (ordenTrabajoId
                ? asociadoAOrden
                  ? " (Asociado a la orden de trabajo)"
                  : " (No se pudo asociar a la orden de trabajo)"
                : "")
          );
          onInformeAdded();
          onOpenChange(false);
        } catch (error) {
          console.error("Error enviando informe al cliente:", error);
          showError(
            "Informe creado pero no enviado",
            "El informe fue creado exitosamente, pero no se pudo enviar al cliente. Puedes intentar enviarlo manualmente más tarde."
          );
          onInformeAdded();
          onOpenChange(false);
        }
      } else {
        // Para edición, no implementamos envío directo
        showError(
          "Función no disponible",
          "El envío al cliente solo está disponible al crear nuevos informes."
        );
      }
    } catch (error) {
      showError(
        "Error al crear y enviar informe",
        error instanceof Error ? error.message : JSON.stringify(error)
      );
    } finally {
      setLoadingSendToClient(false);
    }
  };

  const getPiezaDisplayName = (pieza: Pieza) => {
    const parts = [];
    if (pieza.pieza_nombre) parts.push(pieza.pieza_nombre);
    if (pieza.pieza_marca) parts.push(`(${pieza.pieza_marca})`);
    if (pieza.pieza_precio) parts.push(`- $${pieza.pieza_precio}`);
    return parts.length > 0 ? parts.join(" ") : `Pieza ${pieza.pieza_id}`;
  };
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="!max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {isEditing ? "Editar Informe" : "Crear Nuevo Informe"}
          </DialogTitle>
          <DialogDescription>
            {isEditing
              ? "Modifica los datos del informe."
              : "Completa los campos para crear un nuevo informe técnico."}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Código del informe (solo mostrar en edición) */}
          {isEditing && informe?.informe_codigo && (
            <div className="space-y-2">
              <Label htmlFor="codigo">Código del Informe</Label>
              <Input
                id="codigo"
                value={informe.informe_codigo}
                disabled
                className="bg-gray-100"
              />
            </div>
          )}
          {/* Campos del formulario */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="diagnostico">
                Diagnóstico <span className="text-red-500">*</span>
              </Label>
              <Textarea
                id="diagnostico"
                value={formData.diagnostico}
                onChange={(e) =>
                  handleInputChange("diagnostico", e.target.value)
                }
                placeholder="Describa el diagnóstico del equipo..."
                rows={4}
                className={errors.diagnostico ? "border-red-500" : ""}
              />
              {errors.diagnostico && (
                <p className="text-sm text-red-500">{errors.diagnostico}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="tecnico_responsable">
                Técnico Responsable <span className="text-red-500">*</span>
              </Label>
              <Input
                id="tecnico_responsable"
                value={formData.tecnico_responsable}
                onChange={(e) =>
                  handleInputChange("tecnico_responsable", e.target.value)
                }
                placeholder="Nombre del técnico responsable"
                className={errors.tecnico_responsable ? "border-red-500" : ""}
              />
              {errors.tecnico_responsable && (
                <p className="text-sm text-red-500">
                  {errors.tecnico_responsable}
                </p>
              )}
            </div>
          </div>
          <div className="space-y-2">
            <Label htmlFor="recomendaciones">Recomendaciones</Label>
            <Textarea
              id="recomendaciones"
              value={formData.recomendaciones}
              onChange={(e) =>
                handleInputChange("recomendaciones", e.target.value)
              }
              placeholder="Recomendaciones para el cliente..."
              rows={3}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="solucion_aplicada">Solución Aplicada</Label>
            <Textarea
              id="solucion_aplicada"
              value={formData.solucion_aplicada}
              onChange={(e) =>
                handleInputChange("solucion_aplicada", e.target.value)
              }
              placeholder="Describa la solución aplicada..."
              rows={3}
            />
          </div>
          {/* Sección de piezas utilizadas */}
          <div className="space-y-4 border-t pt-4">
            <h3 className="text-lg font-semibold">Piezas Utilizadas</h3>

            {/* Agregar pieza */}
            <div className="flex gap-2 items-end">
              <div className="flex-1">
                <Label htmlFor="pieza">Seleccionar Pieza</Label>
                <Select
                  value={selectedPiezaId}
                  onValueChange={setSelectedPiezaId}
                  disabled={loadingPiezas}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Selecciona una pieza..." />
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
                disabled={!selectedPiezaId || !cantidad || loadingPiezas}
                className="mb-0"
              >
                <Plus className="w-4 h-4" />
              </Button>
            </div>

            {/* Lista de piezas seleccionadas */}
            {selectedPiezas.length > 0 && (
              <div className="border rounded-lg overflow-hidden">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Pieza</TableHead>
                      <TableHead>Marca</TableHead>
                      <TableHead>Cantidad</TableHead>
                      <TableHead>Precio Unitario</TableHead>
                      <TableHead>Subtotal</TableHead>
                      <TableHead>Acciones</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {selectedPiezas.map((pieza) => (
                      <TableRow key={pieza.pieza_id}>
                        <TableCell>{pieza.pieza_nombre}</TableCell>
                        <TableCell>{pieza.pieza_marca || "N/A"}</TableCell>
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
                            className="w-20"
                          />
                        </TableCell>
                        <TableCell>
                          ${pieza.pieza_precio.toLocaleString()}
                        </TableCell>
                        <TableCell>
                          $
                          {(
                            pieza.pieza_precio * pieza.cantidad
                          ).toLocaleString()}
                        </TableCell>
                        <TableCell>
                          <Button
                            type="button"
                            variant="outline"
                            size="sm"
                            onClick={() => handleRemovePieza(pieza.pieza_id)}
                          >
                            <Trash2 className="w-4 h-4" />
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            )}
          </div>{" "}
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={loading || loadingSendToClient}
            >
              Cancelar
            </Button>            <Button type="submit" disabled={loading || loadingSendToClient}>
              {loading
                ? "Procesando..."
                : isEditing
                ? "Actualizar Informe"
                : "Guardar Informe"}
            </Button>
            {!isEditing && (
              <Button
                type="button"
                onClick={handleSubmitAndSendToClient}
                disabled={loading || loadingSendToClient}
                className="bg-green-600 hover:bg-green-700"
              >
                {loadingSendToClient
                  ? "Enviando..."
                  : "Guardar y Enviar al Cliente"}
              </Button>
            )}
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
