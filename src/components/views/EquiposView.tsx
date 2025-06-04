import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ViewTitle } from "@/components/ViewTitle";
import { Button } from "@/components/ui/button";
import { EquipoFormDialog } from "@/components/views/EquipoFormDialog";

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_precio?: number;
  equipo_ubicacion?: string;
  cliente_id?: number;
  cliente_nombre?: string;
  cliente_correo?: string;
  created_by?: number;
  created_at?: string;
}

export function EquiposView() {
  const [equipos, setEquipos] = useState<Equipo[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);

  const loadEquipos = async () => {
    try {
      setLoading(true);
      const equiposData = await invoke<Equipo[]>("get_equipos_with_cliente");
      setEquipos(equiposData);
    } catch (error) {
      console.error("Error cargando equipos:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadEquipos();
  }, []);

  const handleEquipoAdded = () => {
    loadEquipos(); // Recargar la lista
    setShowAddForm(false);
  };

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando equipos...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>Agregar Equipo</Button>
      </div>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[120px]">Marca</TableHead>
            <TableHead>Modelo</TableHead>
            <TableHead>Número de Serie</TableHead>
            <TableHead>Tipo</TableHead>
            <TableHead>Cliente</TableHead>
            <TableHead>Ubicación</TableHead>
            <TableHead>Precio</TableHead>
            <TableHead>Acciones</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {equipos.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center py-8 text-gray-500">
                No hay equipos registrados
              </TableCell>
            </TableRow>
          ) : (
            equipos.map((equipo) => (
              <TableRow key={equipo.equipo_id}>
                <TableCell className="font-medium">
                  {equipo.equipo_marca || "N/A"}
                </TableCell>
                <TableCell>{equipo.equipo_modelo || "N/A"}</TableCell>
                <TableCell>{equipo.numero_serie || "N/A"}</TableCell>
                <TableCell>{equipo.equipo_tipo || "N/A"}</TableCell>
                <TableCell>{equipo.cliente_nombre || "Sin cliente"}</TableCell>
                <TableCell>{equipo.equipo_ubicacion || "N/A"}</TableCell>
                <TableCell>
                  {equipo.equipo_precio
                    ? `$${equipo.equipo_precio.toLocaleString()}`
                    : "N/A"}
                </TableCell>
                <TableCell>
                  <div className="flex gap-2">
                    <Button variant="outline" size="sm">
                      Editar
                    </Button>
                    <Button variant="outline" size="sm">
                      Ver Detalles
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>

      <EquipoFormDialog
        open={showAddForm}
        onOpenChange={setShowAddForm}
        onEquipoAdded={handleEquipoAdded}
      />
    </div>
  );
}
