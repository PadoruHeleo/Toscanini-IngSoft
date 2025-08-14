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
  onChange: (fechas: {
    fechaInicio: string | null;
    fechaFin: string | null;
  }) => void;
  resetKey?: number;
}

export function FiltrarOrdenesPorFecha({ onChange, resetKey }: Props) {
  const [open, setOpen] = useState(false);
  const [fechaInicio, setFechaInicio] = useState("");
  const [fechaFin, setFechaFin] = useState("");

  const aplicar = () => {
    onChange({
      fechaInicio: fechaInicio || null,
      fechaFin: fechaFin || null,
    });
    setOpen(false);
  };

  const limpiar = () => {
    setFechaInicio("");
    setFechaFin("");
    onChange({
      fechaInicio: null,
      fechaFin: null,
    });
    setOpen(false);
  };

  useEffect(() => {
    setFechaInicio("");
    setFechaFin("");
  }, [resetKey]);

  const validarFechas = () => {
    if (fechaInicio && fechaFin) {
      return new Date(fechaInicio) <= new Date(fechaFin);
    }
    return true;
  };

  const hayFechas = Boolean(fechaInicio || fechaFin);

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Fechas
        {hayFechas && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              limpiar();
            }}
            className="ml-2 text-red-500 font-bold text-lg leading-none"
          >
            ×
          </button>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Seleccionar Intervalo de Fechas</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <label
                htmlFor="fechaInicio"
                className="block text-sm font-medium mb-1"
              >
                Fecha Inicio:
              </label>
              <input
                id="fechaInicio"
                type="date"
                value={fechaInicio}
                onChange={(e) => setFechaInicio(e.target.value)}
                className="border p-2 w-full rounded"
                max={fechaFin || undefined}
              />
            </div>
            <div>
              <label
                htmlFor="fechaFin"
                className="block text-sm font-medium mb-1"
              >
                Fecha Fin:
              </label>
              <input
                id="fechaFin"
                type="date"
                value={fechaFin}
                onChange={(e) => setFechaFin(e.target.value)}
                className="border p-2 w-full rounded"
                min={fechaInicio || undefined}
              />
            </div>
            {!validarFechas() && (
              <p className="text-red-500 text-sm">
                La fecha de inicio debe ser anterior o igual a la fecha de fin
              </p>
            )}
          </div>
          <DialogFooter>
            <Button onClick={aplicar} disabled={!validarFechas()}>
              Aplicar
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
