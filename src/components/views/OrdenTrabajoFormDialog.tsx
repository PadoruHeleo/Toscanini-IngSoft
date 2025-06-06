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
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";

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

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  cliente_nombre?: string;
}

interface OrdenTrabajoFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onOrdenAdded: () => void;
  orden?: OrdenTrabajo;
  isEditing?: boolean;
}

interface FormData {
  orden_codigo: string;
  orden_desc: string;
  prioridad: string;
  estado: string;
  has_garantia: boolean;
  equipo_id: string;
  pre_informe: string;
}

interface FormErrors {
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  equipo_id?: string;
  pre_informe?: string;
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
  const [loading, setLoading] = useState(false);
  const [equipos, setEquipos] = useState<Equipo[]>([]);
  const [loadingEquipos, setLoadingEquipos] = useState(false);
  const [formData, setFormData] = useState<FormData>({
    orden_codigo: "",
    orden_desc: "",
    prioridad: "media",
    estado: "recibido",
    has_garantia: false,
    equipo_id: "",
    pre_informe: "",
  });

  const [errors, setErrors] = useState<FormErrors>({}); // Función para generar la descripción automáticamente
  const generateDescription = (equipoId: string, preInforme: string) => {
    if (!equipoId || !preInforme.trim() || equipos.length === 0) return "";

    const equipo = equipos.find((e) => e.equipo_id.toString() === equipoId);
    if (!equipo) return "";

    const marca = equipo.equipo_marca || "Marca desconocida";
    const modelo = equipo.equipo_modelo || "Modelo desconocido";

    return `El equipo ${marca} ${modelo} presenta ${preInforme.trim()}`;
  }; // Actualizar descripción automáticamente cuando cambie el equipo o pre-informe
  useEffect(() => {
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
        setFormData((prev) => ({ ...prev, orden_desc: newDescription }));
      }
    }
  }, [formData.equipo_id, formData.pre_informe, equipos]);
  // Cargar equipos al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadEquipos();
    }
  }, [open]); // Inicializar formulario cuando se pasa una orden para editar
  useEffect(() => {
    if (isEditing && orden && open) {
      setFormData({
        orden_codigo: orden.orden_codigo || "",
        orden_desc: orden.orden_desc || "",
        prioridad: orden.prioridad || "media",
        estado: orden.estado || "recibido",
        has_garantia: orden.has_garantia || false,
        equipo_id: orden.equipo_id?.toString() || "",
        pre_informe: orden.pre_informe || "",
      });
    } else if (!isEditing && open) {
      // Resetear formulario para crear nueva orden
      setFormData({
        orden_codigo: "",
        orden_desc: "",
        prioridad: "media",
        estado: "recibido",
        has_garantia: false,
        equipo_id: "",
        pre_informe: "",
      });
    }
    setErrors({});
  }, [isEditing, orden, open]);
  // Regenerar descripción para órdenes existentes una vez que se cargan los equipos
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
        setFormData((prev) => ({ ...prev, orden_desc: newDescription }));
      }
    }
  }, [equipos.length, isEditing]);

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
  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.orden_codigo.trim()) {
      newErrors.orden_codigo = "El código es requerido";
    }

    // La descripción se genera automáticamente, pero verificamos que exista
    if (!formData.orden_desc.trim()) {
      newErrors.orden_desc =
        "Seleccione un equipo y complete el pre-informe para generar la descripción";
    }

    if (!formData.prioridad) {
      newErrors.prioridad = "La prioridad es requerida";
    }

    if (!formData.estado) {
      newErrors.estado = "El estado es requerido";
    }

    if (!formData.equipo_id) {
      newErrors.equipo_id = "Debe seleccionar un equipo";
    }

    if (!formData.pre_informe.trim()) {
      newErrors.pre_informe = "El pre-informe es requerido";
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
            newData.orden_desc = newDescription;
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

    if (!validateForm()) {
      return;
    }

    try {
      setLoading(true);

      if (isEditing && orden) {
        // Actualizar orden existente
        const updateData = {
          orden_codigo:
            formData.orden_codigo !== orden.orden_codigo
              ? formData.orden_codigo
              : undefined,
          orden_desc:
            formData.orden_desc !== orden.orden_desc
              ? formData.orden_desc
              : undefined,
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
            `La orden ${formData.orden_codigo} ha sido actualizada exitosamente.`
          );
          onOrdenAdded();
        } else {
          showError("Error", "No se pudo actualizar la orden de trabajo.");
        }
      } else {
        // Crear nueva orden
        const createData = {
          orden_codigo: formData.orden_codigo,
          orden_desc: formData.orden_desc,
          prioridad: formData.prioridad,
          estado: formData.estado,
          has_garantia: formData.has_garantia,
          equipo_id: parseInt(formData.equipo_id),
          created_by: user.usuario_id,
          pre_informe: formData.pre_informe,
          cotizacion_id: null,
          informe_id: null,
        };
        const result = await invoke<number>("create_orden_trabajo", {
          request: createData,
        });

        if (result) {
          success(
            "Orden creada",
            `La orden ${formData.orden_codigo} ha sido creada exitosamente.`
          );
          onOrdenAdded();
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
      <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
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

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            {/* Código de la orden */}
            <div className="space-y-2">
              <Label htmlFor="orden_codigo">Código de Orden *</Label>
              <Input
                id="orden_codigo"
                type="text"
                value={formData.orden_codigo}
                onChange={(e) =>
                  handleInputChange("orden_codigo", e.target.value)
                }
                placeholder="Ej: OT-2024-001"
                className={errors.orden_codigo ? "border-red-500" : ""}
              />
              {errors.orden_codigo && (
                <p className="text-sm text-red-500">{errors.orden_codigo}</p>
              )}
            </div>

            {/* Equipo */}
            <div className="space-y-2">
              <Label htmlFor="equipo_id">Equipo *</Label>
              <Select
                value={formData.equipo_id}
                onValueChange={(value) => handleInputChange("equipo_id", value)}
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
                <p className="text-sm text-red-500">{errors.equipo_id}</p>
              )}
            </div>
          </div>{" "}
          {/* Descripción */}
          <div className="space-y-2">
            <Label htmlFor="orden_desc">
              Descripción *
              <span className="text-sm text-gray-500 font-normal ml-1">
                (Se genera automáticamente)
              </span>
            </Label>
            <Textarea
              id="orden_desc"
              value={formData.orden_desc}
              readOnly
              placeholder="La descripción se generará automáticamente al seleccionar un equipo y escribir el pre-informe"
              className={`bg-gray-50 ${
                errors.orden_desc ? "border-red-500" : ""
              }`}
              rows={3}
            />
            {errors.orden_desc && (
              <p className="text-sm text-red-500">{errors.orden_desc}</p>
            )}
          </div>
          <div className="grid grid-cols-2 gap-4">
            {/* Prioridad */}
            <div className="space-y-2">
              <Label htmlFor="prioridad">Prioridad *</Label>
              <Select
                value={formData.prioridad}
                onValueChange={(value) => handleInputChange("prioridad", value)}
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
              <Label htmlFor="estado">Estado *</Label>
              <Select
                value={formData.estado}
                onValueChange={(value) => handleInputChange("estado", value)}
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
          {/* Garantía */}
          <div className="flex items-center space-x-2">
            <Checkbox
              id="has_garantia"
              checked={formData.has_garantia}
              onCheckedChange={(checked) =>
                handleInputChange("has_garantia", !!checked)
              }
            />
            <Label htmlFor="has_garantia">Equipo tiene garantía</Label>
          </div>
          {/* Pre-informe */}
          <div className="space-y-2">
            <Label htmlFor="pre_informe">Pre-informe *</Label>
            <Textarea
              id="pre_informe"
              value={formData.pre_informe}
              onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) =>
                handleInputChange("pre_informe", e.target.value)
              }
              placeholder="Diagnóstico inicial del equipo"
              className={errors.pre_informe ? "border-red-500" : ""}
              rows={4}
            />
            {errors.pre_informe && (
              <p className="text-sm text-red-500">{errors.pre_informe}</p>
            )}
          </div>
          {Object.keys(errors).length > 0 && (
            <div className="text-sm text-red-500 bg-red-50 p-3 rounded-md">
              Por favor, corrija los errores antes de continuar.
            </div>
          )}
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button type="submit" disabled={loading}>
              {loading ? "Guardando..." : isEditing ? "Actualizar" : "Crear"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
