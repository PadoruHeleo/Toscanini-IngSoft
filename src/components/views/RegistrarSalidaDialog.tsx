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
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";
import { AlertCircle, Package, LogOut } from "lucide-react";
import { Alert, AlertDescription } from "@/components/ui/alert";

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  cliente_id?: number;
}

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string;
  estado?: string;
  equipo_id?: number;
}

interface RegistrarSalidaRequest {
  equipo_id: number;
  orden_trabajo_id?: number;
  motivo_salida: string;
  observaciones?: string;
  usuario_id: number;
}

interface SalidaEquipoResponse {
  success: boolean;
  message: string;
  nuevo_estado?: string;
}

interface RegistrarSalidaDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  equipo: Equipo | null;
  ordenTrabajo?: OrdenTrabajo | null;
  onSalidaRegistrada: () => void;
}

const motivosOptions = [
  {
    value: "entregado_cliente",
    label: "Entregado al cliente",
    description: "El equipo ha sido entregado exitosamente al cliente",
  },
  {
    value: "retirado_sin_reparacion",
    label: "Retirado sin reparación",
    description: "El equipo fue retirado sin realizar reparación",
  },
  {
    value: "abandonado",
    label: "Equipo abandonado",
    description: "El equipo ha sido abandonado por el cliente",
  },
  {
    value: "baja_definitiva",
    label: "Baja definitiva",
    description: "El equipo se da de baja definitiva del inventario",
  },
];

export function RegistrarSalidaDialog({
  open,
  onOpenChange,
  equipo,
  ordenTrabajo,
  onSalidaRegistrada,
}: RegistrarSalidaDialogProps) {
  const { user } = useAuth();
  const { success, error } = useToastContext();

  const [loading, setLoading] = useState(false);
  const [canRegister, setCanRegister] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [formData, setFormData] = useState({
    motivo_salida: "",
    observaciones: "",
  });
  const [errors, setErrors] = useState<{ [key: string]: string }>({});

  // Verificar si se puede registrar salida cuando se abre el diálogo
  useEffect(() => {
    if (open && equipo) {
      checkCanRegisterSalida();
    }
  }, [open, equipo]);

  const checkCanRegisterSalida = async () => {
    if (!equipo) return;

    try {
      const [canReg, message] = await invoke<[boolean, string]>(
        "puede_registrar_salida_equipo",
        { equipoId: equipo.equipo_id }
      );
      setCanRegister(canReg);
      setStatusMessage(message);
    } catch (error) {
      console.error("Error verificando estado:", error);
      setCanRegister(false);
      setStatusMessage("Error verificando el estado del equipo");
    }
  };

  const validateForm = (): boolean => {
    const newErrors: { [key: string]: string } = {};

    if (!formData.motivo_salida) {
      newErrors.motivo_salida = "Debe seleccionar un motivo de salida";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm() || !user || !equipo) return;

    setLoading(true);

    try {
      const request: RegistrarSalidaRequest = {
        equipo_id: equipo.equipo_id,
        orden_trabajo_id: ordenTrabajo?.orden_id,
        motivo_salida: formData.motivo_salida,
        observaciones: formData.observaciones || undefined,
        usuario_id: user.usuario_id,
      };

      const response = await invoke<SalidaEquipoResponse>(
        "registrar_salida_equipo",
        { request }
      );

      if (response.success) {
        success("Salida registrada", response.message);
        onSalidaRegistrada();
        onOpenChange(false);
        resetForm();
      } else {
        error("Error", response.message);
      }
    } catch (err) {
      console.error("Error registrando salida:", err);
      error("Error", `No se pudo registrar la salida: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setFormData({
      motivo_salida: "",
      observaciones: "",
    });
    setErrors({});
    setCanRegister(false);
    setStatusMessage("");
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!newOpen) {
      resetForm();
    }
    onOpenChange(newOpen);
  };

  const getEquipoDisplay = () => {
    if (!equipo) return "Equipo no disponible";
    return `${equipo.equipo_marca || "N/A"} ${
      equipo.equipo_modelo || "N/A"
    } - Serie: ${equipo.numero_serie || "N/A"}`;
  };

  const selectedMotivo = motivosOptions.find(
    (m) => m.value === formData.motivo_salida
  );

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <LogOut className="h-5 w-5" />
            Registrar Salida de Equipo
          </DialogTitle>
          <DialogDescription>
            Registre la salida del equipo del inventario con el motivo
            correspondiente.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* Información del equipo */}
          <div className="bg-gray-50 p-3 rounded-lg">
            <div className="flex items-center gap-2 mb-2">
              <Package className="h-4 w-4" />
              <span className="font-medium">Equipo seleccionado</span>
            </div>
            <p className="text-sm text-gray-700">{getEquipoDisplay()}</p>
            {ordenTrabajo && (
              <p className="text-sm text-blue-600 mt-1">
                Orden: {ordenTrabajo.orden_codigo} - Estado:{" "}
                {ordenTrabajo.estado}
              </p>
            )}
          </div>

          {/* Estado de verificación */}
          <Alert variant={canRegister ? "default" : "destructive"}>
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>
              <div className="space-y-1">
                <p>{statusMessage}</p>
                {canRegister && (
                  <p className="text-xs text-muted-foreground">
                    Estados válidos para registro de salida: Recibido,
                    Cotización enviada, Aprobación pendiente, En reparación,
                    Espera de retiro
                  </p>
                )}
                {!canRegister && statusMessage.includes("Ya se registró") && (
                  <p className="text-xs text-muted-foreground">
                    El equipo ya ha salido del sistema. Estados finales:
                    Entregado, Abandonado, No reparable
                  </p>
                )}
              </div>
            </AlertDescription>
          </Alert>

          {canRegister && (
            <>
              {/* Motivo de salida */}
              <div className="space-y-2">
                <Label htmlFor="motivo_salida">Motivo de salida *</Label>
                <Select
                  value={formData.motivo_salida}
                  onValueChange={(value) =>
                    setFormData((prev) => ({ ...prev, motivo_salida: value }))
                  }
                >
                  <SelectTrigger
                    className={errors.motivo_salida ? "border-red-500" : ""}
                  >
                    <SelectValue placeholder="Seleccionar motivo..." />
                  </SelectTrigger>
                  <SelectContent>
                    {motivosOptions.map((motivo) => (
                      <SelectItem key={motivo.value} value={motivo.value}>
                        {motivo.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {errors.motivo_salida && (
                  <p className="text-sm text-red-500">{errors.motivo_salida}</p>
                )}
                {selectedMotivo && (
                  <p className="text-sm text-gray-600">
                    {selectedMotivo.description}
                  </p>
                )}
              </div>

              {/* Observaciones */}
              <div className="space-y-2">
                <Label htmlFor="observaciones">Observaciones</Label>
                <Textarea
                  id="observaciones"
                  placeholder="Observaciones adicionales (opcional)..."
                  value={formData.observaciones}
                  onChange={(e) =>
                    setFormData((prev) => ({
                      ...prev,
                      observaciones: e.target.value,
                    }))
                  }
                  rows={3}
                />
              </div>
            </>
          )}
        </form>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => handleOpenChange(false)}
            disabled={loading}
          >
            Cancelar
          </Button>
          {canRegister && (
            <Button
              type="submit"
              onClick={handleSubmit}
              disabled={loading || !formData.motivo_salida}
            >
              {loading ? "Registrando..." : "Registrar Salida"}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
