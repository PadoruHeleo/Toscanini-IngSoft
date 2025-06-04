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
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useAuth } from "@/contexts/AuthContext";

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
  created_by?: number;
  created_at?: string;
}

interface EquipoFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onEquipoAdded: () => void;
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

interface CreateOrdenTrabajoRequest {
  orden_codigo: string;
  orden_desc: string;
  prioridad: string;
  estado: string;
  has_garantia: boolean;
  equipo_id: number;
  created_by: number;
  pre_informe: string;
  cotizacion_id?: number;
  informe_id?: number;
}

export function EquipoFormDialog({
  open,
  onOpenChange,
  onEquipoAdded,
}: EquipoFormDialogProps) {
  const { user } = useAuth();
  const [loading, setLoading] = useState(false);
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [tipoIngreso, setTipoIngreso] = useState<
    "almacenamiento" | "mantenimiento"
  >("almacenamiento");
  const [preInforme, setPreInforme] = useState("");
  const [formData, setFormData] = useState<Partial<CreateEquipoRequest>>({
    numero_serie: "",
    equipo_marca: "",
    equipo_modelo: "",
    equipo_tipo: "",
    equipo_precio: undefined,
    equipo_ubicacion: "",
    cliente_id: undefined,
    created_by: user?.usuario_id || 0,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  // Cargar clientes al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadClientes();
    }
  }, [open]);

  const loadClientes = async () => {
    try {
      const clientesData = await invoke<Cliente[]>("get_clientes");
      setClientes(clientesData);
    } catch (error) {
      console.error("Error cargando clientes:", error);
    }
  };
  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!formData.numero_serie?.trim()) {
      newErrors.numero_serie = "El número de serie es obligatorio";
    }
    if (!formData.equipo_marca?.trim()) {
      newErrors.equipo_marca = "La marca es obligatoria";
    }
    if (!formData.equipo_modelo?.trim()) {
      newErrors.equipo_modelo = "El modelo es obligatorio";
    }
    if (!formData.equipo_tipo) {
      newErrors.equipo_tipo = "El tipo es obligatorio";
    }
    if (!formData.cliente_id) {
      newErrors.cliente_id = "Debe seleccionar un cliente";
    }

    // Validar pre-informe si es para mantenimiento
    if (tipoIngreso === "mantenimiento" && !preInforme.trim()) {
      newErrors.preInforme =
        "El pre-informe es obligatorio para equipos de mantenimiento";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    try {
      setLoading(true);

      const equipoData: CreateEquipoRequest = {
        numero_serie: formData.numero_serie!,
        equipo_marca: formData.equipo_marca!,
        equipo_modelo: formData.equipo_modelo!,
        equipo_tipo: formData.equipo_tipo!,
        equipo_precio: formData.equipo_precio || undefined,
        equipo_ubicacion: formData.equipo_ubicacion || undefined,
        cliente_id: formData.cliente_id!,
        created_by: user?.usuario_id || 0,
      }; // Crear el equipo primero
      const equipoCreado = await invoke<Equipo>("create_equipo", {
        request: equipoData,
      });

      // Si es para mantenimiento, crear orden de trabajo
      if (tipoIngreso === "mantenimiento") {
        const fechaActual = new Date();
        const codigoOrden = `OT-${fechaActual.getFullYear()}${(
          fechaActual.getMonth() + 1
        )
          .toString()
          .padStart(2, "0")}${fechaActual
          .getDate()
          .toString()
          .padStart(2, "0")}-${equipoCreado.equipo_id}`;
        const ordenData: CreateOrdenTrabajoRequest = {
          orden_codigo: codigoOrden,
          orden_desc: `Orden de trabajo para ${formData.equipo_marca} ${formData.equipo_modelo} (S/N: ${formData.numero_serie})`,
          prioridad: "media",
          estado: "recibido",
          has_garantia: false,
          equipo_id: equipoCreado.equipo_id,
          created_by: user?.usuario_id || 0,
          pre_informe: preInforme,
          cotizacion_id: undefined,
          informe_id: undefined,
        };

        await invoke("create_orden_trabajo", { request: ordenData });
      }

      // Limpiar formulario
      setFormData({
        numero_serie: "",
        equipo_marca: "",
        equipo_modelo: "",
        equipo_tipo: "",
        equipo_precio: undefined,
        equipo_ubicacion: "",
        cliente_id: undefined,
        created_by: user?.usuario_id || 0,
      });
      setTipoIngreso("almacenamiento");
      setPreInforme("");
      setErrors({});

      onEquipoAdded();
    } catch (error) {
      console.error("Error creando equipo:", error);
      setErrors({ submit: "Error al crear el equipo. Intente nuevamente." });
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (field: keyof CreateEquipoRequest, value: any) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    // Limpiar error del campo cuando el usuario empiece a escribir
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: "" }));
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Agregar Nuevo Equipo</DialogTitle>
          <DialogDescription>
            Complete la información del equipo que desea registrar.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* Selector de tipo de ingreso */}
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm">Tipo de Ingreso</CardTitle>
              <CardDescription className="text-xs">
                Seleccione si el equipo es para almacenamiento o ingresa
                directamente a mantenimiento
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Select
                value={tipoIngreso}
                onValueChange={(value: "almacenamiento" | "mantenimiento") =>
                  setTipoIngreso(value)
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="almacenamiento">
                    📦 Almacenamiento - Solo registrar equipo
                  </SelectItem>
                  <SelectItem value="mantenimiento">
                    🔧 Mantenimiento - Crear orden de trabajo
                  </SelectItem>
                </SelectContent>
              </Select>
            </CardContent>
          </Card>

          {/* Campo de pre-informe - solo visible si es mantenimiento */}
          {tipoIngreso === "mantenimiento" && (
            <div className="space-y-2">
              <Label htmlFor="preInforme">
                Pre-informe / Observaciones del Cliente *
              </Label>
              <textarea
                id="preInforme"
                value={preInforme}
                onChange={(e) => {
                  setPreInforme(e.target.value);
                  if (errors.preInforme) {
                    setErrors((prev) => ({ ...prev, preInforme: "" }));
                  }
                }}
                placeholder="Ingrese las observaciones del cliente sobre el estado del equipo, problemas reportados, etc."
                className={`min-h-[100px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none ${
                  errors.preInforme ? "border-red-500" : ""
                }`}
              />
              {errors.preInforme && (
                <p className="text-sm text-red-500">{errors.preInforme}</p>
              )}
            </div>
          )}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="numero_serie">Número de Serie *</Label>
              <Input
                id="numero_serie"
                value={formData.numero_serie || ""}
                onChange={(e) =>
                  handleInputChange("numero_serie", e.target.value)
                }
                placeholder="Ej: NS123456"
                className={errors.numero_serie ? "border-red-500" : ""}
              />
              {errors.numero_serie && (
                <p className="text-sm text-red-500">{errors.numero_serie}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="equipo_marca">Marca *</Label>
              <Input
                id="equipo_marca"
                value={formData.equipo_marca || ""}
                onChange={(e) =>
                  handleInputChange("equipo_marca", e.target.value)
                }
                placeholder="Ej: Motorola"
                className={errors.equipo_marca ? "border-red-500" : ""}
              />
              {errors.equipo_marca && (
                <p className="text-sm text-red-500">{errors.equipo_marca}</p>
              )}
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="equipo_modelo">Modelo *</Label>
              <Input
                id="equipo_modelo"
                value={formData.equipo_modelo || ""}
                onChange={(e) =>
                  handleInputChange("equipo_modelo", e.target.value)
                }
                placeholder="Ej: DGP5050"
                className={errors.equipo_modelo ? "border-red-500" : ""}
              />
              {errors.equipo_modelo && (
                <p className="text-sm text-red-500">{errors.equipo_modelo}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="equipo_tipo">Tipo *</Label>
              <Select
                value={formData.equipo_tipo || ""}
                onValueChange={(value) =>
                  handleInputChange("equipo_tipo", value)
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
                  <SelectItem value="otro">Otro</SelectItem>
                </SelectContent>
              </Select>
              {errors.equipo_tipo && (
                <p className="text-sm text-red-500">{errors.equipo_tipo}</p>
              )}
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="cliente_id">Cliente *</Label>
            <Select
              value={formData.cliente_id?.toString() || ""}
              onValueChange={(value) =>
                handleInputChange("cliente_id", parseInt(value))
              }
            >
              <SelectTrigger
                className={errors.cliente_id ? "border-red-500" : ""}
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
              </SelectContent>
            </Select>
            {errors.cliente_id && (
              <p className="text-sm text-red-500">{errors.cliente_id}</p>
            )}
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="equipo_precio">Precio</Label>
              <Input
                id="equipo_precio"
                type="number"
                value={formData.equipo_precio || ""}
                onChange={(e) =>
                  handleInputChange(
                    "equipo_precio",
                    e.target.value ? parseInt(e.target.value) : undefined
                  )
                }
                placeholder="Ej: 150000"
                min="0"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="equipo_ubicacion">Ubicación</Label>
              <Input
                id="equipo_ubicacion"
                value={formData.equipo_ubicacion || ""}
                onChange={(e) =>
                  handleInputChange("equipo_ubicacion", e.target.value)
                }
                placeholder="Ej: Oficina Central"
              />
            </div>
          </div>

          {errors.submit && (
            <div className="text-sm text-red-500 bg-red-50 p-3 rounded-md">
              {errors.submit}
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
              {loading ? "Guardando..." : "Guardar Equipo"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
