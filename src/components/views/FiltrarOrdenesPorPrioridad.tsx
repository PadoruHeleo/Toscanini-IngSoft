// src/components/FiltrarOrdenesPorPrioridad.tsx
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/core";

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  has_garantia?: boolean;
  created_at?: string;
}

interface Props {
  onFiltrar: (ordenes: OrdenTrabajo[]) => void;
}

export function FiltrarOrdenesPorPrioridad({ onFiltrar }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);

  // Labels visibles; convertiremos a minúsculas antes de enviar al backend
  const prioridades = ["Alta", "Media", "Baja"];

  const togglePrioridad = (p: string) => {
    setSeleccionadas((prev) =>
      prev.includes(p) ? prev.filter((x) => x !== p) : [...prev, p]
    );
  };

  const aplicarFiltro = async () => {
    try {
      // mapear etiquetas a los valores de BD (ej: "Alta" -> "alta")
      const prioridadesBd = seleccionadas.map((p) => p.toLowerCase());

      // Llamada al backend
      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_by_prioridades",
        { prioridades: prioridadesBd }
      );
      onFiltrar(ordenes);
      setOpen(false);
    } catch (err) {
      console.error("Error aplicando filtro por prioridad:", err);
    }
  };

  const quitarFiltro = async () => {
    try {
      const ordenes = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(ordenes);
      setSeleccionadas([]);
      setOpen(false);
    } catch (err) {
      console.error("Error quitando filtro por prioridad:", err);
    }
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Prioridad
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Selecciona Prioridades</DialogTitle>
          </DialogHeader>

          <div className="space-y-2 mt-2">
            {prioridades.map((p) => (
              <label key={p} className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={seleccionadas.includes(p)}
                  onChange={() => togglePrioridad(p)}
                />
                <span className="capitalize">{p}</span>
              </label>
            ))}
          </div>

          <DialogFooter className="mt-4">
            <Button
              onClick={aplicarFiltro}
              disabled={seleccionadas.length === 0}
            >
              Aplicar
            </Button>
            <Button variant="outline" onClick={quitarFiltro}>
              Quitar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
