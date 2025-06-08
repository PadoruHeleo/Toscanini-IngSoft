import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Plus, Edit, Trash2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface Pieza {
  pieza_id: number;
  pieza_nombre?: string;
  pieza_marca?: string;
  pieza_desc?: string;
  pieza_precio?: number;
  created_at?: string;
}

interface FormData {
  pieza_nombre: string;
  pieza_marca: string;
  pieza_desc: string;
  pieza_precio: string;
}

export default function PiezasView() {
  const [piezas, setPiezas] = useState<Pieza[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [editingPieza, setEditingPieza] = useState<Pieza | null>(null);
  const [formData, setFormData] = useState<FormData>({
    pieza_nombre: "",
    pieza_marca: "",
    pieza_desc: "",
    pieza_precio: "",
  });
  const [errors, setErrors] = useState<Partial<FormData>>({});

  const loadPiezas = async () => {
    setLoading(true);
    try {
      const data = await invoke<Pieza[]>("get_piezas");
      setPiezas(data);
    } catch (e) {
      // Manejo de error
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadPiezas();
  }, []);

  const handleOpenForm = (pieza?: Pieza) => {
    setEditingPieza(pieza || null);
    setFormData(
      pieza
        ? {
            pieza_nombre: pieza.pieza_nombre || "",
            pieza_marca: pieza.pieza_marca || "",
            pieza_desc: pieza.pieza_desc || "",
            pieza_precio: pieza.pieza_precio?.toString() || "",
          }
        : {
            pieza_nombre: "",
            pieza_marca: "",
            pieza_desc: "",
            pieza_precio: "",
          }
    );
    setErrors({});
    setShowForm(true);
  };

  const handleCloseForm = () => {
    setShowForm(false);
    setEditingPieza(null);
    setFormData({
      pieza_nombre: "",
      pieza_marca: "",
      pieza_desc: "",
      pieza_precio: "",
    });
    setErrors({});
  };

  const validate = () => {
    const newErrors: Partial<FormData> = {};
    if (!formData.pieza_nombre.trim())
      newErrors.pieza_nombre = "El nombre es requerido";
    if (formData.pieza_precio && isNaN(Number(formData.pieza_precio)))
      newErrors.pieza_precio = "Debe ser un número";
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;
    try {
      if (editingPieza) {
        await invoke("update_pieza", {
          piezaId: editingPieza.pieza_id,
          request: {
            pieza_nombre: formData.pieza_nombre,
            pieza_marca: formData.pieza_marca,
            pieza_desc: formData.pieza_desc,
            pieza_precio: formData.pieza_precio
              ? Number(formData.pieza_precio)
              : null,
          },
        });
      } else {
        await invoke("create_pieza", {
          request: {
            pieza_nombre: formData.pieza_nombre,
            pieza_marca: formData.pieza_marca,
            pieza_desc: formData.pieza_desc,
            pieza_precio: formData.pieza_precio
              ? Number(formData.pieza_precio)
              : null,
          },
        });
      }
      loadPiezas();
      handleCloseForm();
    } catch (e) {
      // Manejo de error
    }
  };

  const handleDelete = async (pieza: Pieza) => {
    if (!window.confirm(`¿Eliminar la pieza "${pieza.pieza_nombre}"?`)) return;
    try {
      await invoke("delete_pieza", { piezaId: pieza.pieza_id });
      loadPiezas();
    } catch (e) {
      // Manejo de error
    }
  };

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Administrar Piezas</h2>
        <Button onClick={() => handleOpenForm()}>
          <Plus className="h-4 w-4 mr-2" /> Agregar Pieza
        </Button>
      </div>
      <div className="rounded-md border bg-white">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Nombre</TableHead>
              <TableHead>Marca</TableHead>
              <TableHead>Descripción</TableHead>
              <TableHead>Precio</TableHead>
              <TableHead>Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loading ? (
              <TableRow>
                <TableCell
                  colSpan={5}
                  className="text-center py-8 text-gray-500"
                >
                  Cargando piezas...
                </TableCell>
              </TableRow>
            ) : piezas.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={5}
                  className="text-center py-8 text-gray-500"
                >
                  No hay piezas registradas
                </TableCell>
              </TableRow>
            ) : (
              piezas.map((pieza) => (
                <TableRow key={pieza.pieza_id}>
                  <TableCell>{pieza.pieza_nombre}</TableCell>
                  <TableCell>{pieza.pieza_marca}</TableCell>
                  <TableCell>{pieza.pieza_desc}</TableCell>
                  <TableCell>
                    {pieza.pieza_precio != null
                      ? `$${pieza.pieza_precio}`
                      : "-"}
                  </TableCell>
                  <TableCell>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleOpenForm(pieza)}
                      title="Editar"
                    >
                      <Edit className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleDelete(pieza)}
                      title="Eliminar"
                    >
                      <Trash2 className="h-4 w-4 text-red-500" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
      <Dialog open={showForm} onOpenChange={setShowForm}>
        <DialogContent style={{ maxWidth: 400 }}>
          <DialogHeader>
            <DialogTitle>
              {editingPieza ? "Editar Pieza" : "Agregar Pieza"}
            </DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Label htmlFor="pieza_nombre">Nombre *</Label>
              <Input
                id="pieza_nombre"
                value={formData.pieza_nombre}
                onChange={(e) =>
                  setFormData({ ...formData, pieza_nombre: e.target.value })
                }
                className={errors.pieza_nombre ? "border-red-500" : ""}
              />
              {errors.pieza_nombre && (
                <p className="text-sm text-red-500">{errors.pieza_nombre}</p>
              )}
            </div>
            <div>
              <Label htmlFor="pieza_marca">Marca</Label>
              <Input
                id="pieza_marca"
                value={formData.pieza_marca}
                onChange={(e) =>
                  setFormData({ ...formData, pieza_marca: e.target.value })
                }
              />
            </div>
            <div>
              <Label htmlFor="pieza_desc">Descripción</Label>
              <Input
                id="pieza_desc"
                value={formData.pieza_desc}
                onChange={(e) =>
                  setFormData({ ...formData, pieza_desc: e.target.value })
                }
              />
            </div>
            <div>
              <Label htmlFor="pieza_precio">Precio</Label>
              <Input
                id="pieza_precio"
                type="number"
                min="0"
                value={formData.pieza_precio}
                onChange={(e) =>
                  setFormData({ ...formData, pieza_precio: e.target.value })
                }
                className={errors.pieza_precio ? "border-red-500" : ""}
              />
              {errors.pieza_precio && (
                <p className="text-sm text-red-500">{errors.pieza_precio}</p>
              )}
            </div>
            <DialogFooter className="gap-2">
              <Button type="button" variant="outline" onClick={handleCloseForm}>
                Cancelar
              </Button>
              <Button type="submit">
                {editingPieza ? "Actualizar" : "Crear"}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}

export { PiezasView };
