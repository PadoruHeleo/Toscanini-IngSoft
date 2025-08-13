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

  const prioridades = ["Alta", "Media", "Baja"];

  const togglePrioridad = (p: string) => {
    setSeleccionadas((prev) =>
      prev.includes(p) ? prev.filter((x) => x !== p) : [...prev, p]
    );
  };

  const aplicar = () => {
    onChange(seleccionadas.map((p) => p.toLowerCase()));
    setOpen(false);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Prioridad
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
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
          <DialogFooter>
            <Button onClick={aplicar} disabled={seleccionadas.length === 0}>
              Aplicar
            </Button>
            <Button
              variant="outline"
              onClick={() => {
                setSeleccionadas([]);
                onChange([]);
                setOpen(false);
              }}
            >
              Quitar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
