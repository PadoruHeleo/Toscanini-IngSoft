import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/core";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";

interface Props {
  resetKey?: number;
  onChange: (ciudades: string[] | null) => void;
}

export function FiltrarClientesPorCiudad({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [ciudadesDisponibles, setCiudadesDisponibles] = useState<string[]>([]);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [busqueda, setBusqueda] = useState("");

  // Cargar ciudades al abrir
  useEffect(() => {
    const fetchCiudades = async () => {
      try {
        const lista = await invoke<string[]>("get_ciudades_clientes");
        setCiudadesDisponibles(lista);
      } catch (err) {
        console.error("❌ Error cargando ciudades:", err);
      }
    };
    fetchCiudades();
  }, [resetKey]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
    setBusqueda("");
  }, [resetKey]);

  const toggleSeleccion = (ciudad: string) => {
    setSeleccionados((prev) =>
      prev.includes(ciudad)
        ? prev.filter((c) => c !== ciudad)
        : [...prev, ciudad]
    );
  };

  const limpiar = () => {
    setSeleccionados([]);
    setBusqueda("");
    onChange(null);
  };

  const aplicarFiltro = () => {
    onChange(seleccionados.length > 0 ? seleccionados : null);
    setOpen(false);
  };

  const ciudadesFiltradas = ciudadesDisponibles.filter((c) =>
    c.toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por dirección
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
            <DialogTitle>Seleccione que dirección quiere ver</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="Buscar ciudad..."
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            className="mb-2"
          />

          <div className="max-h-64 overflow-y-auto space-y-2">
            {ciudadesFiltradas.map((ciudad) => (
              <div key={ciudad} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionados.includes(ciudad)}
                  onCheckedChange={() => toggleSeleccion(ciudad)}
                />
                <span>{ciudad}</span>
              </div>
            ))}
          </div>

          <DialogFooter>
            <Button onClick={aplicarFiltro}>
              Aplicar {seleccionados.length > 0 && `(${seleccionados.length})`}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
