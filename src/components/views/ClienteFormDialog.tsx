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
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";

interface Cliente {
  cliente_id: number;
  cliente_rut?: string;
  cliente_nombre?: string;
  cliente_correo?: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by?: number;
  created_at?: string;
}

interface CreateClienteRequest {
  cliente_rut: string;
  cliente_nombre: string;
  cliente_correo: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by: number;
}

interface UpdateClienteRequest {
  cliente_rut?: string;
  cliente_nombre?: string;
  cliente_correo?: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
}

interface ClienteFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onClienteAdded: () => void;
  cliente?: Cliente;
  isEditing?: boolean;
}

export function ClienteFormDialog({
  open,
  onOpenChange,
  onClienteAdded,
  cliente,
  isEditing = false,
}: ClienteFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [loading, setLoading] = useState(false);
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  const [formData, setFormData] = useState({
    cliente_rut: "",
    cliente_nombre: "",
    cliente_correo: "",
    cliente_telefono: "",
    cliente_direccion: "",
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  // Cargar datos del cliente al editar
  useEffect(() => {
    if (isEditing && cliente) {
      setFormData({
        cliente_rut: cliente.cliente_rut || "",
        cliente_nombre: cliente.cliente_nombre || "",
        cliente_correo: cliente.cliente_correo || "",
        cliente_telefono: cliente.cliente_telefono || "",
        cliente_direccion: cliente.cliente_direccion || "",
      });
    } else {
      setFormData({
        cliente_rut: "",
        cliente_nombre: "",
        cliente_correo: "",
        cliente_telefono: "",
        cliente_direccion: "",
      });
    }
    setErrors({});
  }, [isEditing, cliente, open]);

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!formData.cliente_rut.trim()) {
      newErrors.cliente_rut = "El RUT es obligatorio";
    }
    if (!formData.cliente_nombre.trim()) {
      newErrors.cliente_nombre = "El nombre es obligatorio";
    }
    if (!formData.cliente_correo.trim()) {
      newErrors.cliente_correo = "El correo es obligatorio";
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.cliente_correo)) {
      newErrors.cliente_correo = "El formato del correo no es válido";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

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

      if (isEditing && cliente) {
        // Actualizar cliente existente
        const updateRequest: UpdateClienteRequest = {
          cliente_rut: formData.cliente_rut || undefined,
          cliente_nombre: formData.cliente_nombre || undefined,
          cliente_correo: formData.cliente_correo || undefined,
          cliente_telefono: formData.cliente_telefono || undefined,
          cliente_direccion: formData.cliente_direccion || undefined,
        };

        const updatedCliente = await invoke<Cliente>("update_cliente", {
          clienteId: cliente.cliente_id,
          request: updateRequest,
          updatedBy: user?.usuario_id || 0,
        });

        if (updatedCliente) {
          success(
            "¡Cliente actualizado exitosamente!",
            `${formData.cliente_nombre} ha sido actualizado correctamente.`
          );
        } else {
          showError("Error", "No se pudo actualizar el cliente.");
          return;
        }
      } else {
        // Crear nuevo cliente
        const createRequest: CreateClienteRequest = {
          cliente_rut: formData.cliente_rut,
          cliente_nombre: formData.cliente_nombre,
          cliente_correo: formData.cliente_correo,
          cliente_telefono: formData.cliente_telefono || undefined,
          cliente_direccion: formData.cliente_direccion || undefined,
          created_by: user?.usuario_id || 0,
        };

        await invoke<Cliente>("create_cliente", {
          request: createRequest,
        });

        success(
          "¡Cliente creado exitosamente!",
          `${formData.cliente_nombre} ha sido registrado correctamente.`
        );
      }

      // Cerrar diálogo
      onOpenChange(false);

      // Limpiar formulario
      setFormData({
        cliente_rut: "",
        cliente_nombre: "",
        cliente_correo: "",
        cliente_telefono: "",
        cliente_direccion: "",
      });
      setErrors({});
      onClienteAdded();
    } catch (error) {
      console.error("Error procesando cliente:", error);

      showError(
        isEditing ? "Error al actualizar cliente" : "Error al crear cliente",
        typeof error === "string"
          ? error
          : "Ha ocurrido un error inesperado. Por favor, intente nuevamente."
      );
      setErrors({
        submit: `Error al ${
          isEditing ? "actualizar" : "crear"
        } el cliente. Intente nuevamente.`,
      });
    } finally {
      setLoading(false);
      setShowConfirmationDialog(false);
    }
  };

  const handleInputChange = (field: keyof typeof formData, value: string) => {
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
          <DialogTitle>
            {isEditing ? "Editar Cliente" : "Agregar Nuevo Cliente"}
          </DialogTitle>
          <DialogDescription>
            {isEditing
              ? "Modifique la información del cliente."
              : "Complete la información del cliente que desea registrar."}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="cliente_rut">RUT *</Label>
            <Input
              id="cliente_rut"
              value={formData.cliente_rut}
              onChange={(e) => handleInputChange("cliente_rut", e.target.value)}
              placeholder="Ej: 12.345.678-9"
              className={errors.cliente_rut ? "border-red-500" : ""}
            />
            {errors.cliente_rut && (
              <p className="text-sm text-red-500">{errors.cliente_rut}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="cliente_nombre">Nombre *</Label>
            <Input
              id="cliente_nombre"
              value={formData.cliente_nombre}
              onChange={(e) =>
                handleInputChange("cliente_nombre", e.target.value)
              }
              placeholder="Nombre completo del cliente"
              className={errors.cliente_nombre ? "border-red-500" : ""}
            />
            {errors.cliente_nombre && (
              <p className="text-sm text-red-500">{errors.cliente_nombre}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="cliente_correo">Correo Electrónico *</Label>
            <Input
              id="cliente_correo"
              type="email"
              value={formData.cliente_correo}
              onChange={(e) =>
                handleInputChange("cliente_correo", e.target.value)
              }
              placeholder="cliente@ejemplo.com"
              className={errors.cliente_correo ? "border-red-500" : ""}
            />
            {errors.cliente_correo && (
              <p className="text-sm text-red-500">{errors.cliente_correo}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="cliente_telefono">Teléfono</Label>
            <Input
              id="cliente_telefono"
              value={formData.cliente_telefono}
              onChange={(e) =>
                handleInputChange("cliente_telefono", e.target.value)
              }
              placeholder="Ej: +56 9 1234 5678"
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="cliente_direccion">Dirección</Label>
            <Input
              id="cliente_direccion"
              value={formData.cliente_direccion}
              onChange={(e) =>
                handleInputChange("cliente_direccion", e.target.value)
              }
              placeholder="Dirección completa del cliente"
            />
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
              {loading
                ? isEditing
                  ? "Actualizando..."
                  : "Creando..."
                : isEditing
                ? "Actualizar Cliente"
                : "Crear Cliente"}
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
                : "Confirmar Creación de Cliente"}
            </DialogTitle>
            <DialogDescription>
              {isEditing
                ? "¿Está seguro que desea actualizar este cliente con los cambios realizados?"
                : "¿Está seguro que desea crear este cliente con la siguiente información?"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2 text-sm">
            <div>
              <strong>RUT:</strong> {formData.cliente_rut}
            </div>
            <div>
              <strong>Nombre:</strong> {formData.cliente_nombre}
            </div>
            <div>
              <strong>Correo:</strong> {formData.cliente_correo}
            </div>
            {formData.cliente_telefono && (
              <div>
                <strong>Teléfono:</strong> {formData.cliente_telefono}
              </div>
            )}
            {formData.cliente_direccion && (
              <div>
                <strong>Dirección:</strong> {formData.cliente_direccion}
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
