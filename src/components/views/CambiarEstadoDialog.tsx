import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  orden: { orden_id: number; orden_codigo?: string; estado?: string; equipo_id?: number } | null;
  onCambioCompletado?: () => void;
}

const ESTADOS = [
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

export default function CambiarEstadoDialog({ open, onOpenChange, orden, onCambioCompletado }: Props) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();

  const [equipo, setEquipo] = useState<any | null>(null);
  const [nuevoEstado, setNuevoEstado] = useState<string | undefined>(undefined);
  const [causa, setCausa] = useState("");
  const [loading, setLoading] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  useEffect(() => {
    if (open && orden) {
      setNuevoEstado(orden.estado ?? undefined);
      setCausa("");
      setEquipo(null);
      // cargar equipo si existe equipo_id
      if (orden.equipo_id) {
        invoke("get_equipo_by_id", { equipoId: orden.equipo_id })
          .then((res) => setEquipo(res))
          .catch((err) => {
            console.error("Error cargando equipo:", err);
            // no bloquear el modal por error de equipo
          });
      }
    }
  }, [open, orden]);

  const handleConfirmChange = async () => {
    if (!orden) return;
    if (!nuevoEstado) {
      showError("Error", "Seleccione un estado de destino.");
      return;
    }
    if (!causa.trim()) {
      showError("Error", "Debe ingresar la causa del cambio de estado.");
      return;
    }
    setShowConfirm(false);
    setLoading(true);
    try {
      await invoke("cambiar_estado_orden_trabajo", {
        ordenId: orden.orden_id,
        nuevoEstado,
        updatedBy: user?.usuario_id,
        causa: causa || null,
      });
      success("Estado cambiado", `La orden ${orden.orden_codigo} fue actualizada a "${nuevoEstado}".`);
      onOpenChange(false);
      onCambioCompletado?.();
    } catch (err) {
      console.error("Error cambiando estado:", err);
      showError("Error", "No se pudo cambiar el estado de la orden.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Cambiar estado - {orden?.orden_codigo ?? "N/A"}</DialogTitle>
          </DialogHeader>

          <div className="space-y-4">
            <div className="text-sm text-muted-foreground">
              <strong>Equipo seleccionado:</strong>
              <div className="mt-1">
                {equipo ? (
                  <>
                    {equipo.marca ? <div>{equipo.marca} {equipo.modelo}</div> : null}
                    {equipo.serie ? <div className="text-xs text-muted-foreground">Serie: {equipo.serie}</div> : null}
                  </>
                ) : (
                  <div className="text-xs text-muted-foreground">No hay equipo vinculado o cargando...</div>
                )}
              </div>
            </div>

            <div>
              <label className="text-sm font-medium">Estado actual</label>
              <div className="mt-1">
                <Input readOnly value={orden?.estado ?? "N/A"} />
              </div>
            </div>

            <div>
              <label className="text-sm font-medium">Cambiar estado a</label>
              <div className="mt-1">
                <Select onValueChange={(val) => setNuevoEstado(val)} value={nuevoEstado}>
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Seleccione estado..." />
                  </SelectTrigger>
                  <SelectContent>
                    {ESTADOS.map((s) => (
                      <SelectItem key={s.value} value={s.value}>
                        {s.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div>
              <label className="text-sm font-medium">Causa</label>
              <div className="mt-1">
                <Textarea placeholder="Describa la causa del cambio (opcional)..." value={causa} onChange={(e) => setCausa(e.target.value)} />
              </div>
            </div>
          </div>

          <DialogFooter>
            <div className="flex gap-2 w-full justify-end">
              <Button variant="ghost" onClick={() => onOpenChange(false)} disabled={loading}>Cancelar</Button>
              <Button
                onClick={() => setShowConfirm(true)}
                disabled={loading || !nuevoEstado || !causa.trim()}
                title={!causa.trim() ? "Debe ingresar la causa" : undefined}
              >
                Cambiar estado
              </Button>
            </div>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Confirmación */}
      <Dialog open={showConfirm} onOpenChange={setShowConfirm}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirme cambio de estado</DialogTitle>
          </DialogHeader>
          <div className="py-2">
            ¿Está seguro que desea cambiar el estado de la orden {orden?.orden_codigo} de "{orden?.estado}" a "{nuevoEstado}"?
            {causa && <div className="mt-2 text-sm text-muted-foreground"><strong>Causa:</strong> {causa}</div>}
          </div>
          <DialogFooter>
            <div className="flex gap-2 w-full justify-end">
              <Button variant="ghost" onClick={() => setShowConfirm(false)} disabled={loading}>Cancelar</Button>
              <Button
                onClick={handleConfirmChange}
                disabled={loading || !causa.trim()}
                title={!causa.trim() ? "Debe ingresar la causa" : undefined}
              >
                {loading ? "Procesando..." : "Confirmar cambio"}
              </Button>
            </div>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}