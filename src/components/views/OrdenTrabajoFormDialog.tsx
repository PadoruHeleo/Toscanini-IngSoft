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
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";
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
}

interface Cliente {
  cliente_id: number;
  cliente_nombre: string;
  cliente_correo?: string;
}

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_precio?: number;
  equipo_ubicacion?: string;
  cliente_id?: number;
  cliente_nombre?: string;
  created_by?: number;
  created_at?: string;
}

interface CreateEquipoRequest {
  numero_serie: string;
  equipo_marca: string;
  equipo_modelo: string;
  equipo_tipo: string;
  equipo_precio?: number;
  equipo_ubicacion?: string;
  cliente_id: number;
  created_by: number;
}

interface CreateClienteRequest {
  cliente_rut: string;
  cliente_nombre: string;
  cliente_correo: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by: number;
}

interface OrdenTrabajoFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onOrdenAdded: () => void;
  orden?: OrdenTrabajo;
  isEditing?: boolean;
}

interface FormData {
  prioridad: string;
  estado: string;
  has_garantia: boolean;
  equipo_id: string;
  pre_informe: string;
}

interface EquipoFormData {
  numero_serie: string;
  equipo_marca: string;
  equipo_modelo: string;
  equipo_tipo: string;
  equipo_precio: number;
  equipo_ubicacion: string;
  cliente_id: number | undefined;
}

interface FormErrors {
  prioridad?: string;
  estado?: string;
  equipo_id?: string;
  pre_informe?: string;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  cliente_id?: string;
}

const prioridadOptions = [
  { value: "baja", label: "Baja" },
  { value: "media", label: "Media" },
  { value: "alta", label: "Alta" },
];

const estadoOptions = [
  { value: "recibido", label: "Recibido" },
  { value: "cotizacion_enviada", label: "Cotización Enviada" },
  { value: "aprobacion_pendiente", label: "Aprobación Pendiente" },
  { value: "en_reparacion", label: "En Reparación" },
  { value: "espera_de_retiro", label: "Espera de Retiro" },
  { value: "entregado", label: "Entregado" },
  { value: "abandonado", label: "Abandonado" },
  { value: "equipo_no_reparable", label: "Equipo No Reparable" },
  { value: "cotizacion_rechazada", label: "Cotización Rechazada" },
];

