import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button"
import { invoke } from "@tauri-apps/api/core"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { 
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
}

const loadEquipos = async () => {
  try {
    const equiposData = await invoke<Equipo[]>("get_equipos");
    return equiposData;
  } catch (error) {
    console.error("Error cargando equipos:", error);
    return [];
  }
};

export function EquipoHistorialDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [equipos, setEquipos] = useState<Equipo[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedEquipoId, setSelectedEquipoId] = useState<string | null>(null);
  const [tab, setTab] = useState<"select" | "historial">("select");

  useEffect(() => {
    if (open) {
      setTab("select");
      setSelectedEquipoId(null);
      const fetchEquipos = async () => {
        setLoading(true);
        const equiposData = await loadEquipos();
        setEquipos(equiposData);
        setLoading(false);
      };
      fetchEquipos();
    }
  }, [open]);

  const selectedEquipo = equipos.find(e => String(e.equipo_id) === selectedEquipoId);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Historial de Equipo</DialogTitle>
          <DialogDescription>
            Permite revisar el historial del equipo seleccionado.
          </DialogDescription>
        </DialogHeader>
        {tab === "select" && (
          <div className="grid gap-4">
            <div className="grid gap-3">
              <Select
                value={selectedEquipoId ?? ""}
                onValueChange={setSelectedEquipoId}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder={loading ? "Cargando..." : "Selecciona un Equipo"} />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectLabel>Equipos</SelectLabel>
                    {equipos.map((equipo) => (
                      <SelectItem key={equipo.equipo_id} value={String(equipo.equipo_id)}>
                        {equipo.numero_serie || `Equipo ${equipo.equipo_id}`}
                      </SelectItem>
                    ))}
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
            <DialogFooter>
              <Button onClick={() => onOpenChange(false)}>Cerrar</Button>
              <Button
                onClick={() => setTab("historial")}
                variant="outline"
                disabled={!selectedEquipoId}
              >
                Ver Historial
              </Button>
            </DialogFooter>
          </div>
        )}
        {tab === "historial" && (
          <div className="grid gap-4">
            <div>
              <strong>Número de Serie:</strong> {selectedEquipo?.numero_serie || "N/A"}
            </div>
            <div>
              <strong>Fecha de ingreso:</strong> 2024-01-01
            </div>
            <DialogFooter>
              <Button onClick={() => setTab("select")}>Regresar</Button>
              <Button onClick={() => onOpenChange(false)} variant="outline">Cerrar</Button>
            </DialogFooter>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
