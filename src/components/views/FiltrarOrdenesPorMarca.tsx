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
  onChange: (marcas: string[]) => void;
}

export function FiltrarOrdenesPorMarca({ onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const marcas = ["Marca1", "Marca2", "Marca3"]; // Reemplazar con tus marcas reales

  const toggleMarca = (m: string) => {
    setSeleccionadas((prev) =>
      prev.includes(m) ? prev.filter((x) => x !== m) : [...prev, m]
    );
  };

  const aplicar = () => {
    onChange(seleccionadas);
    setOpen(false);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Marca
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Selecciona Marcas</DialogTitle>
          </DialogHeader>
          <div className="space-y-2 mt-2">
            {marcas.map((m) => (
              <label key={m} className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={seleccionadas.includes(m)}
                  onChange={() => toggleMarca(m)}
                />
                <span>{m}</span>
              </label>
            ))}
          </div>
          <DialogFooter>
            <Button onClick={aplicar}>Aplicar</Button>
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
