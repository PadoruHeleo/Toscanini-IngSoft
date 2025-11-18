import React, { useState, useEffect } from "react";
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";
import { useOrdenTrabajoPermissions } from "@/hooks/use-permissions";
import { Plus, Trash2, Info } from "lucide-react";
import { Textarea } from "@/components/ui/textarea";
import { Alert, AlertDescription } from "@/components/ui/alert";

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
  const { getCotizacionActions } = useOrdenTrabajoPermissions();
  const [loading, setLoading] = useState(false);
  const [piezas, setPiezas] = useState<Pieza[]>([]);
  const [loadingPiezas, setLoadingPiezas] = useState(false);
  const [selectedPiezas, setSelectedPiezas] = useState<SelectedPieza[]>([]);
  const [selectedPiezaId, setSelectedPiezaId] = useState<string>("");
  const [cantidad, setCantidad] = useState<string>("1");
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  const [estadoOrden, setEstadoOrden] = useState<string>("");
  const [showAprobarConfirmDialog, setShowAprobarConfirmDialog] =
    useState(false);
  const [showRechazarConfirmDialog, setShowRechazarConfirmDialog] =
    useState(false);
  const [showNoReparableConfirmDialog, setShowNoReparableConfirmDialog] =
    useState(false);
  const [comentarioNoReparable, setComentarioNoReparable] = useState("");
  const [showAbandonoConfirmDialog, setShowAbandonoConfirmDialog] =
    useState(false);
  const [abandonoComentario, setAbandonoComentario] = useState("");
  const [motivoRechazo, setMotivoRechazo] = useState("");
  const [puedeAbandonar, setPuedeAbandonar] = useState(false);
  const [activeTab, setActiveTab] = useState("informacion");
  const [terminosCondiciones, setTerminosCondiciones] = useState<any[]>([]);
  const [loadingTerminos, setLoadingTerminos] = useState(false);
  const [selectedTerminos, setSelectedTerminos] = useState<number[]>([]);
  const [aplicadosTerminos, setAplicadosTerminos] = useState<number[]>([]); // Términos realmente guardados en BD
  const [formData, setFormData] = useState<FormData>({
    costo_revision: "25000",
    costo_reparacion: "0",
    is_aprobada: false,
    informe: "",
  });

  // Función para determinar si se pueden modificar términos según el estado
  const canModifyTerminos = () => {
    if (!ordenTrabajoId || !estadoOrden) return true; // Si no hay orden, permitir modificación

    // Estados donde SÍ se pueden modificar términos de cotización
    const estadosPermitidos = ["recibido", "cotizacion_rechazada"];

    return estadosPermitidos.includes(estadoOrden);
  };

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
      loadTerminosCondiciones();
      if (isEditing && cotizacion) {
        loadCotizacionPiezas();
        loadTerminosCotizacion();
      }
      // Si hay ordenTrabajoId, obtener el estado de la orden
      console.log("ordenTrabajoId:", ordenTrabajoId);
      if (ordenTrabajoId) {
        invoke<{ estado: string; created_at: string }>(
          "get_orden_trabajo_by_id",
          { ordenId: ordenTrabajoId }
        )
          .then((orden) => {
            setEstadoOrden(orden.estado);
            // Calcular si han pasado más de 168 horas
            if (orden.created_at) {
              const createdDate = new Date(orden.created_at);
              const now = new Date();
              const diffHours =
                (now.getTime() - createdDate.getTime()) / (1000 * 60 * 60);
              setPuedeAbandonar(diffHours >= 168);
            }
          })
          .catch((err) => {
            console.error("Error obteniendo estado de orden:", err);
          });
      }
    }
  }, [open]);

  // Aplicar términos por defecto cuando se tienen los datos necesarios
  useEffect(() => {
    // Solo aplicar términos por defecto si no estamos editando o si editando pero no hay términos cargados
    if (
      terminosCondiciones.length > 0 &&
      selectedTerminos.length === 0 &&
      open &&
      !isEditing // Solo para nuevas cotizaciones
    ) {
      const terminosDefecto = terminosCondiciones
        .filter((termino) => termino.is_default)
        .map((termino) => termino.termino_id);

      console.log("✅ Términos por defecto encontrados:", terminosDefecto);

      if (terminosDefecto.length > 0) {
        console.log(
          "🎯 Aplicando términos por defecto automáticamente:",
          terminosDefecto
        );
        setSelectedTerminos(terminosDefecto);
      }
    }
  }, [terminosCondiciones, open, isEditing]); // Removido selectedTerminos de las dependencias

  // Inicializar formulario cuando se pasa una cotización para editar
  useEffect(() => {
    if (isEditing && cotizacion && open) {
      setFormData({
        costo_revision: cotizacion.costo_revision?.toString() || "0",
        costo_reparacion: cotizacion.costo_reparacion?.toString() || "0",
        is_aprobada: cotizacion.is_aprobada || false,
        informe: cotizacion.informe || "",
      });
      // NO resetear selectedPiezas aquí - se cargarán en el otro useEffect solo si están vacías
    } else if (!isEditing && open) {
      // Resetear formulario para crear nueva cotización
      setFormData({
        costo_revision: "25000",
        costo_reparacion: "0",
        is_aprobada: false,
        informe: "",
      });
      // Solo resetear selectedPiezas cuando se abre el diálogo para crear nueva cotización
      setSelectedPiezas([]);
      // Resetear términos seleccionados para que loadTerminosCondiciones pueda aplicar los por defecto
      setSelectedTerminos([]);
    }
    setErrors({});
  }, [isEditing, cotizacion?.cotizacion_id, open]); // Usar cotizacion_id en lugar del objeto completo

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
          cantidad: pc.cantidad ?? 1, // Manejar None correctamente
          pieza_nombre: pc.pieza_nombre || "Nombre no disponible",
          pieza_marca: pc.pieza_marca,
          pieza_precio: pc.pieza_precio ?? 0,
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

  const loadTerminosCondiciones = async () => {
    try {
      setLoadingTerminos(true);
      // Cargar solo términos aplicables a cotizaciones
      const terminos = await invoke<any[]>("get_terminos_condiciones_by_tipo", {
        tipo: "cotizacion",
      });
      console.log(
        "📋 Términos cargados para cotizaciones:",
        terminos.map((t) => ({
          id: t.termino_id,
          nombre: t.termino_nombre,
          tipo: t.tipo_referencia,
          isDefault: t.is_default,
        }))
      );
      setTerminosCondiciones(terminos);

      // Aplicar términos por defecto inmediatamente al cargar (solo si no hay términos seleccionados)
      if (selectedTerminos.length === 0) {
        const terminosDefecto = terminos
          .filter((termino) => termino.is_default)
          .map((termino) => termino.termino_id);

        if (terminosDefecto.length > 0) {
          console.log(
            "🎯 Aplicando términos por defecto al cargar:",
            terminosDefecto
          );
          setSelectedTerminos(terminosDefecto);
        }
      }
    } catch (error) {
      console.error("Error cargando términos y condiciones:", error);
      showError("Error", "No se pudieron cargar los términos y condiciones.");
    } finally {
      setLoadingTerminos(false);
    }
  };

  const loadTerminosCotizacion = async () => {
    if (!cotizacion?.cotizacion_id) return;
    try {
      console.log(
        "🔍 Cargando términos de la cotización:",
        cotizacion.cotizacion_id
      );

      const terminosCotizacion = await invoke<any[]>(
        "get_terminos_by_cotizacion",
        {
          cotizacionId: cotizacion.cotizacion_id,
        }
      );

      console.log("📋 Términos recibidos del backend:", terminosCotizacion);

      // Extraer solo los IDs de los términos devueltos por el backend
      const terminoIds = terminosCotizacion.map((t) => t.termino_id);

      console.log("🎯 IDs de términos aplicados:", terminoIds);

      setSelectedTerminos(terminoIds);
      setAplicadosTerminos(terminoIds); // Los que están en BD son los realmente aplicados

      console.log("✅ Estado actualizado correctamente");
    } catch (error) {
      console.error("❌ Error cargando términos de la cotización:", error);
      showError("Error", "No se pudieron cargar los términos de la cotización");
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

  const handleActualizarTerminos = async () => {
    console.log("🔍 Iniciando actualización de términos:", {
      cotizacionId: cotizacion?.cotizacion_id,
      selectedTerminos,
      selectedTerminosLength: selectedTerminos.length,
    });

    // Verificar si se pueden modificar términos según el estado
    if (!canModifyTerminos()) {
      showError(
        "Modificación no permitida",
        `No se pueden modificar los términos y condiciones cuando la orden está en estado "${estadoOrden}". Solo se permite modificar en estados iniciales (recibido, cotización rechazada).`
      );
      return;
    }

    // Mostrar detalles de los términos seleccionados
    console.log(
      "📋 Detalles de términos seleccionados:",
      selectedTerminos.map((id) => {
        const termino = terminosCondiciones.find((t) => t.termino_id === id);
        return {
          id,
          nombre: termino?.termino_nombre,
          tipo: termino?.tipo_referencia,
          isDefault: termino?.is_default,
        };
      })
    );

    if (!cotizacion?.cotizacion_id) {
      showError(
        "Error",
        "No se puede actualizar términos sin una cotización guardada"
      );
      return;
    }

    if (selectedTerminos.length === 0) {
      showError("Error", "No hay términos seleccionados para aplicar");
      return;
    }

    try {
      setLoadingTerminos(true);

      const terminoRequests = selectedTerminos.map((id) => ({
        termino_id: id,
        aplicado: true,
      }));

      console.log("📡 Llamando comando apply_terminos_to_cotizacion con:", {
        cotizacionId: cotizacion.cotizacion_id,
        terminos: terminoRequests,
        appliedBy: user?.usuario_id || 1,
      });

      // Aplicar términos seleccionados a la cotización
      await invoke("apply_terminos_to_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        terminos: terminoRequests,
        appliedBy: user?.usuario_id || 1,
      });

      console.log("✅ Comando ejecutado exitosamente");

      // Primero actualizar el estado local inmediatamente
      setAplicadosTerminos([...selectedTerminos]);

      success(
        "Términos actualizados",
        `Se han aplicado ${selectedTerminos.length} términos y condiciones a la cotización`
      );

      // Recargar términos para mostrar el estado actualizado desde la base de datos
      console.log("🔄 Recargando términos desde la base de datos...");

      // Pequeño delay para asegurar que la transacción se complete
      await new Promise((resolve) => setTimeout(resolve, 100));

      await loadTerminosCotizacion();
      console.log("✅ Términos recargados exitosamente");

      // Verificación final deshabilitada - ya no es necesaria
      // setTimeout(async () => {
      //   await loadTerminosCotizacion();
      //   console.log("🔍 Verificación final completada");
      // }, 500);
    } catch (error) {
      console.error("❌ Error detallado actualizando términos:", {
        error,
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
      });
      showError(
        "Error",
        "No se pudieron actualizar los términos y condiciones"
      );
    } finally {
      setLoadingTerminos(false);
    }
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
          updatedBy: user.usuario_id,
        });

        if (result) {
          // Actualizar piezas
          await updateCotizacionPiezas(cotizacion.cotizacion_id);

          // Aplicar términos y condiciones seleccionados
          if (selectedTerminos.length > 0) {
            try {
              // Convertir los IDs a la estructura esperada por el backend
              const terminoRequests = selectedTerminos.map((id) => ({
                termino_id: id,
                aplicado: true,
              }));

              console.log("📡 Aplicando términos a cotización:", {
                cotizacionId: cotizacion.cotizacion_id,
                terminos: terminoRequests, // Cambiar de terminoIds a terminos
                appliedBy: user.usuario_id, // Agregar appliedBy
              });

              await invoke("apply_terminos_to_cotizacion", {
                cotizacionId: cotizacion.cotizacion_id,
                terminos: terminoRequests, // Cambiar de terminoIds a terminos
                appliedBy: user.usuario_id, // Agregar appliedBy
              });
            } catch (error) {
              console.error("Error aplicando términos y condiciones:", error);
              console.error("Detalles del error:", {
                error,
                message: error instanceof Error ? error.message : String(error),
              });
              showError(
                "Advertencia",
                "La cotización se actualizó pero no se pudieron aplicar todos los términos y condiciones."
              );
            }
          }

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
        console.log("🔍 Estado antes de crear cotización:");
        console.log("  - selectedPiezas:", selectedPiezas);
        console.log("  - selectedPiezas.length:", selectedPiezas.length);

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
              ? selectedPiezas.map((pieza) => {
                  const piezaData = {
                    pieza_id: pieza.pieza_id,
                    cantidad: pieza.cantidad ?? 1, // Asegurar que siempre haya una cantidad
                  };
                  console.log("  - Mapeando pieza:", piezaData);
                  return piezaData;
                })
              : undefined,
        };

        console.log("📤 Enviando datos de cotización:", createData);
        console.log("📤 Piezas a enviar:", createData.piezas);
        console.log("📤 Tipo de piezas:", typeof createData.piezas);

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

        // Aplicar términos y condiciones seleccionados
        if (selectedTerminos.length > 0) {
          try {
            const terminoRequests = selectedTerminos.map((id) => ({
              termino_id: id,
              aplicado: true,
            }));

            await invoke("apply_terminos_to_cotizacion", {
              cotizacionId: cotizacionId,
              terminos: terminoRequests,
              appliedBy: user.usuario_id,
            });
          } catch (error) {
            console.error("Error aplicando términos y condiciones:", error);
            showError(
              "Advertencia",
              "La cotización se creó pero no se pudieron aplicar todos los términos y condiciones."
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

    console.log(
      "🔄 updateCotizacionPiezas: Actualizando piezas para cotización",
      cotizacionId
    );
    console.log("  - isEditing:", isEditing);
    console.log("  - selectedPiezas:", selectedPiezas);
    console.log("  - selectedPiezas.length:", selectedPiezas.length);

    if (!isEditing) {
      // Para creación nueva, las piezas ya se envían en create_cotizacion
      console.log(
        "ℹ️ Creación nueva - las piezas ya se enviaron en create_cotizacion"
      );
      return;
    } else {
      // Actualizar piezas de una cotización existente
      if (!user) {
        throw new Error("Usuario no autenticado");
      }

      const piezasData = selectedPiezas.map((pieza) => ({
        pieza_id: pieza.pieza_id,
        cantidad: pieza.cantidad ?? 1,
      }));

      console.log("📤 Enviando piezas para actualizar:", piezasData);

      const result = await invoke<boolean>("update_cotizacion_piezas", {
        cotizacionId: cotizacionId,
        piezas: piezasData,
        updatedBy: user.usuario_id,
      });

      if (result) {
        console.log("✅ Piezas actualizadas correctamente");
      } else {
        throw new Error("No se pudieron actualizar las piezas");
      }
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

      // Enviar el email con PDF y actualizar estados automáticamente
      await invoke<string>("send_cotizacion_email", {
        cotizacionId: cotizacion.cotizacion_id,
        sentBy: user.usuario_id,
      });

      success(
        "Cotización enviada",
        "La cotización ha sido enviada al cliente exitosamente con el PDF adjunto."
      );

      if (onSendToClient) {
        onSendToClient(cotizacion.cotizacion_id);
      }

      onCotizacionAdded(); // Refrescar la lista
      onOpenChange(false); // Cerrar el diálogo
    } catch (error) {
      console.error("Error enviando cotización al cliente:", error);
      showError(
        "Error al enviar cotización",
        typeof error === "string"
          ? error
          : "Ha ocurrido un error inesperado al enviar el correo."
      );
    } finally {
      setLoading(false);
    }
  };

  // Función para aprobar la cotización
  const handleAprobarCotizacion = async () => {
    if (!cotizacion?.cotizacion_id || !user || !ordenTrabajoId) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }
    try {
      setLoading(true);
      // Aprobar la cotización
      const result = await invoke<boolean>("update_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        request: { is_aprobada: true },
        updatedBy: user.usuario_id,
      });
      if (result) {
        // Cambiar estado de la orden a "en_reparacion"
        await invoke("cambiar_estado_orden_trabajo", {
          ordenId: ordenTrabajoId,
          nuevoEstado: "en_reparacion",
          updatedBy: user.usuario_id,
        });
        success(
          "Cotización aprobada",
          "La cotización ha sido aprobada y la orden está en reparación."
        );
        onCotizacionAdded();
        onOpenChange(false);
      } else {
        showError("Error", "No se pudo aprobar la cotización.");
      }
    } catch (error) {
      console.error(error);
      showError("Error", "Ocurrió un error al aprobar la cotización.");
    } finally {
      setLoading(false);
    }
  };

  const handleRechazarCotizacion = async (motivo?: string) => {
    if (!cotizacion?.cotizacion_id || !user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }
    try {
      setLoading(true);
      // Rechazar la cotización y guardar el motivo
      const result = await invoke<boolean>("update_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        request: { is_aprobada: false, motivo_rechazo: motivo },
        updatedBy: user.usuario_id,
      });
      if (result) {
        // Cambiar estado de la orden a "cotizacion_rechazada"
        await invoke("cambiar_estado_orden_trabajo", {
          ordenId: ordenTrabajoId,
          nuevoEstado: "cotizacion_rechazada",
          updatedBy: user.usuario_id,
        });
        success(
          "Cotización rechazada",
          "La cotización ha sido rechazada y la orden está cotizacion rechazada."
        );
        onCotizacionAdded();
        onOpenChange(false);
      } else {
        showError("Error", "No se pudo rechazar la cotización.");
      }
    } catch (error) {
      console.error(error);
      showError("Error", "Ocurrió un error al rechazar la cotización.");
    } finally {
      setLoading(false);
    }
  };

  const handleRechazarCotizacionBorrador = async (motivo?: string) => {
    if (!cotizacion?.cotizacion_id || !user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }
    try {
      setLoading(true);
      // Rechaza la cotización, guarda el motivo y marca como no borrador
      const result = await invoke<boolean>("update_cotizacion", {
        cotizacionId: cotizacion.cotizacion_id,
        request: {
          is_aprobada: false,
          is_borrador: false,
          motivo_rechazo: motivo,
          estado: "rechazada",
        },
        updatedBy: user.usuario_id,
      });
      if (result) {
        success("Cotización rechazada", "La cotización ha sido rechazada.");
        onCotizacionAdded();
        onOpenChange(false);
      } else {
        showError("Error", "No se pudo rechazar la cotización.");
      }
    } catch (error) {
      console.error(error);
      showError("Error", "Ocurrió un error al rechazar la cotización.");
    } finally {
      setLoading(false);
    }
  };

  const handleNoReparable = async () => {
    if (!cotizacion?.cotizacion_id || !user || !ordenTrabajoId) {
      showError("Error", "No se puede declarar como no reparable.");
      return;
    }

    if (!comentarioNoReparable.trim()) {
      showError(
        "Error",
        "Debe ingresar un comentario para justificar el estado."
      );
      return;
    }

    try {
      setLoading(true);

      // Cambiar estado de la orden a "no_reparable"
      await invoke("cambiar_estado_orden_trabajo", {
        ordenId: ordenTrabajoId,
        nuevoEstado: "equipo_no_reparable",
        updatedBy: user.usuario_id,
        comentario: comentarioNoReparable, // Enviar comentario
      });

      success(
        "Equipo declarado No Reparable",
        "La orden ha sido actualizada correctamente."
      );

      onCotizacionAdded();
      onOpenChange(false);
    } catch (error) {
      console.error(error);
      showError("Error", "Ocurrió un error al declarar como no reparable.");
    } finally {
      setLoading(false);
    }
  };

  const handleAbandonarEquipo = async () => {
    if (!cotizacion?.cotizacion_id || !user || !ordenTrabajoId) {
      showError("Error", "No se puede declarar el equipo como abandono.");
      return;
    }
    if (!abandonoComentario.trim()) {
      showError("Error", "Debe ingresar un comentario de justificación.");
      return;
    }

    try {
      setLoading(true);
      // Cambiar estado de la orden a "abandonado"
      await invoke("cambiar_estado_orden_trabajo", {
        ordenId: ordenTrabajoId,
        nuevoEstado: "abandonado",
        comentario: abandonoComentario,
        updatedBy: user.usuario_id,
      });

      success(
        "Equipo declarado como abandono",
        "El equipo fue marcado como abandonado exitosamente."
      );
      onCotizacionAdded();
      onOpenChange(false);
    } catch (error) {
      console.error(error);
      showError(
        "Error",
        "Ocurrió un error al declarar el equipo como abandono."
      );
    } finally {
      setLoading(false);
    }
  };

  // Determinar permisos según el estado de la cotización y rol del usuario
  const cotizacionActions = getCotizacionActions({
    is_aprobada: cotizacion?.is_aprobada,
    is_borrador: cotizacion?.is_borrador,
  });

  // Determinar si los campos deben estar en solo lectura para recepción
  const shouldFieldsBeReadOnly = () => {
    if (!cotizacion || user?.usuario_rol !== "recepcion") return false;
    return cotizacionActions.canApprove && !cotizacionActions.canEdit;
  };

  // Si es solo lectura (recepción viendo borrador), mostrar diálogo simplificado
  if (cotizacionActions.isReadOnly && cotizacion) {
    return (
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Cotización - Solo Lectura</DialogTitle>
            <DialogDescription>
              Esta cotización está en borrador. Como usuario de recepción, solo
              puede ver cotizaciones enviadas.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <strong>Código:</strong> {cotizacion.cotizacion_codigo || "N/A"}
            </div>
            <div>
              <strong>Costo Revisión:</strong> ${cotizacion.costo_revision || 0}
            </div>
            <div>
              <strong>Costo Reparación:</strong> $
              {cotizacion.costo_reparacion || 0}
            </div>
            <div>
              <strong>Costo Total:</strong> ${cotizacion.costo_total || 0}
            </div>
            <div>
              <strong>Estado:</strong> Borrador
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)} variant="outline">
              Cerrar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  }

  // Si recepción no puede crear cotizaciones nuevas
  if (!isEditing && !cotizacionActions.canCreate) {
    return (
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Acceso Denegado</DialogTitle>
            <DialogDescription>
              No tiene permisos para crear cotizaciones. Solo el personal
              técnico y administrativo puede crear cotizaciones.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button onClick={() => onOpenChange(false)} variant="outline">
              Cerrar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        style={{ maxWidth: "85vw", width: "85vw", minWidth: "600px" }}
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

        {/* Mensaje informativo para recepción con cotización en solo lectura */}
        {shouldFieldsBeReadOnly() && (
          <Alert className="mb-4">
            <Info className="h-4 w-4" />
            <AlertDescription>
              Como recepcionista, puedes aprobar o rechazar esta cotización
              enviada, pero no modificar su contenido técnico.
            </AlertDescription>
          </Alert>
        )}

        {/* Mensaje informativo para recepción */}
        {cotizacionActions.canApprove &&
          !cotizacionActions.canEdit &&
          estadoOrden === "cotizacion_enviada" && (
            <div className="bg-green-50 border border-green-200 rounded-md p-4 mb-4">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <svg
                    className="h-5 w-5 text-green-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clipRule="evenodd"
                    />
                  </svg>
                </div>
                <div className="ml-3">
                  <h3 className="text-sm font-medium text-green-800">
                    Cotización Lista para Aprobación
                  </h3>
                  <div className="mt-1 text-sm text-green-700">
                    Esta cotización ha sido enviada al cliente. Puede aprobarla
                    o rechazarla usando los botones al final del formulario.
                  </div>
                </div>
              </div>
            </div>
          )}

        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="informacion">Información General</TabsTrigger>
            <TabsTrigger value="terminos">Términos y Condiciones</TabsTrigger>
          </TabsList>

          <form onSubmit={handleSubmit} className="space-y-6">
            <TabsContent value="informacion" className="space-y-6 mt-6">
              {/* Datos básicos */}
              <div className="grid grid-cols-2 gap-4">
                {/* Código de cotización (solo lectura si existe) */}
                <div className="space-y-2">
                  <Label htmlFor="cotizacion_codigo">
                    Código de Cotización
                  </Label>
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
                    readOnly={shouldFieldsBeReadOnly()}
                    className={`${
                      errors.costo_revision ? "border-red-500" : ""
                    } ${shouldFieldsBeReadOnly() ? "bg-gray-50" : ""}`}
                  />
                  {errors.costo_revision && (
                    <p className="text-sm text-red-500">
                      {errors.costo_revision}
                    </p>
                  )}
                </div>

                {/* Costo de reparación */}
                <div className="space-y-2">
                  <Label htmlFor="costo_reparacion">
                    Costo de Reparación *
                  </Label>
                  <Input
                    id="costo_reparacion"
                    type="number"
                    min="0"
                    value={formData.costo_reparacion}
                    onChange={(e) =>
                      handleInputChange("costo_reparacion", e.target.value)
                    }
                    placeholder="0"
                    readOnly={shouldFieldsBeReadOnly()}
                    className={`${
                      errors.costo_reparacion ? "border-red-500" : ""
                    } ${shouldFieldsBeReadOnly() ? "bg-gray-50" : ""}`}
                  />
                  {errors.costo_reparacion && (
                    <p className="text-sm text-red-500">
                      {errors.costo_reparacion}
                    </p>
                  )}
                </div>
              </div>{" "}
              {/* Informe */}
              <div className="col-span-2 space-y-2">
                <Label htmlFor="informe">Informe *</Label>
                <Textarea
                  id="informe"
                  value={formData.informe}
                  onChange={(e) => handleInputChange("informe", e.target.value)}
                  placeholder="Redacte aquí el informe técnico de la cotización"
                  readOnly={shouldFieldsBeReadOnly()}
                  className={`${errors.informe ? "border-red-500" : ""} ${
                    shouldFieldsBeReadOnly() ? "bg-gray-50" : ""
                  }`}
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
                {!shouldFieldsBeReadOnly() && (
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
                )}

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
                              {!shouldFieldsBeReadOnly() && (
                                <Button
                                  type="button"
                                  variant="ghost"
                                  size="sm"
                                  onClick={() =>
                                    handleRemovePieza(pieza.pieza_id)
                                  }
                                >
                                  <Trash2 className="h-4 w-4" />
                                </Button>
                              )}
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
              )}
            </TabsContent>

            <TabsContent value="terminos" className="space-y-6 mt-6">
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-semibold flex items-center">
                    Términos y Condiciones de la Cotización
                    {!canModifyTerminos() && (
                      <span className="ml-2 px-2 py-1 text-xs bg-red-100 text-red-600 rounded-full">
                        🔒 Bloqueado
                      </span>
                    )}
                  </h3>

                  {/* Botón prominente para aplicar términos inmediatamente */}
                  {isEditing &&
                    cotizacion?.cotizacion_id &&
                    selectedTerminos.length > 0 && (
                      <Button
                        type="button"
                        onClick={handleActualizarTerminos}
                        disabled={loadingTerminos || !canModifyTerminos()}
                        size="sm"
                        className={`font-medium ${
                          !canModifyTerminos()
                            ? "bg-gray-400 cursor-not-allowed text-gray-700"
                            : "bg-blue-600 hover:bg-blue-700 text-white"
                        }`}
                      >
                        {loadingTerminos ? (
                          <>
                            <span className="animate-spin mr-2">⏳</span>
                            Actualizando...
                          </>
                        ) : (
                          <>
                            <span className="mr-2">💾</span>
                            Aplicar Términos Ahora
                          </>
                        )}
                      </Button>
                    )}
                </div>

                {!canModifyTerminos() && (
                  <div className="mb-4 p-3 bg-yellow-50 border-l-4 border-yellow-400 rounded">
                    <div className="flex">
                      <div className="ml-3">
                        <p className="text-sm text-yellow-700">
                          <strong>⚠️ Modificación restringida:</strong> No se
                          pueden modificar los términos y condiciones cuando la
                          orden está en estado "{estadoOrden}". Solo se permite
                          en estados iniciales.
                        </p>
                      </div>
                    </div>
                  </div>
                )}

                {loadingTerminos ? (
                  <div className="text-center py-4">
                    Cargando términos y condiciones...
                  </div>
                ) : (
                  <div className="space-y-4">
                    {terminosCondiciones.length === 0 ? (
                      <p className="text-gray-500">
                        No hay términos y condiciones disponibles
                      </p>
                    ) : (
                      <div className="border rounded-lg">
                        <Table>
                          <TableHeader>
                            <TableRow>
                              <TableHead className="w-16">Aplicar</TableHead>
                              <TableHead className="w-56">
                                Nombre del Término
                              </TableHead>
                              <TableHead className="w-96">
                                Descripción Completa
                              </TableHead>
                              <TableHead className="w-28">Estado</TableHead>
                            </TableRow>
                          </TableHeader>
                          <TableBody>
                            {terminosCondiciones
                              .sort((a, b) => {
                                // Ordenar: términos normales primero, por defecto al final
                                if (a.is_default && !b.is_default) return 1;
                                if (!a.is_default && b.is_default) return -1;
                                // Si ambos son del mismo tipo, mantener orden alfabético por nombre
                                return (a.termino_nombre || "").localeCompare(
                                  b.termino_nombre || ""
                                );
                              })
                              .map((termino, index, sortedArray) => {
                                const isSelected = selectedTerminos.includes(
                                  termino.termino_id
                                );
                                const isDefault = termino.is_default;

                                // Detectar si es el primer término por defecto para agregar separador
                                const previousTermino = sortedArray[index - 1];
                                const isFirstDefault =
                                  isDefault &&
                                  (!previousTermino ||
                                    !previousTermino.is_default);

                                return (
                                  <React.Fragment key={termino.termino_id}>
                                    {/* Fila separadora antes de los términos por defecto */}
                                    {isFirstDefault && (
                                      <TableRow className="bg-green-50">
                                        <TableCell
                                          colSpan={4}
                                          className="text-center py-2"
                                        >
                                          <div className="flex items-center justify-center space-x-2">
                                            <div className="h-px bg-green-300 flex-1"></div>
                                            <span className="text-sm font-medium text-green-700 px-3">
                                              Términos por Defecto
                                            </span>
                                            <div className="h-px bg-green-300 flex-1"></div>
                                          </div>
                                        </TableCell>
                                      </TableRow>
                                    )}
                                    <TableRow
                                      key={termino.termino_id}
                                      className={`h-24 ${
                                        isSelected
                                          ? "bg-blue-50 border-blue-200"
                                          : ""
                                      } ${
                                        isDefault
                                          ? "border-l-4 border-l-green-500"
                                          : ""
                                      }`}
                                    >
                                      <TableCell className="align-top">
                                        <input
                                          type="checkbox"
                                          checked={
                                            isSelected || termino.is_default
                                          }
                                          disabled={!canModifyTerminos()}
                                          onChange={(e) => {
                                            // Prevenir desseleccionar términos por defecto
                                            if (
                                              !e.target.checked &&
                                              termino.is_default
                                            ) {
                                              showError(
                                                "Término requerido",
                                                `El término "${termino.termino_nombre}" es obligatorio y no puede ser deseleccionado`
                                              );
                                              return;
                                            }

                                            if (e.target.checked) {
                                              setSelectedTerminos((prev) => [
                                                ...prev,
                                                termino.termino_id,
                                              ]);
                                            } else {
                                              setSelectedTerminos((prev) =>
                                                prev.filter(
                                                  (id) =>
                                                    id !== termino.termino_id
                                                )
                                              );
                                            }
                                          }}
                                          className="rounded w-4 h-4"
                                        />
                                      </TableCell>
                                      <TableCell className="align-top">
                                        <div className="space-y-1">
                                          <p className="font-semibold text-sm">
                                            {termino.termino_nombre ||
                                              "Término General"}
                                          </p>
                                          <p className="text-xs text-gray-500">
                                            Tipo:{" "}
                                            {termino.tipo_referencia ||
                                              "General"}
                                          </p>
                                          {isDefault && (
                                            <span className="inline-flex items-center px-1.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                              ✓ Por defecto
                                            </span>
                                          )}
                                        </div>
                                      </TableCell>
                                      <TableCell className="align-top">
                                        <div className="max-w-lg max-h-20 overflow-y-auto border rounded-sm bg-gray-50 p-2 scrollbar-thin scrollbar-thumb-gray-300 hover:scrollbar-thumb-gray-400">
                                          <p className="text-sm text-gray-700 leading-relaxed">
                                            {termino.termino_descripcion ||
                                              "Sin descripción disponible"}
                                          </p>
                                        </div>
                                      </TableCell>
                                      <TableCell className="align-top">
                                        <div className="text-center">
                                          {aplicadosTerminos.includes(
                                            termino.termino_id
                                          ) ? (
                                            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                              ✓ Aplicado
                                            </span>
                                          ) : isSelected ? (
                                            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                              Seleccionado
                                            </span>
                                          ) : (
                                            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-600">
                                              No aplicado
                                            </span>
                                          )}
                                        </div>
                                      </TableCell>
                                    </TableRow>
                                  </React.Fragment>
                                );
                              })}
                          </TableBody>
                        </Table>
                      </div>
                    )}

                    {selectedTerminos.length === 0 &&
                      terminosCondiciones.length > 0 && (
                        <div className="bg-yellow-50 p-4 rounded-lg border border-yellow-200">
                          <div className="flex items-start space-x-2">
                            <span className="text-yellow-500 text-lg">⚠️</span>
                            <div className="flex-1">
                              <p className="text-sm text-yellow-800 font-medium">
                                No hay términos y condiciones seleccionados
                              </p>
                              <p className="text-xs text-yellow-700 mt-1">
                                Se recomienda seleccionar al menos los términos
                                por defecto para la cotización.
                              </p>

                              {/* Botón para aplicar términos por defecto manualmente */}
                              <div className="mt-3">
                                <Button
                                  type="button"
                                  onClick={() => {
                                    const terminosDefecto = terminosCondiciones
                                      .filter((termino) => termino.is_default)
                                      .map((termino) => termino.termino_id);
                                    if (terminosDefecto.length > 0) {
                                      setSelectedTerminos(terminosDefecto);
                                      success(
                                        "Términos aplicados",
                                        `Se han seleccionado ${terminosDefecto.length} términos por defecto`
                                      );
                                    } else {
                                      showError(
                                        "Sin términos",
                                        "No hay términos marcados como por defecto"
                                      );
                                    }
                                  }}
                                  size="sm"
                                  className="bg-yellow-600 hover:bg-yellow-700 text-white"
                                >
                                  ✨ Aplicar Términos por Defecto
                                </Button>
                              </div>
                            </div>
                          </div>
                        </div>
                      )}
                  </div>
                )}
              </div>
            </TabsContent>

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

              {isEditing && cotizacion?.is_borrador && (
                <Button
                  type="button"
                  variant="destructive"
                  onClick={() => setShowRechazarConfirmDialog(true)}
                  disabled={loading}
                  className="bg-red-600 hover:bg-red-700"
                >
                  {loading ? "Eliminando..." : "Eliminar Borrador"}
                </Button>
              )}

              {isEditing &&
                estadoOrden &&
                estadoOrden
                  .toLowerCase()
                  .normalize("NFD")
                  .replace(/[\u0300-\u036f]/g, "")
                  .replace(/\s+/g, " ")
                  .trim() === "cotizacion_enviada" &&
                cotizacionActions.canApprove && (
                  <>
                    <Button
                      type="button"
                      variant="default"
                      onClick={() => setShowAprobarConfirmDialog(true)}
                      disabled={loading}
                      className="bg-green-600 hover:bg-green-700"
                    >
                      {loading ? "Aprobando..." : "Aprobar Cotización"}
                    </Button>
                    <Button
                      type="button"
                      variant="destructive"
                      onClick={() => setShowRechazarConfirmDialog(true)}
                      disabled={loading}
                      className="bg-red-600 hover:bg-red-700"
                    >
                      {loading ? "Rechazando..." : "Rechazar Cotización"}
                    </Button>
                  </>
                )}

              {isEditing &&
                estadoOrden &&
                estadoOrden.toLowerCase().trim() === "recibido" && (
                  <Button
                    type="button"
                    variant="destructive"
                    onClick={() => setShowNoReparableConfirmDialog(true)}
                    disabled={loading}
                    className="bg-gray-800 hover:bg-gray-900"
                  >
                    {loading ? "Procesando..." : "No Reparable"}
                  </Button>
                )}

              {isEditing &&
                estadoOrden.toLowerCase() !== "recibido" &&
                estadoOrden.toLowerCase() !== "abandonado" &&
                puedeAbandonar && ( // Solo si han pasado más de 168 horas
                  <Button
                    type="button"
                    variant="destructive"
                    onClick={() => setShowAbandonoConfirmDialog(true)}
                    disabled={loading}
                    className="bg-orange-600 hover:bg-orange-700"
                  >
                    Declarar Abandono
                  </Button>
                )}

              <Button type="submit" disabled={loading}>
                {loading ? "Guardando..." : isEditing ? "Actualizar" : "Crear"}
              </Button>
            </DialogFooter>
          </form>
        </Tabs>
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

      {/* Modal de confirmación de aprobación */}
      <Dialog
        open={showAprobarConfirmDialog}
        onOpenChange={setShowAprobarConfirmDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar aprobación</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea aprobar esta cotización?
              <br />
              El estado de la orden cambiará a <b>En reparación</b>.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowAprobarConfirmDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={async () => {
                setShowAprobarConfirmDialog(false);
                await handleAprobarCotizacion();
              }}
              disabled={loading}
              className="bg-green-600 hover:bg-green-700"
            >
              {loading ? "Aprobando..." : "Confirmar aprobación"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación de rechazo */}
      <Dialog
        open={showRechazarConfirmDialog}
        onOpenChange={setShowRechazarConfirmDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar rechazo</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea <b>rechazar</b> esta cotización?
              <br />
              El estado de la orden cambiará a <b>Aprobación Pendiente</b>.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowRechazarConfirmDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={async () => {
                setShowRechazarConfirmDialog(false);
                await handleRechazarCotizacion();
              }}
              disabled={loading}
              className="bg-red-600 hover:bg-red-700"
            >
              {loading ? "Rechazando..." : "Confirmar rechazo"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación de rechazo en estado Borrador */}
      {isEditing && cotizacion?.is_borrador && (
        <Dialog
          open={showRechazarConfirmDialog}
          onOpenChange={setShowRechazarConfirmDialog}
        >
          <DialogContent className="max-w-md">
            <DialogHeader>
              <DialogTitle>Confirmar rechazo</DialogTitle>
              <DialogDescription>
                ¿Está seguro que desea <b>rechazar</b> esta cotización?
                <br />
                El estado de la cotización cambiará a{" "}
                <b>Cotización Rechazada</b>.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-3">
              <Label htmlFor="motivo_rechazo">Motivo del rechazo *</Label>
              <Textarea
                id="motivo_rechazo"
                value={motivoRechazo}
                onChange={(e) => setMotivoRechazo(e.target.value)}
                placeholder="Ingrese el motivo del rechazo..."
                rows={3}
                required
              />
            </div>
            <DialogFooter className="gap-2">
              <Button
                type="button"
                variant="outline"
                onClick={() => setShowRechazarConfirmDialog(false)}
                disabled={loading}
              >
                Cancelar
              </Button>
              <Button
                type="button"
                onClick={async () => {
                  setShowRechazarConfirmDialog(false);
                  await handleRechazarCotizacionBorrador(motivoRechazo);
                }}
                disabled={loading || !motivoRechazo.trim()}
                className="bg-red-600 hover:bg-red-700"
              >
                {loading ? "Eliminando..." : "Confirmar Eliminación"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      )}

      {/* Modal de confirmación de No Reparable */}
      <Dialog
        open={showNoReparableConfirmDialog}
        onOpenChange={setShowNoReparableConfirmDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar No Reparable</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea declarar este equipo como{" "}
              <b>No Reparable</b>?
              <br />
              Debe justificar su decisión en el campo de comentario.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-3">
            <Label htmlFor="comentario">Comentario *</Label>
            <Textarea
              id="comentario"
              value={comentarioNoReparable}
              onChange={(e) => setComentarioNoReparable(e.target.value)}
              placeholder="Explique por qué el equipo no es reparable..."
              rows={4}
              required
            />
          </div>

          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowNoReparableConfirmDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={async () => {
                setShowNoReparableConfirmDialog(false);
                await handleNoReparable();
              }}
              disabled={loading}
              className="bg-gray-800 hover:bg-gray-900"
            >
              {loading ? "Procesando..." : "Confirmar No Reparable"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <Dialog
        open={showAbandonoConfirmDialog}
        onOpenChange={setShowAbandonoConfirmDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Abandono</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea declarar este equipo como <b>Abandonado</b>
              ? Por favor, ingrese un comentario justificando esta acción.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2">
            <Label htmlFor="abandono_comentario">Comentario *</Label>
            <Textarea
              id="abandono_comentario"
              value={abandonoComentario}
              onChange={(e) => setAbandonoComentario(e.target.value)}
              placeholder="Ingrese la justificación del abandono"
              rows={4}
              className="border-red-500 focus:border-red-600"
            />
          </div>

          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowAbandonoConfirmDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={async () => {
                setShowAbandonoConfirmDialog(false);
                await handleAbandonarEquipo();
              }}
              disabled={loading}
              className="bg-orange-600 hover:bg-orange-700"
            >
              {loading ? "Procesando..." : "Confirmar Abandono"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}
