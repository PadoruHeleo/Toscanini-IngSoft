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
import { Input } from "@/components/ui/input";
import { Search, Edit } from "lucide-react";
import { EquipoFormDialog } from "@/components/views/EquipoFormDialog";
import { EquipoHistorialDialog } from "@/components/views/EquipoHistorialDialog";

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
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingEquipo, setEditingEquipo] = useState<Equipo | null>(null);
  const [historialEquipo, setHistorialEquipo] = useState<Equipo | null>(null);
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

  const filteredEquipos = equipos.filter((equipo) => {
    if (!searchTerm.trim()) return true;
    const searchLower = searchTerm.toLowerCase();
    return (
      equipo.equipo_marca?.toLowerCase().includes(searchLower) ||
      equipo.equipo_modelo?.toLowerCase().includes(searchLower) ||
      equipo.numero_serie?.toLowerCase().includes(searchLower) ||
      equipo.equipo_tipo?.toLowerCase().includes(searchLower) ||
      equipo.cliente_nombre?.toLowerCase().includes(searchLower) ||
      equipo.equipo_ubicacion?.toLowerCase().includes(searchLower)
    );
  });
  const handleEquipoAdded = () => {
    loadEquipos(); // Recargar la lista
    setShowAddForm(false);
    setEditingEquipo(null); // Clear editing state
  };

  const handleEditEquipo = (equipo: Equipo) => {
    setEditingEquipo(equipo);
    setShowAddForm(true); // Use the same form for editing
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
      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Buscar equipos..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
          />
        </div>
        {searchTerm && (
          <Button variant="outline" onClick={() => setSearchTerm("")}>
            Limpiar
          </Button>
        )}
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[120px]">Marca</TableHead>
              <TableHead>Modelo</TableHead>
              <TableHead>Número de Serie</TableHead>
              <TableHead>Tipo</TableHead>
              <TableHead>Cliente</TableHead>
              <TableHead>Ubicación</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredEquipos.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron equipos"
                    : "No hay equipos registrados"}
                </TableCell>
              </TableRow>
            ) : (
              filteredEquipos.map((equipo) => (
                <TableRow key={equipo.equipo_id}>
                  <TableCell className="font-medium">
                    {equipo.equipo_marca || "N/A"}
                  </TableCell>
                  <TableCell>{equipo.equipo_modelo || "N/A"}</TableCell>
                  <TableCell>{equipo.numero_serie || "N/A"}</TableCell>
                  <TableCell>{equipo.equipo_tipo || "N/A"}</TableCell>
                  <TableCell>
                    {equipo.cliente_nombre || "Sin cliente"}
                  </TableCell>
                  <TableCell>{equipo.equipo_ubicacion || "N/A"}</TableCell>{" "}
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditEquipo(equipo)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar equipo"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => setHistorialEquipo(equipo)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Ver historial del equipo"
                      >
                        Ver Historial Del Equipo
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
      {/* Total de equipos */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {filteredEquipos.length} equipo
        {filteredEquipos.length !== 1 ? "s" : ""}
      </div>{" "}
      <EquipoFormDialog
        open={showAddForm || editingEquipo !== null}
        onOpenChange={(open) => {
          if (!open) {
            setShowAddForm(false);
            setEditingEquipo(null);
          }
        }}
        onEquipoAdded={handleEquipoAdded}
        equipo={editingEquipo || undefined}
        isEditing={editingEquipo !== null}
      />
      <EquipoHistorialDialog
        open={historialEquipo !== null}
        onOpenChange={(open) => !open && setHistorialEquipo(null)}
        equipo={historialEquipo}
      />
    </div>
  );
}

