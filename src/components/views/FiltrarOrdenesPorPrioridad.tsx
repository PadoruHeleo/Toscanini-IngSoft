import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";

interface Props {
  onChange: (prioridades: string[]) => void;
}

export function FiltrarOrdenesPorPrioridad({ onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const prioridadesDisponibles = ["Alta", "Media", "Baja"];

  const togglePrioridad = (p: string) => {
    setSeleccionadas((prev) =>
      prev.includes(p) ? prev.filter((x) => x !== p) : [...prev, p]
    );
  };

  const aplicar = () => {
    const prioridadesParaBackend = seleccionadas.map((p) => p.toLowerCase());
    onChange(prioridadesParaBackend);
    setOpen(false);
  };

  const limpiar = () => {
    setSeleccionadas([]);
    onChange([]);
    setOpen(false);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Prioridad
        {seleccionadas.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs">
            {seleccionadas.length}
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Selecciona Prioridades</DialogTitle>
          </DialogHeader>

          <div className="space-y-2 mt-2 max-h-60 overflow-y-auto">
            {prioridadesDisponibles.map((p) => (
              <label
                key={p}
                className="flex items-center gap-2 hover:bg-gray-50 p-1 rounded cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={seleccionadas.includes(p)}
                  onChange={() => togglePrioridad(p)}
                  className="rounded"
                />
                <span className="capitalize">{p}</span>
              </label>
            ))}
          </div>

          <DialogFooter>
            <Button onClick={aplicar}>
              Aplicar {seleccionadas.length > 0 && `(${seleccionadas.length})`}
            </Button>
            <Button variant="outline" onClick={limpiar}>
              Limpiar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
