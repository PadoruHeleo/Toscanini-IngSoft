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
  onChange: (estados: string[]) => void;
  resetKey?: number;
}

export function FiltrarOrdenesPorEstado({ onChange, resetKey }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);

  const estadosDisponibles = [
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

  // Reset cuando resetKey cambia (igual que otros filtros)
  useEffect(() => {
    if (resetKey !== undefined) {
      setSeleccionados([]);
      onChange([]); // notificar al padre que se limpió
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [resetKey]);

  const toggleEstado = (estado: string) => {
    setSeleccionados((prev) =>
      prev.includes(estado)
        ? prev.filter((x) => x !== estado)
        : [...prev, estado]
    );
  };

  const aplicar = () => {
    onChange(seleccionados);
    setOpen(false);
  };

  const limpiar = () => {
    setSeleccionados([]);
    onChange([]);
    setOpen(false);
  };

  const limpiarDirecto = (e: React.MouseEvent) => {
    e.stopPropagation();
    limpiar();
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Estado
        {seleccionados.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionados.length}
            <button
              onClick={limpiarDirecto}
              className="ml-1 text-red-500 hover:text-red-700 font-bold text-lg leading-none hover:bg-red-200 rounded-full px-1"
              title="Limpiar filtro"
            >
              ×
            </button>
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Selecciona Estados</DialogTitle>
          </DialogHeader>

          <div className="space-y-2 mt-2 max-h-60 overflow-y-auto">
            {estadosDisponibles.map((estado) => (
              <label
                key={estado.value}
                className="flex items-center gap-2 hover:bg-gray-50 p-1 rounded cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={seleccionados.includes(estado.value)}
                  onChange={() => toggleEstado(estado.value)}
                  className="rounded"
                />
                <span>{estado.label}</span>
              </label>
            ))}
          </div>

          <DialogFooter className="gap-2">
            <Button onClick={aplicar}>
              Aplicar {seleccionados.length > 0 && `(${seleccionados.length})`}
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
