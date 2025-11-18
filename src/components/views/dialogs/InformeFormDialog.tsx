import React, { useState, useEffect, useCallback } from "react";
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
  is_borrador?: boolean;
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
  const { isRecepcion, canEditInforme } = useOrdenTrabajoPermissions();
  const [loading, setLoading] = useState(false);
  const [loadingSendToClient, setLoadingSendToClient] = useState(false);
  const [loadingSendExisting, setLoadingSendExisting] = useState(false);
  const [piezas, setPiezas] = useState<Pieza[]>([]);
  const [loadingPiezas, setLoadingPiezas] = useState(false);
  const [selectedPiezas, setSelectedPiezas] = useState<SelectedPieza[]>([]);
  const [selectedPiezaId, setSelectedPiezaId] = useState<string>("");
  const [cantidad, setCantidad] = useState<string>("1");
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  const [showEliminarInformeDialog, setShowEliminarInformeDialog] =
    useState(false);
  const [motivoEliminacion, setMotivoEliminacion] = useState("");
  const [activeTab, setActiveTab] = useState("informacion");
  const [terminosCondiciones, setTerminosCondiciones] = useState<any[]>([]);
  const [loadingTerminos, setLoadingTerminos] = useState(false);
  const [selectedTerminos, setSelectedTerminos] = useState<number[]>([]);
  const [aplicadosTerminos, setAplicadosTerminos] = useState<number[]>([]); // Términos realmente guardados en BD
  const [estadoOrden, setEstadoOrden] = useState<string>("");
  const [formData, setFormData] = useState<FormData>({
    diagnostico: "",
    recomendaciones: "",
    solucion_aplicada: "",
    tecnico_responsable: "",
  });

  // Función para determinar si se pueden modificar términos según el estado
  const canModifyTerminos = () => {
    if (!ordenTrabajoId || !estadoOrden) return true; // Si no hay orden, permitir modificación

    // Estados donde NO se pueden modificar términos de informe
    const estadosBloqueados = ["espera_de_retiro", "entregado"];

    return !estadosBloqueados.includes(estadoOrden);
  };

  const [errors, setErrors] = useState<FormErrors>({});
  // Estado para el diálogo de confirmación de eliminación
  // Función para manejar la eliminación del informe

  const loadTerminosCondiciones = async () => {
    try {
      setLoadingTerminos(true);
      // Cargar solo términos aplicables a informes
      const terminos = await invoke<any[]>("get_terminos_condiciones_by_tipo", {
        tipo: "informe",
      });
      console.log(
        "📋 Términos cargados para informes:",
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

  const loadTerminosInforme = async () => {
    if (!informe?.informe_id) return;
    try {
      console.log("🔍 Cargando términos del informe:", informe.informe_id);

      const terminosInforme = await invoke<any[]>("get_terminos_by_informe", {
        informeId: informe.informe_id,
      });

      console.log("📋 Términos recibidos del backend:", terminosInforme);

      // Extraer solo los IDs de los términos devueltos por el backend
      const terminoIds = terminosInforme.map((t) => t.termino_id);

      console.log("🎯 IDs de términos aplicados:", terminoIds);

      setSelectedTerminos(terminoIds);
      setAplicadosTerminos(terminoIds); // Los que están en BD son los realmente aplicados

      console.log("✅ Estado actualizado correctamente");
    } catch (error) {
      console.error("❌ Error cargando términos del informe:", error);
      showError("Error", "No se pudieron cargar los términos del informe");
    }
  };

  useEffect(() => {
    if (open) {
      loadPiezas();
      loadTerminosCondiciones();

      if (isEditing && informe) {
        loadInformePiezas();
        loadTerminosInforme();
      }

      // Cargar estado de la orden si existe
      if (ordenTrabajoId) {
        invoke<{ estado: string }>("get_orden_trabajo_by_id", {
          ordenId: ordenTrabajoId,
        })
          .then((orden) => {
            setEstadoOrden(orden.estado);
          })
          .catch((err) => {
            console.error("Error obteniendo estado de orden:", err);
          });
      }
    }
  }, [open, ordenTrabajoId, isEditing]);

  // Aplicar términos por defecto cuando se tienen los datos necesarios
  useEffect(() => {
    console.log("🔍 InformeFormDialog - useEffect términos por defecto:", {
      isEditing,
      terminosCondicionesLength: terminosCondiciones.length,
      selectedTerminosLength: selectedTerminos.length,
      open,
      terminosCondiciones: terminosCondiciones.map((t) => ({
        id: t.termino_id,
        nombre: t.termino_nombre,
        isDefault: t.is_default,
      })),
    });

    if (
      !isEditing &&
      terminosCondiciones.length > 0 &&
      selectedTerminos.length === 0 &&
      open
    ) {
      const terminosDefecto = terminosCondiciones
        .filter((termino) => termino.is_default)
        .map((termino) => termino.termino_id);
      console.log(
        "✅ Condiciones cumplidas. Términos por defecto encontrados:",
        terminosDefecto
      );
      if (terminosDefecto.length > 0) {
        console.log(
          "🎯 Aplicando términos por defecto automáticamente:",
          terminosDefecto
        );
        setSelectedTerminos(terminosDefecto);
      } else {
        console.log("⚠️ No se encontraron términos marcados como por defecto");
      }
    } else {
      console.log(
        "❌ No se aplican términos por defecto porque no se cumplieron las condiciones"
      );
    }
  }, [terminosCondiciones, selectedTerminos, isEditing, open]);

  // Función para cargar datos de la orden de trabajo (piezas y diagnóstico de la cotización)
  const loadDataFromOrdenTrabajo = useCallback(async () => {
    if (!ordenTrabajoId) {
      console.log("⚠️ loadDataFromOrdenTrabajo: No hay ordenTrabajoId");
      return;
    }

    try {
      console.log("🔄 Cargando datos de la orden de trabajo:", ordenTrabajoId);

      // Primero obtener la orden de trabajo para conseguir la cotización asociada
      const ordenTrabajo = await invoke<any>("get_orden_trabajo_by_id", {
        ordenId: ordenTrabajoId,
      });

      console.log("📋 Orden de trabajo obtenida:", ordenTrabajo);

      if (!ordenTrabajo?.cotizacion_id) {
        console.log("⚠️ La orden de trabajo no tiene una cotización asociada");
        return;
      }

      console.log("✅ Cotización encontrada:", ordenTrabajo.cotizacion_id);

      // Obtener la cotización completa para acceder al campo informe
      const cotizacion = await invoke<any>("get_cotizacion_by_id", {
        cotizacionId: ordenTrabajo.cotizacion_id,
      });

      console.log("📋 Cotización obtenida:", cotizacion);

      // Cargar el diagnóstico desde el campo informe de la cotización
      if (cotizacion?.informe) {
        setFormData((prev) => ({
          ...prev,
          diagnostico: cotizacion.informe,
        }));
        console.log("✅ Diagnóstico cargado desde la cotización asociada");
      }

      // Obtener las piezas de la cotización
      console.log("🔄 Obteniendo piezas de la cotización...");
      const piezasCotizacion = await invoke<any[]>("get_piezas_cotizacion", {
        cotizacionId: ordenTrabajo.cotizacion_id,
      });

      console.log(
        "📦 Piezas obtenidas de la cotización (raw):",
        piezasCotizacion
      );
      console.log("📦 Cantidad de piezas:", piezasCotizacion?.length || 0);

      if (!piezasCotizacion || piezasCotizacion.length === 0) {
        console.log("⚠️ No se encontraron piezas en la cotización");
        return;
      }

      // Convertir las piezas de cotización a formato de piezas de informe
      const selectedPiezasWithDetails: SelectedPieza[] = piezasCotizacion
        .filter((pc) => {
          const isValid = pc && pc.pieza_id;
          if (!isValid) {
            console.log("⚠️ Pieza inválida filtrada:", pc);
          }
          return isValid;
        })
        .map((pc) => {
          const pieza = {
            pieza_id: pc.pieza_id,
            informe_id: 0, // Se asignará cuando se cree el informe
            cantidad: pc.cantidad ?? 1, // Usar nullish coalescing para manejar null/undefined
            pieza_nombre: pc.pieza_nombre || "Nombre no disponible",
            pieza_marca: pc.pieza_marca || undefined,
            pieza_precio: pc.pieza_precio ?? 0, // Usar nullish coalescing para manejar null/undefined
          };
          console.log("🔧 Pieza convertida:", pieza);
          return pieza;
        });

      console.log("✅ Piezas convertidas (final):", selectedPiezasWithDetails);
      console.log(
        "✅ Cantidad de piezas convertidas:",
        selectedPiezasWithDetails.length
      );

      setSelectedPiezas(selectedPiezasWithDetails);

      if (selectedPiezasWithDetails.length > 0) {
        console.log(
          `✅ Se cargaron ${selectedPiezasWithDetails.length} pieza(s) de la cotización asociada`
        );
        success(
          "Piezas cargadas",
          `Se cargaron ${selectedPiezasWithDetails.length} pieza(s) de la cotización asociada a esta orden de trabajo.`
        );
      } else {
        console.log("⚠️ No se encontraron piezas válidas después del filtrado");
      }
    } catch (error) {
      console.error("❌ Error cargando datos de la orden de trabajo:", error);
      showError(
        "Error",
        "No se pudieron cargar las piezas de la cotización asociada."
      );
    }
  }, [ordenTrabajoId, success, showError]);

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
      // porque se cargarán automáticamente desde loadDataFromOrdenTrabajo
      if (!ordenTrabajoId) {
        setSelectedPiezas([]);
      }
      // Resetear términos seleccionados para que loadTerminosCondiciones pueda aplicar los por defecto
      setSelectedTerminos([]);
    }
    setErrors({});
  }, [isEditing, informe, open, user, ordenTrabajoId]);

  // Cargar piezas de la cotización cuando se abre el modal para crear nuevo informe
  useEffect(() => {
    if (open && !isEditing && ordenTrabajoId) {
      console.log(
        "🔄 useEffect: Cargando piezas desde cotización, ordenTrabajoId:",
        ordenTrabajoId
      );
      // Usar un pequeño delay para asegurar que el formulario se haya reseteado
      const timer = setTimeout(() => {
        loadDataFromOrdenTrabajo();
      }, 100);
      return () => clearTimeout(timer);
    }
  }, [open, isEditing, ordenTrabajoId, loadDataFromOrdenTrabajo]);

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
  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.diagnostico.trim()) {
      newErrors.diagnostico = "El diagnóstico es requerido";
    }

    if (!formData.solucion_aplicada.trim()) {
      newErrors.solucion_aplicada = "La solución aplicada es requerida";
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

  const handleActualizarTerminos = async () => {
    console.log("🔍 Iniciando actualización de términos:", {
      informeId: informe?.informe_id,
      selectedTerminos,
      selectedTerminosLength: selectedTerminos.length,
    });

    // Verificar si se pueden modificar términos según el estado
    if (!canModifyTerminos()) {
      showError(
        "Modificación no permitida",
        `No se pueden modificar los términos y condiciones cuando la orden está en estado "${estadoOrden}". Los términos no se pueden modificar cuando el equipo está listo para entrega o ya fue entregado.`
      );
      return;
    }

    if (!informe?.informe_id) {
      showError(
        "Error",
        "No se puede actualizar términos sin un informe guardado"
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

      console.log("📡 Llamando comando apply_terminos_to_informe con:", {
        informeId: informe.informe_id,
        terminos: terminoRequests,
        appliedBy: user?.usuario_id || 1,
      });

      // Aplicar términos seleccionados al informe
      await invoke("apply_terminos_to_informe", {
        informeId: informe.informe_id,
        terminos: terminoRequests,
        appliedBy: user?.usuario_id || 1,
      });

      console.log("✅ Comando ejecutado exitosamente");

      // Primero actualizar el estado local inmediatamente
      setAplicadosTerminos([...selectedTerminos]);

      success(
        "Términos actualizados",
        `Se han aplicado ${selectedTerminos.length} términos y condiciones al informe`
      );

      // Recargar términos para mostrar el estado actualizado desde la base de datos
      console.log("🔄 Recargando términos desde la base de datos...");

      // Pequeño delay para asegurar que la transacción se complete
      await new Promise((resolve) => setTimeout(resolve, 100));

      await loadTerminosInforme();
      console.log("✅ Términos recargados exitosamente");

      // Verificación final
      setTimeout(async () => {
        await loadTerminosInforme();
        console.log("🔍 Verificación final completada");
      }, 500);
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
          updatedBy: user.usuario_id,
        });

        if (result) {
          // Aplicar términos y condiciones seleccionados
          if (selectedTerminos.length > 0) {
            try {
              await invoke("apply_terminos_to_informe", {
                informeId: informe.informe_id,
                terminoIds: selectedTerminos,
              });
            } catch (error) {
              console.error("Error aplicando términos y condiciones:", error);
              showError(
                "Advertencia",
                "El informe se actualizó pero no se pudieron aplicar todos los términos y condiciones."
              );
            }
          }

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
          // Campos nuevos (principales)
          diagnostico: formData.diagnostico,
          recomendaciones: formData.recomendaciones.trim() || undefined,
          solucion_aplicada: formData.solucion_aplicada.trim() || undefined,
          tecnico_responsable: formData.tecnico_responsable,
          // Campos antiguos para compatibilidad con el backend
          informe_acciones: formData.diagnostico, // Mapear diagnóstico a informe_acciones
          informe_obs: formData.recomendaciones.trim() || undefined, // Mapear recomendaciones a informe_obs
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

        let asociadoAOrden = false; // Si se proporciona ordenTrabajoId, asociar el informe a la orden
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

        // Aplicar términos y condiciones seleccionados
        if (selectedTerminos.length > 0) {
          try {
            await invoke("apply_terminos_to_informe", {
              informeId: informeId,
              terminoIds: selectedTerminos,
            });
          } catch (error) {
            console.error("Error aplicando términos y condiciones:", error);
            showError(
              "Advertencia",
              "El informe se creó pero no se pudieron aplicar todos los términos y condiciones."
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
      setShowConfirmationDialog(false);
    }
  };

  const handleEliminarInformeBorrador = async (motivo: string) => {
    if (!informe?.informe_id || !user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }
    try {
      setLoading(true);
      // Elimina el informe y desvincula de la orden
      const result = await invoke<boolean>("rechazar_informe_borrador", {
        informeId: informe.informe_id,
        motivoEliminacion: motivo,
        updatedBy: user.usuario_id,
      });
      if (result) {
        success(
          "Informe eliminado",
          "El informe en borrador ha sido eliminado."
        );
        onInformeAdded(); // Refresca la lista
        onOpenChange(false); // Cierra el diálogo
      } else {
        showError("Error", "No se pudo eliminar el informe en borrador.");
      }
    } catch (error) {
      console.error(error);
      showError("Error", "Hubo un error al eliminar el informe.");
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
      setLoadingSendToClient(true); // Crear el informe primero (solo para creación nueva)
      if (!isEditing) {
        const createData = {
          // Campos nuevos (principales)
          diagnostico: formData.diagnostico,
          recomendaciones: formData.recomendaciones.trim() || undefined,
          solucion_aplicada: formData.solucion_aplicada.trim() || undefined,
          tecnico_responsable: formData.tecnico_responsable,
          // Campos antiguos para compatibilidad con el backend
          informe_acciones: formData.diagnostico, // Mapear diagnóstico a informe_acciones
          informe_obs: formData.recomendaciones.trim() || undefined, // Mapear recomendaciones a informe_obs
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

          // Cambiar el estado de la orden a "espera_de_retiro" para activar el envío automático del correo con PDF
          if (ordenTrabajoId && asociadoAOrden) {
            try {
              await invoke("cambiar_estado_orden_trabajo", {
                ordenId: ordenTrabajoId,
                nuevoEstado: "espera_de_retiro",
                updatedBy: user.usuario_id,
              });
              console.log(
                "✅ Estado cambiado a 'espera_de_retiro' - el correo con PDF se enviará automáticamente"
              );
            } catch (error) {
              console.error(
                "Error cambiando estado a 'espera_de_retiro':",
                error
              );
              // No mostramos error al usuario porque el informe ya se envió
              // El correo con PDF se puede enviar manualmente después si es necesario
            }
          }

          success(
            "Informe creado y enviado",
            `El informe ha sido creado y enviado al cliente exitosamente.` +
              (ordenTrabajoId && asociadoAOrden
                ? " El estado de la orden se ha actualizado a 'Espera de Retiro' y el cliente recibirá un correo con el informe PDF adjunto."
                : ordenTrabajoId
                ? " (No se pudo asociar a la orden de trabajo)"
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
  const handleSendExistingToClient = async () => {
    if (!user || !informe || !isEditing) {
      showError("Error", "No se puede enviar el informe");
      return;
    }

    try {
      setLoadingSendExisting(true);

      // Enviar el informe existente al cliente
      await invoke<boolean>("send_informe_to_client", {
        informeId: informe.informe_id,
        sentBy: user.usuario_id,
      });

      // Cambiar el estado de la orden a "espera_de_retiro" para activar el envío automático del correo con PDF
      if (ordenTrabajoId) {
        try {
          await invoke("cambiar_estado_orden_trabajo", {
            ordenId: ordenTrabajoId,
            nuevoEstado: "espera_de_retiro",
            updatedBy: user.usuario_id,
          });
          console.log(
            "✅ Estado cambiado a 'espera_de_retiro' - el correo con PDF se enviará automáticamente"
          );
        } catch (error) {
          console.error("Error cambiando estado a 'espera_de_retiro':", error);
          // No mostramos error al usuario porque el informe ya se envió
        }
      }

      // Actualizar el estado del informe para que ya no sea borrador
      await invoke<boolean>("update_informe", {
        informeId: informe.informe_id,
        request: { is_borrador: false },
        updatedBy: user.usuario_id,
      });

      success(
        "Informe enviado",
        `El informe ha sido enviado al cliente exitosamente.` +
          (ordenTrabajoId
            ? " El estado de la orden se ha actualizado a 'Espera de Retiro' y el cliente recibirá un correo con el informe PDF adjunto."
            : "")
      );

      onInformeAdded(); // Recargar la vista
      onOpenChange(false); // Cerrar el modal
    } catch (error) {
      console.error("Error enviando informe al cliente:", error);
      showError(
        "Error al enviar informe",
        error instanceof Error
          ? error.message
          : "No se pudo enviar el informe al cliente."
      );
    } finally {
      setLoadingSendExisting(false);
    }
  };

  const getPiezaDisplayName = (pieza: Pieza) => {
    const parts = [];
    if (pieza.pieza_nombre) parts.push(pieza.pieza_nombre);
    if (pieza.pieza_marca) parts.push(`(${pieza.pieza_marca})`);
    return parts.length > 0 ? parts.join(" ") : `Pieza ${pieza.pieza_id}`;
  };

  // Verificar permisos para recepción - solo lectura
  if (isRecepcion && !canEditInforme) {
    return (
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Informe - Solo Lectura</DialogTitle>
            <DialogDescription>
              Como usuario de recepción, solo puede visualizar los informes
              existentes. No tiene permisos para crear o editar informes.
            </DialogDescription>
          </DialogHeader>
          {informe && (
            <div className="space-y-4">
              <div>
                <strong>Código:</strong> {informe.informe_codigo || "N/A"}
              </div>
              <div>
                <strong>Técnico Responsable:</strong>{" "}
                {informe.tecnico_responsable || "N/A"}
              </div>
              <div>
                <strong>Diagnóstico:</strong>
                <div className="mt-1 p-2 bg-gray-50 rounded text-sm">
                  {informe.diagnostico || "N/A"}
                </div>
              </div>
              <div>
                <strong>Recomendaciones:</strong>
                <div className="mt-1 p-2 bg-gray-50 rounded text-sm">
                  {informe.recomendaciones || "N/A"}
                </div>
              </div>
              <div>
                <strong>Solución Aplicada:</strong>
                <div className="mt-1 p-2 bg-gray-50 rounded text-sm">
                  {informe.solucion_aplicada || "N/A"}
                </div>
              </div>
            </div>
          )}
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
      <DialogContent className="max-w-[95vw] sm:max-w-lg md:max-w-2xl lg:max-w-4xl max-h-[90vh] overflow-y-auto">
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

        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="informacion">Información General</TabsTrigger>
            <TabsTrigger value="terminos">Términos y Condiciones</TabsTrigger>
          </TabsList>

          <form onSubmit={handleSubmit} className="space-y-6">
            <TabsContent value="informacion" className="space-y-6 mt-6">
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
                </div>{" "}
                <div className="space-y-2">
                  <Label htmlFor="tecnico_responsable">
                    Técnico Responsable <span className="text-red-500">*</span>
                  </Label>
                  <Input
                    id="tecnico_responsable"
                    value={formData.tecnico_responsable}
                    placeholder="Nombre del técnico responsable"
                    disabled
                    className={`bg-gray-100 ${
                      errors.tecnico_responsable ? "border-red-500" : ""
                    }`}
                  />
                  {errors.tecnico_responsable && (
                    <p className="text-sm text-red-500">
                      {errors.tecnico_responsable}
                    </p>
                  )}{" "}
                </div>
              </div>{" "}
              <div className="space-y-2">
                <Label htmlFor="solucion_aplicada">
                  Solución Aplicada <span className="text-red-500">*</span>
                </Label>
                <Textarea
                  id="solucion_aplicada"
                  value={formData.solucion_aplicada}
                  onChange={(e) =>
                    handleInputChange("solucion_aplicada", e.target.value)
                  }
                  placeholder="Describa la solución aplicada..."
                  rows={3}
                  className={errors.solucion_aplicada ? "border-red-500" : ""}
                />
                {errors.solucion_aplicada && (
                  <p className="text-sm text-red-500">
                    {errors.solucion_aplicada}
                  </p>
                )}
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
                      {" "}
                      <TableHeader>
                        <TableRow>
                          <TableHead>Pieza</TableHead>
                          <TableHead>Marca</TableHead>
                          <TableHead>Cantidad</TableHead>
                          <TableHead>Acciones</TableHead>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        {selectedPiezas.map((pieza) => (
                          <TableRow key={pieza.pieza_id}>
                            <TableCell>{pieza.pieza_nombre}</TableCell>
                            <TableCell>
                              {pieza.pieza_marca || "N/A"}
                            </TableCell>{" "}
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
                              <Button
                                type="button"
                                variant="outline"
                                size="sm"
                                onClick={() =>
                                  handleRemovePieza(pieza.pieza_id)
                                }
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
              </div>
            </TabsContent>

            <TabsContent value="terminos" className="space-y-6 mt-6">
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-semibold flex items-center">
                    Términos y Condiciones del Informe
                    {!canModifyTerminos() && (
                      <span className="ml-2 px-2 py-1 text-xs bg-red-100 text-red-600 rounded-full">
                        🔒 Bloqueado
                      </span>
                    )}
                  </h3>

                  {/* Botón prominente para aplicar términos inmediatamente */}
                  {isEditing &&
                    informe?.informe_id &&
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
                          orden está en estado "{estadoOrden}". Los términos no
                          se pueden modificar cuando el equipo está listo para
                          entrega o ya fue entregado.
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
                                por defecto para el informe.
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
                                  Aplicar Términos por Defecto
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

            <DialogFooter>
              <Button
                type="button"
                variant="outline"
                onClick={() => onOpenChange(false)}
                disabled={loading || loadingSendToClient || loadingSendExisting}
              >
                Cancelar
              </Button>{" "}
              <Button
                type="submit"
                disabled={loading || loadingSendToClient || loadingSendExisting}
              >
                {loading
                  ? "Procesando..."
                  : isEditing
                  ? "Actualizar Informe"
                  : "Guardar Informe"}
              </Button>
              {isEditing && informe?.is_borrador && (
                <Button
                  type="button"
                  onClick={handleSendExistingToClient}
                  disabled={
                    loading || loadingSendToClient || loadingSendExisting
                  }
                  className="bg-blue-600 hover:bg-blue-700"
                >
                  {loadingSendExisting
                    ? "Enviando..."
                    : "Enviar Informe a Cliente"}
                </Button>
              )}
              {isEditing && informe?.is_borrador && (
                <Button
                  type="button"
                  variant="destructive"
                  onClick={() => setShowEliminarInformeDialog(true)}
                  disabled={loading}
                  className="bg-red-600 hover:bg-red-700"
                >
                  {loading ? "Eliminando..." : "Eliminar Informe"}
                </Button>
              )}
              {!isEditing && (
                <Button
                  type="button"
                  onClick={handleSubmitAndSendToClient}
                  disabled={
                    loading || loadingSendToClient || loadingSendExisting
                  }
                  className="bg-green-600 hover:bg-green-700"
                >
                  {loadingSendToClient
                    ? "Enviando..."
                    : "Guardar y Enviar al Cliente"}
                </Button>
              )}
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
                : "Confirmar Creación de Informe"}
            </DialogTitle>
            <DialogDescription>
              {isEditing
                ? "¿Está seguro que desea actualizar este informe con los cambios realizados?"
                : "¿Está seguro que desea crear este informe con la siguiente información?"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2 text-sm">
            <div>
              <strong>Técnico Responsable:</strong>{" "}
              {formData.tecnico_responsable}
            </div>
            <div>
              <strong>Piezas:</strong> {selectedPiezas.length} pieza(s)
              utilizada(s)
            </div>
            {formData.diagnostico && (
              <div>
                <strong>Diagnóstico:</strong>{" "}
                {formData.diagnostico.length > 50
                  ? formData.diagnostico.substring(0, 50) + "..."
                  : formData.diagnostico}
              </div>
            )}
            {formData.solucion_aplicada && (
              <div>
                <strong>Solución:</strong>{" "}
                {formData.solucion_aplicada.length > 50
                  ? formData.solucion_aplicada.substring(0, 50) + "..."
                  : formData.solucion_aplicada}
              </div>
            )}
            {formData.recomendaciones && (
              <div>
                <strong>Recomendaciones:</strong>{" "}
                {formData.recomendaciones.length > 50
                  ? formData.recomendaciones.substring(0, 50) + "..."
                  : formData.recomendaciones}
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
      {/* Modal de eliminar informe */}
      <Dialog
        open={showEliminarInformeDialog}
        onOpenChange={setShowEliminarInformeDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar eliminación</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea <b>eliminar</b> este informe en borrador?
              <br />
              Podrá crear un nuevo informe para esta orden.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-3">
            <Label htmlFor="motivo_eliminacion">
              Motivo de la eliminación *
            </Label>
            <Textarea
              id="motivo_eliminacion"
              value={motivoEliminacion}
              onChange={(e) => setMotivoEliminacion(e.target.value)}
              placeholder="Ingrese el motivo de la eliminación..."
              rows={3}
              required
            />
          </div>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowEliminarInformeDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={async () => {
                setShowEliminarInformeDialog(false);
                await handleEliminarInformeBorrador(motivoEliminacion);
              }}
              disabled={loading || !motivoEliminacion.trim()}
              className="bg-red-600 hover:bg-red-700"
            >
              {loading ? "Eliminando..." : "Confirmar Eliminación"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}
