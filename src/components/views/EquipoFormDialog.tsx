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
import { useToastContext } from "@/contexts/ToastContext";

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

interface CreateClienteRequest {
  cliente_rut: string;
  cliente_nombre: string;
  cliente_correo: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by: number;
}

export function EquipoFormDialog({
  open,
  onOpenChange,
  onEquipoAdded,
}: EquipoFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [loading, setLoading] = useState(false);
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [marcas, setMarcas] = useState<string[]>([]);
  const [modelos, setModelos] = useState<string[]>([]);
  const [showNewMarcaInput, setShowNewMarcaInput] = useState(false);
  const [showNewModeloInput, setShowNewModeloInput] = useState(false);
  const [newMarcaValue, setNewMarcaValue] = useState("");
  const [newModeloValue, setNewModeloValue] = useState("");
  const [tipoIngreso, setTipoIngreso] = useState<
    "almacenamiento" | "mantenimiento"
  >("almacenamiento");
  const [preInforme, setPreInforme] = useState("");  const [showNewClienteDialog, setShowNewClienteDialog] = useState(false);
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
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
  >({});  const [formData, setFormData] = useState<Partial<CreateEquipoRequest>>({
    numero_serie: "",
    equipo_marca: "",
    equipo_modelo: "",
    equipo_tipo: "",
    equipo_ubicacion: "",
    cliente_id: undefined,
    created_by: user?.usuario_id || 0,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});
  // Cargar clientes al abrir el diálogo
  useEffect(() => {
    if (open) {
      loadClientes();
      loadMarcas();
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

  const loadMarcas = async () => {
    try {
      const marcasData = await invoke<string[]>("get_equipos_marcas");
      setMarcas(marcasData);
    } catch (error) {
      console.error("Error cargando marcas:", error);
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
      };      const nuevoCliente = await invoke<Cliente>("create_cliente", {
        request: clienteRequest,
      });      // Agregar el nuevo cliente a la lista inmediatamente
      setClientes(prev => [...prev, nuevoCliente]);

      // Seleccionar el nuevo cliente en el formulario
      setFormData(prev => ({ ...prev, cliente_id: nuevoCliente.cliente_id }));

      // Cerrar el modal y limpiar datos
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
  };  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    // Mostrar modal de confirmación en lugar de enviar directamente
    setShowConfirmationDialog(true);
  };

  const handleConfirmSubmit = async () => {
    try {
      setLoading(true);      const equipoData: CreateEquipoRequest = {
        numero_serie: formData.numero_serie!,
        equipo_marca: formData.equipo_marca!,
        equipo_modelo: formData.equipo_modelo!,
        equipo_tipo: formData.equipo_tipo!,
        equipo_precio: 0, // Precio fijo en 0
        equipo_ubicacion: formData.equipo_ubicacion || undefined,
        cliente_id: formData.cliente_id!,
        created_by: user?.usuario_id || 0,
      };// Crear el equipo primero
      const equipoCreado = await invoke<Equipo>("create_equipo", {
        request: equipoData,
      });

      let ordenCodigo: string | null = null;

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
          orden_desc: `El equipo ${
            formData.equipo_marca || "Marca desconocida"
          } ${
            formData.equipo_modelo || "Modelo desconocido"
          } presenta ${preInforme.trim()}`,
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
        ordenCodigo = codigoOrden;
      }

      // Mostrar notificación de éxito
      if (tipoIngreso === "mantenimiento" && ordenCodigo) {
        success(
          "¡Equipo creado y orden de trabajo generada!",
          `Equipo: ${formData.equipo_marca} ${formData.equipo_modelo} (S/N: ${formData.numero_serie})\nOrden de trabajo: ${ordenCodigo}`
        );
      } else {
        success(
          "¡Equipo creado exitosamente!",
          `${formData.equipo_marca} ${formData.equipo_modelo} (S/N: ${formData.numero_serie}) ha sido registrado correctamente.`
        );
      }

      // Cerrar diálogo
      onOpenChange(false);      // Limpiar formulario
      setFormData({
        numero_serie: "",
        equipo_marca: "",
        equipo_modelo: "",
        equipo_tipo: "",
        equipo_ubicacion: "",
        cliente_id: undefined,
        created_by: user?.usuario_id || 0,
      });
      setTipoIngreso("almacenamiento");
      setPreInforme("");
      setErrors({});
      setShowNewMarcaInput(false);
      setShowNewModeloInput(false);
      setNewMarcaValue("");
      setNewModeloValue("");
      setShowNewClienteDialog(false);
      setNewClienteData({
        cliente_rut: "",
        cliente_nombre: "",
        cliente_correo: "",
        cliente_telefono: "",
        cliente_direccion: "",
      });
      setNewClienteErrors({});
      onEquipoAdded();
    } catch (error) {
      console.error("Error creando equipo:", error);

      // Mostrar notificación de error
      showError(
        "Error al crear el equipo",
        typeof error === "string"
          ? error
          : "Ha ocurrido un error inesperado. Por favor, intente nuevamente."
      );      setErrors({ submit: "Error al crear el equipo. Intente nuevamente." });
    } finally {
      setLoading(false);
      setShowConfirmationDialog(false);
    }
  };
  const handleInputChange = (field: keyof CreateEquipoRequest, value: any) => {
    setFormData((prev) => ({ ...prev, [field]: value }));

    // Si cambia la marca, cargar modelos y limpiar el modelo actual
    if (field === "equipo_marca" && value) {
      loadModelosByMarca(value);
      setFormData((prev) => ({ ...prev, equipo_modelo: "" }));
      setShowNewModeloInput(false);
      setNewModeloValue("");
    }

    // Limpiar error del campo cuando el usuario empiece a escribir
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: "" }));
    }
  };
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="!max-w-4xl max-h-[90vh] overflow-y-auto">
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
            </div>{" "}
            <div className="space-y-2">
              <Label htmlFor="equipo_marca">Marca *</Label>
              {showNewMarcaInput ? (
                <div className="flex gap-2">
                  <Input
                    value={newMarcaValue}
                    onChange={(e) => setNewMarcaValue(e.target.value)}
                    placeholder="Ingrese nueva marca"
                    className={errors.equipo_marca ? "border-red-500" : ""}
                  />                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      if (newMarcaValue.trim()) {
                        const nuevaMarca = newMarcaValue.trim();
                        // Agregar la nueva marca a la lista si no existe
                        if (!marcas.includes(nuevaMarca)) {
                          setMarcas(prev => [...prev, nuevaMarca]);
                        }
                        handleInputChange("equipo_marca", nuevaMarca);
                        setShowNewMarcaInput(false);
                        setNewMarcaValue("");
                      }
                    }}
                  >
                    ✓
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      setShowNewMarcaInput(false);
                      setNewMarcaValue("");
                    }}
                  >
                    ✕
                  </Button>
                </div>
              ) : (
                <Select
                  value={formData.equipo_marca || ""}
                  onValueChange={(value) => {
                    if (value === "nueva_marca") {
                      setShowNewMarcaInput(true);
                    } else {
                      handleInputChange("equipo_marca", value);
                    }
                  }}
                >
                  <SelectTrigger
                    className={errors.equipo_marca ? "border-red-500" : ""}
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
                      ➕ Agregar nueva marca
                    </SelectItem>
                  </SelectContent>
                </Select>
              )}
              {errors.equipo_marca && (
                <p className="text-sm text-red-500">{errors.equipo_marca}</p>
              )}
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            {" "}
            <div className="space-y-2">
              <Label htmlFor="equipo_modelo">Modelo *</Label>
              {showNewModeloInput ? (
                <div className="flex gap-2">
                  <Input
                    value={newModeloValue}
                    onChange={(e) => setNewModeloValue(e.target.value)}
                    placeholder="Ingrese nuevo modelo"
                    className={errors.equipo_modelo ? "border-red-500" : ""}
                    disabled={!formData.equipo_marca}
                  />                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      if (newModeloValue.trim()) {
                        const nuevoModelo = newModeloValue.trim();
                        // Agregar el nuevo modelo a la lista si no existe
                        if (!modelos.includes(nuevoModelo)) {
                          setModelos(prev => [...prev, nuevoModelo]);
                        }
                        handleInputChange(
                          "equipo_modelo",
                          nuevoModelo
                        );
                        setShowNewModeloInput(false);
                        setNewModeloValue("");
                      }
                    }}
                    disabled={!formData.equipo_marca}
                  >
                    ✓
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      setShowNewModeloInput(false);
                      setNewModeloValue("");
                    }}
                  >
                    ✕
                  </Button>
                </div>
              ) : (
                <Select
                  value={formData.equipo_modelo || ""}
                  onValueChange={(value) => {
                    if (value === "nuevo_modelo") {
                      setShowNewModeloInput(true);
                    } else {
                      handleInputChange("equipo_modelo", value);
                    }
                  }}
                  disabled={!formData.equipo_marca}
                >
                  <SelectTrigger
                    className={errors.equipo_modelo ? "border-red-500" : ""}
                    disabled={!formData.equipo_marca}
                  >
                    <SelectValue
                      placeholder={
                        !formData.equipo_marca
                          ? "Seleccione una marca primero"
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
                    {formData.equipo_marca && (
                      <SelectItem value="nuevo_modelo">
                        ➕ Agregar nuevo modelo
                      </SelectItem>
                    )}
                  </SelectContent>
                </Select>
              )}
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
          </div>{" "}
          <div className="space-y-2">
            <Label htmlFor="cliente_id">Cliente *</Label>
            <Select
              value={formData.cliente_id?.toString() || ""}
              onValueChange={(value) => {
                if (value === "nuevo_cliente") {
                  setShowNewClienteDialog(true);
                } else {
                  handleInputChange("cliente_id", parseInt(value));
                }
              }}
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
                <SelectItem value="nuevo_cliente">
                  ➕ Agregar nuevo cliente
                </SelectItem>
              </SelectContent>
            </Select>
            {errors.cliente_id && (
              <p className="text-sm text-red-500">{errors.cliente_id}</p>
            )}
          </div>          <div className="grid grid-cols-2 gap-4">
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
            <Button type="submit" disabled={loading}>
              {loading ? "Guardando..." : "Guardar Equipo"}
            </Button>{" "}
          </DialogFooter>
        </form>
      </DialogContent>

      {/* Modal de confirmación */}
      <Dialog
        open={showConfirmationDialog}
        onOpenChange={setShowConfirmationDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Creación de Equipo</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea crear este equipo con la siguiente información?
            </DialogDescription>
          </DialogHeader>
          
          <div className="space-y-2 text-sm">
            <div><strong>Marca:</strong> {formData.equipo_marca}</div>
            <div><strong>Modelo:</strong> {formData.equipo_modelo}</div>
            <div><strong>Número de Serie:</strong> {formData.numero_serie}</div>
            <div><strong>Tipo:</strong> {formData.equipo_tipo}</div>
            <div><strong>Cliente:</strong> {clientes.find(c => c.cliente_id === formData.cliente_id)?.cliente_nombre}</div>
            {formData.equipo_ubicacion && (
              <div><strong>Ubicación:</strong> {formData.equipo_ubicacion}</div>
            )}
            <div><strong>Tipo de Ingreso:</strong> {tipoIngreso === "mantenimiento" ? "Mantenimiento (se creará orden de trabajo)" : "Almacenamiento"}</div>
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
            <Button 
              onClick={handleConfirmSubmit}
              disabled={loading}
            >
              {loading ? "Creando..." : "Confirmar y Crear"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Modal para agregar nuevo cliente */}
      <Dialog
        open={showNewClienteDialog}
        onOpenChange={setShowNewClienteDialog}
      >
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>Agregar Nuevo Cliente</DialogTitle>
            <DialogDescription>
              Complete la información del nuevo cliente.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="nuevo_cliente_rut">RUT *</Label>
              <Input
                id="nuevo_cliente_rut"
                value={newClienteData.cliente_rut}
                onChange={(e) => {
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_rut: e.target.value,
                  }));
                  if (newClienteErrors.cliente_rut) {
                    setNewClienteErrors((prev) => ({
                      ...prev,
                      cliente_rut: "",
                    }));
                  }
                }}
                placeholder="Ej: 12.345.678-9"
                className={newClienteErrors.cliente_rut ? "border-red-500" : ""}
              />
              {newClienteErrors.cliente_rut && (
                <p className="text-sm text-red-500">
                  {newClienteErrors.cliente_rut}
                </p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="nuevo_cliente_nombre">Nombre *</Label>
              <Input
                id="nuevo_cliente_nombre"
                value={newClienteData.cliente_nombre}
                onChange={(e) => {
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_nombre: e.target.value,
                  }));
                  if (newClienteErrors.cliente_nombre) {
                    setNewClienteErrors((prev) => ({
                      ...prev,
                      cliente_nombre: "",
                    }));
                  }
                }}
                placeholder="Nombre completo del cliente"
                className={
                  newClienteErrors.cliente_nombre ? "border-red-500" : ""
                }
              />
              {newClienteErrors.cliente_nombre && (
                <p className="text-sm text-red-500">
                  {newClienteErrors.cliente_nombre}
                </p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="nuevo_cliente_correo">Correo Electrónico *</Label>
              <Input
                id="nuevo_cliente_correo"
                type="email"
                value={newClienteData.cliente_correo}
                onChange={(e) => {
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_correo: e.target.value,
                  }));
                  if (newClienteErrors.cliente_correo) {
                    setNewClienteErrors((prev) => ({
                      ...prev,
                      cliente_correo: "",
                    }));
                  }
                }}
                placeholder="cliente@ejemplo.com"
                className={
                  newClienteErrors.cliente_correo ? "border-red-500" : ""
                }
              />
              {newClienteErrors.cliente_correo && (
                <p className="text-sm text-red-500">
                  {newClienteErrors.cliente_correo}
                </p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="nuevo_cliente_telefono">Teléfono</Label>
              <Input
                id="nuevo_cliente_telefono"
                value={newClienteData.cliente_telefono}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_telefono: e.target.value,
                  }))
                }
                placeholder="Ej: +56 9 1234 5678"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="nuevo_cliente_direccion">Dirección</Label>
              <Input
                id="nuevo_cliente_direccion"
                value={newClienteData.cliente_direccion}
                onChange={(e) =>
                  setNewClienteData((prev) => ({
                    ...prev,
                    cliente_direccion: e.target.value,
                  }))
                }
                placeholder="Dirección completa del cliente"
              />
            </div>
          </div>

          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => {
                setShowNewClienteDialog(false);
                setNewClienteData({
                  cliente_rut: "",
                  cliente_nombre: "",
                  cliente_correo: "",
                  cliente_telefono: "",
                  cliente_direccion: "",
                });
                setNewClienteErrors({});
              }}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button
              type="button"
              onClick={handleCreateCliente}
              disabled={loading}
            >
              {loading ? "Creando..." : "Crear Cliente"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}
