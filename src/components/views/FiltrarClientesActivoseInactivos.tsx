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
  onChange: (estados: boolean[] | null) => void;
}

export function FiltrarClienteActivoeInactivos({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionados, setSeleccionados] = useState<boolean[]>([]);

  // Opciones disponibles
  const opcionesEstado = [
    { valor: true, etiqueta: "Clientes Activos" },
    { valor: false, etiqueta: "Clientes Inactivos" },
  ];

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
  }, [resetKey]);

  const toggleSeleccion = (estado: boolean) => {
    setSeleccionados((prev) =>
      prev.includes(estado)
        ? prev.filter((s) => s !== estado)
        : [...prev, estado]
    );
  };

  const limpiar = () => {
    setSeleccionados([]);
    onChange(null);
  };

  const aplicarFiltro = () => {
    onChange(seleccionados.length > 0 ? seleccionados : null);
    setOpen(false);
  };

  // Función para obtener el texto del botón
  const obtenerTextoBoton = () => {
    if (seleccionados.length === 0) {
      return "Activos/Inactivos";
    }

    if (seleccionados.length === 2) {
      return "Todos los Estados";
    }

    if (seleccionados.includes(true)) {
      return "Solo Activos";
    }

    return "Solo Inactivos";
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        {obtenerTextoBoton()}
        {seleccionados.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionados.length}
            <button
              onClick={(e) => {
                e.stopPropagation(); // evita cerrar el diálogo
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
            <DialogTitle>Filtrar por Estado del Cliente</DialogTitle>
          </DialogHeader>

          <div className="space-y-4 py-4">
            {opcionesEstado.map((opcion) => (
              <div
                key={opcion.valor.toString()}
                className="flex items-center space-x-2"
              >
                <Checkbox
                  checked={seleccionados.includes(opcion.valor)}
                  onCheckedChange={() => toggleSeleccion(opcion.valor)}
                />
                <span className="text-sm font-medium">{opcion.etiqueta}</span>
              </div>
            ))}
          </div>

          <DialogFooter className="flex gap-2">
            <Button onClick={aplicarFiltro}>
              Aplicar
              {seleccionados.length > 0 && ` (${seleccionados.length})`}
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
