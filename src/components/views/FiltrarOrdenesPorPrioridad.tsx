import { useState, useEffect } from "react";
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
  resetKey?: number; // 🔹 Nueva prop para reiniciar desde fuera
}

export function FiltrarOrdenesPorPrioridad({ onChange, resetKey }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const prioridadesDisponibles = ["Alta", "Media", "Baja"];

  // 🔹 Cuando cambie resetKey, limpiar selección
  useEffect(() => {
    if (resetKey !== undefined) {
      setSeleccionadas([]);
    }
  }, [resetKey]);

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
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionadas.length}
            <button
              onClick={(e) => {
                e.stopPropagation();
                limpiar();
              }}
              className="ml-1 text-red-500 hover:text-red-700 font-bold text-lg leading-none hover:bg-red-200 rounded-full px-1"
            >
              ×
            </button>
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

          <DialogFooter className="gap-2">
            {/* 🔹 Quitamos disabled para que siempre se pueda aplicar */}
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