export default function OrdenTrabajoFormDialog({
  open,
  onOpenChange,
  onOrdenAdded,
  orden,
  isEditing = false,
}: OrdenTrabajoFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const { isRecepcion } = useOrdenTrabajoPermissions();
  const [loading, setLoading] = useState(false);
  const [equipos, setEquipos] = useState<Equipo[]>([]);
  const [loadingEquipos, setLoadingEquipos] = useState(false);
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  // Check if the work order is locked (has associated quote or report)
  const isOrderLocked = Boolean(
    isEditing && orden && (orden.cotizacion_id || orden.informe_id)
  );
  const [formData, setFormData] = useState<FormData>({
    prioridad: "media",
    estado: "recibido",
    has_garantia: false,
    equipo_id: "",
    pre_informe: "",
  });

  // Estados para manejo de equipos nuevos
  const [tipoEquipo, setTipoEquipo] = useState<"existente" | "nuevo">("nuevo");
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [marcas, setMarcas] = useState<string[]>([]);
  const [modelos, setModelos] = useState<string[]>([]);
  const [ubicaciones, setUbicaciones] = useState<string[]>([]);

  // Estados para crear nuevos valores
  const [showNewMarcaInput, setShowNewMarcaInput] = useState(false);
  const [showNewModeloInput, setShowNewModeloInput] = useState(false);
  const [showNewUbicacionInput, setShowNewUbicacionInput] = useState(false);
  const [newMarcaValue, setNewMarcaValue] = useState("");
  const [newModeloValue, setNewModeloValue] = useState("");
  const [newUbicacionValue, setNewUbicacionValue] = useState("");

  // Estados para crear nuevo cliente
  const [showNewClienteDialog, setShowNewClienteDialog] = useState(false);
  const [newClienteData, setNewClienteData] = useState<{
    cliente_rut: string;
    cliente_nombre: string;
    cliente_correo: string;
    cliente_telefono: string;
    cliente_direccion: string;
  }>({
    cliente_rut: "",
    cliente_nombre: "",
    cliente_correo: "",
    cliente_telefono: "",
    cliente_direccion: "",
  });
  const [newClienteErrors, setNewClienteErrors] = useState<
    Record<string, string>
  >({});

  // Estados para confirmaciones
  const [showConfirmMarcaDialog, setShowConfirmMarcaDialog] = useState(false);
  const [showConfirmModeloDialog, setShowConfirmModeloDialog] = useState(false);
  const [showConfirmUbicacionDialog, setShowConfirmUbicacionDialog] =
    useState(false);
  const [showConfirmClienteDialog, setShowConfirmClienteDialog] =
    useState(false);

  // Datos del equipo nuevo
  const [equipoFormData, setEquipoFormData] = useState<EquipoFormData>({
    numero_serie: "",
    equipo_marca: "",
    equipo_modelo: "",
    equipo_tipo: "",
    equipo_precio: 0,
    equipo_ubicacion: "",
    cliente_id: undefined,
  });

  // Función para generar la descripción automáticamente
  const generateDescription = (equipoId: string, preInforme: string) => {
    if (!equipoId || !preInforme.trim() || equipos.length === 0) return "";

    const equipo = equipos.find((e) => e.equipo_id.toString() === equipoId);
    if (!equipo) return "";

    const marca = equipo.equipo_marca || "Marca desconocida";
    const modelo = equipo.equipo_modelo || "Modelo desconocido";

    return `El equipo ${marca} ${modelo} presenta ${preInforme.trim()}`;
  };

  // Función para generar descripción con equipo nuevo
  const generateDescriptionForNewEquipo = (preInforme: string) => {
    if (
      !preInforme.trim() ||
      !equipoFormData.equipo_marca ||
      !equipoFormData.equipo_modelo
    )
      return "";

    const marca = equipoFormData.equipo_marca || "Marca desconocida";
    const modelo = equipoFormData.equipo_modelo || "Modelo desconocido";

    return `El equipo ${marca} ${modelo} presenta ${preInforme.trim()}`;
  };

  const [generatedDescription, setGeneratedDescription] = useState<string>("");
  const [errors, setErrors] = useState<FormErrors>({});

  // Actualizar descripción automáticamente cuando cambie el equipo o pre-informe
  useEffect(() => {
    if (tipoEquipo === "existente") {
      if (
        formData.equipo_id &&
        formData.pre_informe.trim() &&
        equipos.length > 0
      ) {
        const newDescription = generateDescription(
          formData.equipo_id,
          formData.pre_informe
        );
        if (newDescription) {
          setGeneratedDescription(newDescription);
        }
      }
    } else {
      if (
        formData.pre_informe.trim() &&
        equipoFormData.equipo_marca &&
        equipoFormData.equipo_modelo
      ) {
        const newDescription = generateDescriptionForNewEquipo(
          formData.pre_informe
        );
        if (newDescription) {
          setGeneratedDescription(newDescription);
        }
      }
    }
  }, [
    formData.equipo_id,
    formData.pre_informe,
    equipos,
    tipoEquipo,
    equipoFormData.equipo_marca,
    equipoFormData.equipo_modelo,
  ]);
  // Cargar equipos al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadEquipos();
      loadClientes();
      loadMarcas();
      loadUbicaciones();
    }
  }, [open]); // Inicializar formulario cuando se pasa una orden para editar
  useEffect(() => {
    if (isEditing && orden && open) {
      setFormData({
        prioridad: orden.prioridad || "media",
        estado: orden.estado || "recibido",
        has_garantia: orden.has_garantia || false,
        equipo_id: orden.equipo_id?.toString() || "",
        pre_informe: orden.pre_informe || "",
      });
    } else if (!isEditing && open) {
      // Resetear formulario para crear nueva orden
      setFormData({
        prioridad: "media",
        estado: "recibido",
        has_garantia: false,
        equipo_id: "",
        pre_informe: "",
      });
      // Resetear datos del equipo nuevo
      setEquipoFormData({
        numero_serie: "",
        equipo_marca: "",
        equipo_modelo: "",
        equipo_tipo: "",
        equipo_precio: 0,
        equipo_ubicacion: "",
        cliente_id: undefined,
      });
      setTipoEquipo("nuevo");
    }
    setErrors({});
    setGeneratedDescription("");
  }, [isEditing, orden, open]); // Regenerar descripción para órdenes existentes una vez que se cargan los equipos
  useEffect(() => {
    if (
      isEditing &&
      formData.equipo_id &&
      formData.pre_informe &&
      equipos.length > 0
    ) {
      const newDescription = generateDescription(
        formData.equipo_id,
        formData.pre_informe
      );
      if (newDescription) {
        setGeneratedDescription(newDescription);
      }
    }
  }, [equipos.length, isEditing]);
  // Asegurar que el equipo se seleccione correctamente al editar después de cargar equipos
  useEffect(() => {
    if (isEditing && orden && equipos.length > 0 && orden.equipo_id) {
      // Verificar si el equipo existe en la lista cargada
      const equipoExists = equipos.find((e) => e.equipo_id === orden.equipo_id);
      if (equipoExists) {
        // Solo actualizar si el formData no tiene el equipo correcto establecido
        setFormData((prev) => {
          if (prev.equipo_id !== orden.equipo_id!.toString()) {
            console.log(
              `Estableciendo equipo_id al editar: ${orden.equipo_id}`
            );
            return {
              ...prev,
              equipo_id: orden.equipo_id!.toString(),
            };
          }
          return prev;
        });
      }
    }
  }, [equipos.length, isEditing, orden?.equipo_id]);

  const loadEquipos = async () => {
    try {
      setLoadingEquipos(true);
      const equiposData = await invoke<Equipo[]>("get_equipos_with_cliente");
      setEquipos(equiposData);
    } catch (error) {
      console.error("Error cargando equipos:", error);
      showError("Error", "No se pudieron cargar los equipos.");
    } finally {
      setLoadingEquipos(false);
    }
  };

  // Funciones para cargar datos de equipos nuevos
  const loadClientes = async () => {
    try {
      const clientesData = await invoke<Cliente[]>("get_clientes");
      setClientes(clientesData);
    } catch (error) {
      console.error("Error cargando clientes:", error);
    }
  };

  const loadMarcas = async () => {
    try {
      const marcasData = await invoke<string[]>("get_equipos_marcas");
      setMarcas(marcasData);
    } catch (error) {
      console.error("Error cargando marcas:", error);
    }
  };

  const loadUbicaciones = async () => {
    try {
      const ubicacionesData = await invoke<string[]>("get_equipos_ubicaciones");
      setUbicaciones(ubicacionesData);
    } catch (error) {
      console.error("Error cargando ubicaciones:", error);
    }
  };

  const loadModelosByMarca = async (marca: string) => {
    try {
      const modelosData = await invoke<string[]>(
        "get_equipos_modelos_by_marca",
        { marca }
      );
      setModelos(modelosData);
    } catch (error) {
      console.error("Error cargando modelos:", error);
      setModelos([]);
    }
  };

  // Funciones para manejo de equipos nuevos
  const validateClienteForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!newClienteData.cliente_rut.trim()) {
      errors.cliente_rut = "El RUT es obligatorio";
    }
    if (!newClienteData.cliente_nombre.trim()) {
      errors.cliente_nombre = "El nombre es obligatorio";
    }
    if (!newClienteData.cliente_correo.trim()) {
      errors.cliente_correo = "El correo es obligatorio";
    } else if (
      !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(newClienteData.cliente_correo)
    ) {
      errors.cliente_correo = "El formato del correo no es válido";
    }

    setNewClienteErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleCreateCliente = async () => {
    if (!validateClienteForm()) {
      return;
    }

    try {
      setLoading(true);
      const clienteRequest: CreateClienteRequest = {
        ...newClienteData,
        created_by: user?.usuario_id || 0,
      };
      const nuevoCliente = await invoke<Cliente>("create_cliente", {
        request: clienteRequest,
      });

      setClientes((prev) => [...prev, nuevoCliente]);
      setEquipoFormData((prev) => ({
        ...prev,
        cliente_id: nuevoCliente.cliente_id,
      }));

      setShowNewClienteDialog(false);
      setNewClienteData({
        cliente_rut: "",
        cliente_nombre: "",
        cliente_correo: "",
        cliente_telefono: "",
        cliente_direccion: "",
      });
      setNewClienteErrors({});

      success(
        "¡Cliente creado exitosamente!",
        `${nuevoCliente.cliente_nombre} ha sido registrado.`
      );
    } catch (error) {
      console.error("Error creando cliente:", error);
      showError(
        "Error al crear cliente",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    } finally {
      setLoading(false);
    }
  };

  // Funciones de confirmación para valores nuevos
  const handleConfirmMarca = () => {
    if (newMarcaValue.trim()) {
      const nuevaMarca = newMarcaValue.trim();
      if (!marcas.includes(nuevaMarca)) {
        setMarcas((prev) => [...prev, nuevaMarca]);
      }
      handleEquipoInputChange("equipo_marca", nuevaMarca);
      setShowNewMarcaInput(false);
      setNewMarcaValue("");
      setShowConfirmMarcaDialog(false);
    }
  };

  const handleConfirmModelo = () => {
    if (newModeloValue.trim()) {
      const nuevoModelo = newModeloValue.trim();
      if (!modelos.includes(nuevoModelo)) {
        setModelos((prev) => [...prev, nuevoModelo]);
      }
      handleEquipoInputChange("equipo_modelo", nuevoModelo);
      setShowNewModeloInput(false);
      setNewModeloValue("");
      setShowConfirmModeloDialog(false);
    }
  };

  const handleConfirmUbicacion = () => {
    if (newUbicacionValue.trim()) {
      const nuevaUbicacion = newUbicacionValue.trim();
      if (!ubicaciones.includes(nuevaUbicacion)) {
        setUbicaciones((prev) => [...prev, nuevaUbicacion]);
      }
      handleEquipoInputChange("equipo_ubicacion", nuevaUbicacion);
      setShowNewUbicacionInput(false);
      setNewUbicacionValue("");
      setShowConfirmUbicacionDialog(false);
    }
  };

  const handleConfirmCreateCliente = () => {
    setShowConfirmClienteDialog(false);
    setShowNewClienteDialog(true);
  };

  const handleEquipoInputChange = (field: keyof EquipoFormData, value: any) => {
    setEquipoFormData((prev) => ({ ...prev, [field]: value }));

    // Si cambia la marca, cargar modelos y limpiar el modelo actual
    if (field === "equipo_marca" && value) {
      loadModelosByMarca(value);
      setEquipoFormData((prev) => ({ ...prev, equipo_modelo: "" }));
      setShowNewModeloInput(false);
      setNewModeloValue("");
    }

    // Limpiar error del campo cuando el usuario empiece a escribir
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: "" }));
    }
  };
  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.prioridad) {
      newErrors.prioridad = "La prioridad es requerida";
    }

    if (!formData.estado) {
      newErrors.estado = "El estado es requerido";
    }

    if (!formData.pre_informe.trim()) {
      newErrors.pre_informe = "El pre-informe es requerido";
    }

    // Validar según el tipo de equipo
    if (tipoEquipo === "existente") {
      if (!formData.equipo_id) {
        newErrors.equipo_id = "Debe seleccionar un equipo";
      }
    } else {
      // Validar datos del equipo nuevo
      if (!equipoFormData.numero_serie?.trim()) {
        newErrors.numero_serie = "El número de serie es obligatorio";
      }
      if (!equipoFormData.equipo_marca?.trim()) {
        newErrors.equipo_marca = "La marca es obligatoria";
      }
      if (!equipoFormData.equipo_modelo?.trim()) {
        newErrors.equipo_modelo = "El modelo es obligatorio";
      }
      if (!equipoFormData.equipo_tipo) {
        newErrors.equipo_tipo = "El tipo es obligatorio";
      }
      if (!equipoFormData.cliente_id) {
        newErrors.cliente_id = "Debe seleccionar un cliente";
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };
  const handleInputChange = (
    field: keyof FormData,
    value: string | boolean
  ) => {
    setFormData((prev) => {
      const newData = { ...prev, [field]: value };

      // Si se cambia el equipo o pre-informe, regenerar descripción inmediatamente
      if (
        (field === "equipo_id" || field === "pre_informe") &&
        equipos.length > 0
      ) {
        const equipoId =
          field === "equipo_id" ? (value as string) : prev.equipo_id;
        const preInforme =
          field === "pre_informe" ? (value as string) : prev.pre_informe;

        if (
          equipoId &&
          preInforme &&
          typeof preInforme === "string" &&
          preInforme.trim()
        ) {
          const newDescription = generateDescription(equipoId, preInforme);
          if (newDescription) {
            setGeneratedDescription(newDescription);
          }
        }
      }

      return newData;
    });

    // Limpiar error del campo cuando el usuario empiece a escribir
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!user) {
      showError("Error de autenticación", "Usuario no autenticado");
      return;
    }

    // Check if the order is locked for editing
    if (isOrderLocked) {
      const lockReason = orden?.cotizacion_id ? "cotización" : "informe";
      showError(
        "Edición no permitida",
        `No se puede editar esta orden de trabajo porque ya tiene una ${lockReason} asociada.`
      );
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
      if (isEditing && orden) {
        // Actualizar orden existente
        const updateData = {
          prioridad:
            formData.prioridad !== orden.prioridad
              ? formData.prioridad
              : undefined,
          estado:
            formData.estado !== orden.estado ? formData.estado : undefined,
          has_garantia:
            formData.has_garantia !== orden.has_garantia
              ? formData.has_garantia
              : undefined,
          equipo_id:
            parseInt(formData.equipo_id) !== orden.equipo_id
              ? parseInt(formData.equipo_id)
              : undefined,
          pre_informe:
            formData.pre_informe !== orden.pre_informe
              ? formData.pre_informe
              : undefined,
        };
        const result = await invoke<boolean>("update_orden_trabajo", {
          ordenId: orden.orden_id,
          request: updateData,
          updatedBy: user.usuario_id,
        });

        if (result) {
          success(
            "Orden actualizada",
            `La orden ha sido actualizada exitosamente.`
          );
          onOrdenAdded();
        } else {
          showError("Error", "No se pudo actualizar la orden de trabajo.");
        }
      } else {
        // Crear nueva orden
        let equipoId: number;

        if (tipoEquipo === "nuevo") {
          // Primero crear el equipo nuevo
          const equipoData: CreateEquipoRequest = {
            numero_serie: equipoFormData.numero_serie!,
            equipo_marca: equipoFormData.equipo_marca!,
            equipo_modelo: equipoFormData.equipo_modelo!,
            equipo_tipo: equipoFormData.equipo_tipo!,
            equipo_precio: equipoFormData.equipo_precio || 0,
            equipo_ubicacion: equipoFormData.equipo_ubicacion || undefined,
            cliente_id: equipoFormData.cliente_id!,
            created_by: user.usuario_id,
          };

          const nuevoEquipo = await invoke<Equipo>("create_equipo", {
            request: equipoData,
          });

          equipoId = nuevoEquipo.equipo_id;
        } else {
          // Usar equipo existente
          equipoId = parseInt(formData.equipo_id);
        }

        const createData = {
          orden_desc: generatedDescription,
          prioridad: formData.prioridad,
          estado: formData.estado,
          has_garantia: formData.has_garantia,
          equipo_id: equipoId,
          created_by: user.usuario_id,
          pre_informe: formData.pre_informe,
          cotizacion_id: null,
          informe_id: null,
        };
        const result = await invoke<OrdenTrabajo>("create_orden_trabajo", {
          request: createData,
        });

        if (result) {
          success("Orden creada", `La orden ha sido creada exitosamente.`);
          onOrdenAdded();

          // Enviar comando al cliente después de crear la orden
          await invoke("send_orden_trabajo_cliente", {
            ordenId: result.orden_id, // id de la orden creada
            sentBy: user.usuario_id,
          });
        } else {
          showError("Error", "No se pudo crear la orden de trabajo.");
        }
      }
    } catch (error) {
      console.error("Error al guardar orden:", error);
      showError(
        `Error al ${isEditing ? "actualizar" : "crear"} orden`,
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    } finally {
      setLoading(false);
      setShowConfirmationDialog(false);
    }
  };

  const getEquipoDisplayName = (equipo: Equipo) => {
    const parts = [];
    if (equipo.numero_serie) parts.push(`S/N: ${equipo.numero_serie}`);
    if (equipo.equipo_marca) parts.push(equipo.equipo_marca);
    if (equipo.equipo_modelo) parts.push(equipo.equipo_modelo);
    if (equipo.cliente_nombre) parts.push(`(${equipo.cliente_nombre})`);

    return parts.length > 0 ? parts.join(" ") : `Equipo ${equipo.equipo_id}`;
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="!max-w-4xl max-h-[90vh] overflow-y-auto">
        {" "}
        <DialogHeader>
          <DialogTitle>
            {isEditing
              ? "Editar Orden de Trabajo"
              : "Crear Nueva Orden de Trabajo"}
          </DialogTitle>
          <DialogDescription>
            {isEditing
              ? "Modifica los datos de la orden de trabajo"
              : "Completa los datos para crear una nueva orden de trabajo"}
          </DialogDescription>
        </DialogHeader>
        {/* Warning message for locked orders */}
        {isOrderLocked && (
          <div className="bg-yellow-50 border border-yellow-200 rounded-md p-4 mb-4">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <svg
                  className="h-5 w-5 text-yellow-400"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-yellow-800">
                  Edición restringida
                </h3>
                <div className="mt-1 text-sm text-yellow-700">
                  Esta orden de trabajo no se puede editar porque ya tiene una{" "}
                  {orden?.cotizacion_id ? "cotización" : "informe"} asociada.
                  Los campos se muestran en modo de solo lectura.
                </div>
              </div>
            </div>
          </div>
        )}
        {/* Info message for reception users */}
        {isRecepcion && isEditing && (
          <div className="bg-blue-50 border border-blue-200 rounded-md p-4 mb-4">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <svg
                  className="h-5 w-5 text-blue-400"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-blue-800">
                  Permisos de Recepción
                </h3>
                <div className="mt-1 text-sm text-blue-700">
                  Como usuario de recepción, puede modificar todos los campos de
                  la orden. No puede eliminar órdenes de trabajo.
                </div>
              </div>
            </div>
          </div>
        )}
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            {/* Código de la orden (auto-generado) */}
            <div className="space-y-2">
              <Label htmlFor="orden_codigo">Código de Orden</Label>
              <Input
                id="orden_codigo"
                type="text"
                value={
                  isEditing && orden?.orden_codigo
                    ? orden.orden_codigo
                    : "(Se generará automáticamente)"
                }
                readOnly
                className="bg-gray-50 font-semibold"
              />
            </div>

            {/* Tipo de Equipo - Solo en modo creación */}
            {!isEditing && (
              <div className="col-span-2 space-y-2">
                <Label>Tipo de Equipo *</Label>
                <Tabs
                  value={tipoEquipo}
                  onValueChange={(value) =>
                    setTipoEquipo(value as "existente" | "nuevo")
                  }
                  className="w-full"
                >
                  <TabsList className="grid w-full grid-cols-2">
                    <TabsTrigger value="nuevo">Equipo Nuevo</TabsTrigger>
                    <TabsTrigger value="existente">
                      Equipo Existente
                    </TabsTrigger>
                  </TabsList>

                  <TabsContent value="nuevo" className="space-y-4">
                    {/* Campos del equipo nuevo - Layout simplificado */}
                    <div className="space-y-4">
                      <div className="grid grid-cols-2 gap-4">
                        <div className="space-y-2">
                          <Label htmlFor="numero_serie">
                            Número de Serie *
                          </Label>
                          <Input
                            id="numero_serie"
                            value={equipoFormData.numero_serie}
                            onChange={(e) =>
                              handleEquipoInputChange(
                                "numero_serie",
                                e.target.value
                              )
                            }
                            placeholder="Ingrese el número de serie"
                            className={
                              errors.numero_serie ? "border-red-500" : ""
                            }
                          />
                          {errors.numero_serie && (
                            <p className="text-sm text-red-600">
                              {errors.numero_serie}
                            </p>
                          )}
                        </div>

                        <div className="space-y-2">
                          <Label htmlFor="equipo_tipo">Tipo *</Label>
                          <Select
                            value={equipoFormData.equipo_tipo}
                            onValueChange={(value) =>
                              handleEquipoInputChange("equipo_tipo", value)
                            }
                          >
                            <SelectTrigger
                              className={
                                errors.equipo_tipo ? "border-red-500" : ""
                              }
                            >
                              <SelectValue placeholder="Seleccionar tipo" />
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="telefono">Teléfono</SelectItem>
                              <SelectItem value="tablet">Tablet</SelectItem>
                              <SelectItem value="laptop">Laptop</SelectItem>
                              <SelectItem value="computador">
                                Computador
                              </SelectItem>
                              <SelectItem value="impresora">
                                Impresora
                              </SelectItem>
                              <SelectItem value="monitor">Monitor</SelectItem>
                              <SelectItem value="otro">Otro</SelectItem>
                            </SelectContent>
                          </Select>
                          {errors.equipo_tipo && (
                            <p className="text-sm text-red-600">
                              {errors.equipo_tipo}
                            </p>
                          )}
                        </div>
                      </div>

                      <div className="grid grid-cols-2 gap-4">
                        <div className="space-y-2">
                          <Label htmlFor="equipo_marca">Marca *</Label>
                          <Select
                            value={equipoFormData.equipo_marca}
                            onValueChange={(value) => {
                              if (value === "nueva_marca") {
                                setShowNewMarcaInput(true);
                              } else {
                                handleEquipoInputChange("equipo_marca", value);
                              }
                            }}
                          >
                            <SelectTrigger
                              className={
                                errors.equipo_marca ? "border-red-500" : ""
                              }
                            >
                              <SelectValue placeholder="Seleccionar marca" />
                            </SelectTrigger>
                            <SelectContent>
                              {marcas.map((marca) => (
                                <SelectItem key={marca} value={marca}>
                                  {marca}
                                </SelectItem>
                              ))}
                              <SelectItem value="nueva_marca">
                                + Agregar nueva marca
                              </SelectItem>
                            </SelectContent>
                          </Select>
                          {showNewMarcaInput && (
                            <div className="flex gap-2">
                              <Input
                                placeholder="Nueva marca"
                                value={newMarcaValue}
                                onChange={(e) =>
                                  setNewMarcaValue(e.target.value)
                                }
                              />
                              <Button
                                type="button"
                                onClick={() => setShowConfirmMarcaDialog(true)}
                                size="sm"
                              >
                                ✓
                              </Button>
                              <Button
                                type="button"
                                variant="outline"
                                onClick={() => {
                                  setShowNewMarcaInput(false);
                                  setNewMarcaValue("");
                                }}
                                size="sm"
                              >
                                ✕
                              </Button>
                            </div>
                          )}
                          {errors.equipo_marca && (
                            <p className="text-sm text-red-600">
                              {errors.equipo_marca}
                            </p>
                          )}
                        </div>

                        <div className="space-y-2">
                          <Label htmlFor="equipo_modelo">Modelo *</Label>
                          <Select
                            value={equipoFormData.equipo_modelo}
                            onValueChange={(value) => {
                              if (value === "nuevo_modelo") {
                                setShowNewModeloInput(true);
                              } else {
                                handleEquipoInputChange("equipo_modelo", value);
                              }
                            }}
                            disabled={!equipoFormData.equipo_marca}
                          >
                            <SelectTrigger
                              className={
                                errors.equipo_modelo ? "border-red-500" : ""
                              }
                            >
                              <SelectValue
                                placeholder={
                                  !equipoFormData.equipo_marca
                                    ? "Primero seleccione una marca"
                                    : "Seleccionar modelo"
                                }
                              />
                            </SelectTrigger>
                            <SelectContent>
                              {modelos.map((modelo) => (
                                <SelectItem key={modelo} value={modelo}>
                                  {modelo}
                                </SelectItem>
                              ))}
                              <SelectItem value="nuevo_modelo">
                                + Agregar nuevo modelo
                              </SelectItem>
                            </SelectContent>
                          </Select>
                          {showNewModeloInput && (
                            <div className="flex gap-2">
                              <Input
                                placeholder="Nuevo modelo"
                                value={newModeloValue}
                                onChange={(e) =>
                                  setNewModeloValue(e.target.value)
                                }
                              />
                              <Button
                                type="button"
                                onClick={() => setShowConfirmModeloDialog(true)}
                                size="sm"
                              >
                                ✓
                              </Button>
                              <Button
                                type="button"
                                variant="outline"
                                onClick={() => {
                                  setShowNewModeloInput(false);
                                  setNewModeloValue("");
                                }}
                                size="sm"
                              >
                                ✕
                              </Button>
                            </div>
                          )}
                          {errors.equipo_modelo && (
                            <p className="text-sm text-red-600">
                              {errors.equipo_modelo}
                            </p>
                          )}
                        </div>
                      </div>

                      <div className="grid grid-cols-2 gap-4">
                        <div className="space-y-2">
                          <Label htmlFor="cliente_id">Cliente *</Label>
                          <Select
                            value={equipoFormData.cliente_id?.toString() || ""}
                            onValueChange={(value) => {
                              if (value === "nuevo_cliente") {
                                setShowConfirmClienteDialog(true);
                              } else {
                                handleEquipoInputChange(
                                  "cliente_id",
                                  parseInt(value)
                                );
                              }
                            }}
                          >
                            <SelectTrigger
                              className={
                                errors.cliente_id ? "border-red-500" : ""
                              }
                            >
                              <SelectValue placeholder="Seleccionar cliente" />
                            </SelectTrigger>
                            <SelectContent>
                              {clientes.map((cliente) => (
                                <SelectItem
                                  key={cliente.cliente_id}
                                  value={cliente.cliente_id.toString()}
                                >
                                  {cliente.cliente_nombre}
                                </SelectItem>
                              ))}
                              <SelectItem value="nuevo_cliente">
                                + Crear nuevo cliente
                              </SelectItem>
                            </SelectContent>
                          </Select>
                          {errors.cliente_id && (
                            <p className="text-sm text-red-600">
                              {errors.cliente_id}
                            </p>
                          )}
                        </div>

                        <div className="space-y-2">
                          <Label htmlFor="equipo_ubicacion">Ubicación</Label>
                          <Select
                            value={equipoFormData.equipo_ubicacion || ""}
                            onValueChange={(value) => {
                              if (value === "nueva_ubicacion") {
                                setShowNewUbicacionInput(true);
                              } else {
                                handleEquipoInputChange(
                                  "equipo_ubicacion",
                                  value
                                );
                              }
                            }}
                          >
                            <SelectTrigger>
                              <SelectValue placeholder="Seleccionar ubicación" />
                            </SelectTrigger>
                            <SelectContent>
                              {ubicaciones.map((ubicacion) => (
                                <SelectItem key={ubicacion} value={ubicacion}>
                                  {ubicacion}
                                </SelectItem>
                              ))}
                              <SelectItem value="nueva_ubicacion">
                                + Agregar nueva ubicación
                              </SelectItem>
                            </SelectContent>
                          </Select>
                          {showNewUbicacionInput && (
                            <div className="flex gap-2">
                              <Input
                                placeholder="Nueva ubicación"
                                value={newUbicacionValue}
                                onChange={(e) =>
                                  setNewUbicacionValue(e.target.value)
                                }
                              />
                              <Button
                                type="button"
                                onClick={() =>
                                  setShowConfirmUbicacionDialog(true)
                                }
                                size="sm"
                              >
                                ✓
                              </Button>
                              <Button
                                type="button"
                                variant="outline"
                                onClick={() => {
                                  setShowNewUbicacionInput(false);
                                  setNewUbicacionValue("");
                                }}
                                size="sm"
                              >
                                ✕
                              </Button>
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                  </TabsContent>

                  <TabsContent value="existente" className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="equipo_id">Seleccionar Equipo *</Label>
                      <Select
                        value={formData.equipo_id}
                        onValueChange={(value) =>
                          handleInputChange("equipo_id", value)
                        }
                        disabled={isOrderLocked}
                      >
                        <SelectTrigger
                          className={errors.equipo_id ? "border-red-500" : ""}
                        >
                          <SelectValue
                            placeholder={
                              loadingEquipos
                                ? "Cargando equipos..."
                                : "Seleccionar equipo"
                            }
                          />
                        </SelectTrigger>
                        <SelectContent>
                          {equipos.map((equipo) => (
                            <SelectItem
                              key={equipo.equipo_id}
                              value={equipo.equipo_id.toString()}
                            >
                              {getEquipoDisplayName(equipo)}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      {errors.equipo_id && (
                        <p className="text-sm text-red-600">
                          {errors.equipo_id}
                        </p>
                      )}
                    </div>
                  </TabsContent>
                </Tabs>
              </div>
            )}

            {/* En modo edición, mostrar campo de equipo normal */}
            {isEditing && (
              <div className="space-y-2">
                <Label htmlFor="equipo_id">Equipo *</Label>
                <Select
                  value={formData.equipo_id}
                  onValueChange={(value) =>
                    handleInputChange("equipo_id", value)
                  }
                  disabled={isOrderLocked}
                >
                  <SelectTrigger
                    className={errors.equipo_id ? "border-red-500" : ""}
                  >
                    <SelectValue
                      placeholder={
                        loadingEquipos
                          ? "Cargando equipos..."
                          : "Seleccionar equipo"
                      }
                    />
                  </SelectTrigger>
                  <SelectContent>
                    {equipos.map((equipo) => (
                      <SelectItem
                        key={equipo.equipo_id}
                        value={equipo.equipo_id.toString()}
                      >
                        {getEquipoDisplayName(equipo)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {errors.equipo_id && (
                  <p className="text-sm text-red-600">{errors.equipo_id}</p>
                )}
              </div>
            )}
          </div>
          {/* Detalles del Equipo Seleccionado */}
          {formData.equipo_id && (
            <div className="bg-gray-50 p-4 rounded-lg border">
              <h3 className="text-sm font-medium text-gray-900 mb-3">
                Detalles del Equipo Seleccionado
              </h3>
              {(() => {
                const equipo = equipos.find(
                  (e) => e.equipo_id.toString() === formData.equipo_id
                );
                if (!equipo)
                  return (
                    <p className="text-sm text-gray-500">
                      Cargando detalles...
                    </p>
                  );

                return (
                  <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <span className="font-medium text-gray-600">
                        Cliente:
                      </span>
                      <p className="mt-1">
                        {equipo.cliente_nombre || "No especificado"}
                      </p>
                    </div>
                    <div>
                      <span className="font-medium text-gray-600">
                        Número de Serie:
                      </span>
                      <p className="mt-1">
                        {equipo.numero_serie || "No especificado"}
                      </p>
                    </div>
                    <div>
                      <span className="font-medium text-gray-600">Marca:</span>
                      <p className="mt-1">
                        {equipo.equipo_marca || "No especificado"}
                      </p>
                    </div>
                    <div>
                      <span className="font-medium text-gray-600">Modelo:</span>
                      <p className="mt-1">
                        {equipo.equipo_modelo || "No especificado"}
                      </p>
                    </div>
                    <div>
                      <span className="font-medium text-gray-600">Tipo:</span>
                      <p className="mt-1">
                        {equipo.equipo_tipo || "No especificado"}
                      </p>
                    </div>
                  </div>
                );
              })()}
            </div>
          )}
          {/* La descripción se genera automáticamente internamente y no se muestra al usuario */}
          <div className="grid grid-cols-2 gap-4">
            {/* Prioridad */}
            <div className="space-y-2">
              <Label htmlFor="prioridad">Prioridad *</Label>{" "}
              <Select
                value={formData.prioridad}
                onValueChange={(value) => handleInputChange("prioridad", value)}
                disabled={isOrderLocked}
              >
                <SelectTrigger
                  className={errors.prioridad ? "border-red-500" : ""}
                >
                  <SelectValue placeholder="Seleccionar prioridad" />
                </SelectTrigger>
                <SelectContent>
                  {prioridadOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {errors.prioridad && (
                <p className="text-sm text-red-500">{errors.prioridad}</p>
              )}
            </div>

            {/* Estado */}
            <div className="space-y-2">
              <Label htmlFor="estado">Estado *</Label>{" "}
              <Select
                value={formData.estado}
                onValueChange={(value) => handleInputChange("estado", value)}
                disabled={isOrderLocked}
              >
                <SelectTrigger
                  className={errors.estado ? "border-red-500" : ""}
                >
                  <SelectValue placeholder="Seleccionar estado" />
                </SelectTrigger>
                <SelectContent>
                  {estadoOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {errors.estado && (
                <p className="text-sm text-red-500">{errors.estado}</p>
              )}
            </div>
          </div>
          {/* Garantía */}{" "}
          <div className="flex items-center space-x-2">
            <Checkbox
              id="has_garantia"
              checked={formData.has_garantia}
              onCheckedChange={(checked) =>
                handleInputChange("has_garantia", !!checked)
              }
              disabled={isOrderLocked}
            />
            <Label htmlFor="has_garantia">Equipo tiene garantía</Label>
          </div>
          {/* Pre-informe */}
          <div className="space-y-2">
            <Label htmlFor="pre_informe">Pre-informe *</Label>{" "}
            <Textarea
              id="pre_informe"
              value={formData.pre_informe}
              onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) =>
                handleInputChange("pre_informe", e.target.value)
              }
              placeholder="Diagnóstico inicial del equipo"
              className={errors.pre_informe ? "border-red-500" : ""}
              rows={4}
              disabled={isOrderLocked}
            />
            {errors.pre_informe && (
              <p className="text-sm text-red-500">{errors.pre_informe}</p>
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
              {isOrderLocked ? "Cerrar" : "Cancelar"}
            </Button>
            {!isOrderLocked && (
              <Button type="submit" disabled={loading}>
                {loading ? "Guardando..." : isEditing ? "Actualizar" : "Crear"}
              </Button>
            )}
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
                : "Confirmar Creación de Orden"}
            </DialogTitle>
            <DialogDescription>
              {isEditing
                ? "¿Está seguro que desea actualizar esta orden de trabajo con los cambios realizados?"
                : "¿Está seguro que desea crear esta orden de trabajo con la siguiente información?"}
            </DialogDescription>
          </DialogHeader>{" "}
          <div className="space-y-2 text-sm">
            <div>
              <strong>Equipo:</strong>{" "}
              {equipos.find(
                (e) => e.equipo_id.toString() === formData.equipo_id
              )
                ? getEquipoDisplayName(
                    equipos.find(
                      (e) => e.equipo_id.toString() === formData.equipo_id
                    )!
                  )
                : "No seleccionado"}
            </div>
            <div>
              <strong>Prioridad:</strong>{" "}
              {prioridadOptions.find((p) => p.value === formData.prioridad)
                ?.label || formData.prioridad}
            </div>
            <div>
              <strong>Estado:</strong>{" "}
              {estadoOptions.find((e) => e.value === formData.estado)?.label ||
                formData.estado}
            </div>
            <div>
              <strong>Garantía:</strong> {formData.has_garantia ? "Sí" : "No"}
            </div>
            {formData.pre_informe && (
              <div>
                <strong>Pre-informe:</strong>{" "}
                {formData.pre_informe.length > 50
                  ? formData.pre_informe.substring(0, 50) + "..."
                  : formData.pre_informe}
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

      {/* Modal para agregar nuevo cliente */}
      <Dialog
        open={showNewClienteDialog}
        onOpenChange={setShowNewClienteDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Crear Nuevo Cliente</DialogTitle>
            <DialogDescription>
              Complete la información del nuevo cliente.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="cliente_rut">RUT *</Label>
              <Input
                id="cliente_rut"
                value={newClienteData.cliente_rut}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_rut: e.target.value,
                  }))
                }
                placeholder="12.345.678-9"
                className={newClienteErrors.cliente_rut ? "border-red-500" : ""}
              />
              {newClienteErrors.cliente_rut && (
                <p className="text-sm text-red-600">
                  {newClienteErrors.cliente_rut}
                </p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="cliente_nombre">Nombre *</Label>
              <Input
                id="cliente_nombre"
                value={newClienteData.cliente_nombre}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_nombre: e.target.value,
                  }))
                }
                placeholder="Nombre completo"
                className={
                  newClienteErrors.cliente_nombre ? "border-red-500" : ""
                }
              />
              {newClienteErrors.cliente_nombre && (
                <p className="text-sm text-red-600">
                  {newClienteErrors.cliente_nombre}
                </p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="cliente_correo">Correo Electrónico *</Label>
              <Input
                id="cliente_correo"
                type="email"
                value={newClienteData.cliente_correo}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_correo: e.target.value,
                  }))
                }
                placeholder="correo@ejemplo.com"
                className={
                  newClienteErrors.cliente_correo ? "border-red-500" : ""
                }
              />
              {newClienteErrors.cliente_correo && (
                <p className="text-sm text-red-600">
                  {newClienteErrors.cliente_correo}
                </p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="cliente_telefono">Teléfono</Label>
              <Input
                id="cliente_telefono"
                value={newClienteData.cliente_telefono}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_telefono: e.target.value,
                  }))
                }
                placeholder="+56 9 1234 5678"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="cliente_direccion">Dirección</Label>
              <Input
                id="cliente_direccion"
                value={newClienteData.cliente_direccion}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_direccion: e.target.value,
                  }))
                }
                placeholder="Dirección completa"
              />
            </div>
          </div>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowNewClienteDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleCreateCliente} disabled={loading}>
              {loading ? "Creando..." : "Crear Cliente"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación para crear marca */}
      <Dialog
        open={showConfirmMarcaDialog}
        onOpenChange={setShowConfirmMarcaDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Nueva Marca</DialogTitle>
          </DialogHeader>
          <p>¿Desea agregar la marca "{newMarcaValue}" a la lista?</p>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmMarcaDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmMarca}>Confirmar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación para crear modelo */}
      <Dialog
        open={showConfirmModeloDialog}
        onOpenChange={setShowConfirmModeloDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Nuevo Modelo</DialogTitle>
          </DialogHeader>
          <p>¿Desea agregar el modelo "{newModeloValue}" a la lista?</p>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmModeloDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmModelo}>Confirmar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación para crear ubicación */}
      <Dialog
        open={showConfirmUbicacionDialog}
        onOpenChange={setShowConfirmUbicacionDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Nueva Ubicación</DialogTitle>
          </DialogHeader>
          <p>¿Desea agregar la ubicación "{newUbicacionValue}" a la lista?</p>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmUbicacionDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmUbicacion}>Confirmar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal de confirmación para crear cliente */}
      <Dialog
        open={showConfirmClienteDialog}
        onOpenChange={setShowConfirmClienteDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Crear Nuevo Cliente</DialogTitle>
          </DialogHeader>
          <p>
            ¿Desea crear un nuevo cliente? Se abrirá un formulario para
            completar los datos.
          </p>
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmClienteDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmCreateCliente}>Continuar</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}
