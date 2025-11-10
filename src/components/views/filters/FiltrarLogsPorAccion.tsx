import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Checkbox } from "@/components/ui/checkbox";

interface Props {
  resetKey?: number;
  onChange: (acciones: string[] | null) => void;
}

const opcionesAccion = [
  { valor: "CREATE", etiqueta: "Crear" },
  { valor: "UPDATE", etiqueta: "Actualizar" },
  { valor: "DELETE", etiqueta: "Eliminar" },
  { valor: "LOGIN", etiqueta: "Inicio de sesión" },
  { valor: "LOGOUT", etiqueta: "Cierre de sesión" },
  { valor: "LOGIN_FAILED", etiqueta: "Inicio de sesión fallido" },
];

export function FiltrarLogsPorAccion({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionadas([]);
  }, [resetKey]);

  const toggleSeleccion = (valor: string) => {
    setSeleccionadas((prev) =>
      prev.includes(valor) ? prev.filter((v) => v !== valor) : [...prev, valor]
    );
  };

  const aplicarFiltro = () => {
    onChange(seleccionadas.length > 0 ? seleccionadas : null);
    setOpen(false);
  };

  const limpiar = () => {
    setSeleccionadas([]);
    onChange(null);
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por acción
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
            <DialogTitle>Seleccione las acciones</DialogTitle>
          </DialogHeader>

          <div className="max-h-64 overflow-y-auto space-y-2">
            {opcionesAccion.map((op) => (
              <div key={op.valor} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionadas.includes(op.valor)}
                  onCheckedChange={() => toggleSeleccion(op.valor)}
                />
                <label
                  className="cursor-pointer"
                  onClick={() => toggleSeleccion(op.valor)}
                >
                  {op.etiqueta}
                </label>
              </div>
            ))}
          </div>

          <DialogFooter className="flex gap-2">
            <Button
              onClick={aplicarFiltro}
              disabled={seleccionadas.length === 0}
            >
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
