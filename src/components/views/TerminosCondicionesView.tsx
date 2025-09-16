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
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Search,
  Plus,
  Edit,
  Eye,
  Trash2,
  CheckCircle,
  Circle,
} from "lucide-react";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";

interface TerminoCondicion {
  termino_id: number;
  termino_nombre: string;
  termino_descripcion: string;
  is_active?: boolean;
  tipo_referencia: string;
  is_default?: boolean;
  created_at?: string;
  updated_at?: string;
}

interface TerminoInforme {
  termino_id: number;
  informe_id: number;
  aplicado?: boolean;
  created_at?: string;
  termino_nombre?: string;
  termino_descripcion?: string;
}

interface TerminoCotizacion {
  termino_id: number;
  cotizacion_id: number;
  aplicado?: boolean;
  created_at?: string;
  termino_nombre?: string;
  termino_descripcion?: string;
}

interface TerminoInformeRequest {
  termino_id: number;
  aplicado?: boolean;
}

interface TerminoCotizacionRequest {
  termino_id: number;
  aplicado?: boolean;
}

interface FormData {
  termino_nombre: string;
  termino_descripcion: string;
  tipo_referencia: string;
  is_default: boolean;
}

export function TerminosCondicionesView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [terminos, setTerminos] = useState<TerminoCondicion[]>([]);
  const [filteredTerminos, setFilteredTerminos] = useState<TerminoCondicion[]>(
    []
  );
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingTermino, setEditingTermino] = useState<TerminoCondicion | null>(
    null
  );
  const [deleteConfirm, setDeleteConfirm] = useState<TerminoCondicion | null>(
    null
  );
  const [filterType, setFilterType] = useState<string>("todos");
  const [filterStatus, setFilterStatus] = useState<string>("activos");
  const [formData, setFormData] = useState<FormData>({
    termino_nombre: "",
    termino_descripcion: "",
    tipo_referencia: "",
    is_default: false,
  });
  const [errors, setErrors] = useState<Partial<FormData>>({});

  useEffect(() => {
    loadTerminos();
  }, []);

  useEffect(() => {
    filterTerminos();
  }, [terminos, searchTerm, filterType, filterStatus]);

  const loadTerminos = async () => {
    try {
      setLoading(true);
      const data = await invoke<TerminoCondicion[]>("get_terminos_condiciones");
      setTerminos(data);
    } catch (error) {
      showError("Error al cargar términos y condiciones");
      console.error("Error loading terminos:", error);
    } finally {
      setLoading(false);
    }
  };

  const filterTerminos = () => {
    let filtered = terminos;

    // Filtro por texto
    if (searchTerm) {
      filtered = filtered.filter(
        (termino) =>
          termino.termino_nombre
            .toLowerCase()
            .includes(searchTerm.toLowerCase()) ||
          termino.termino_descripcion
            .toLowerCase()
            .includes(searchTerm.toLowerCase())
      );
    }

    // Filtro por tipo
    if (filterType !== "todos") {
      filtered = filtered.filter(
        (termino) => termino.tipo_referencia === filterType
      );
    }

    // Filtro por estado
    if (filterStatus !== "todos") {
      if (filterStatus === "activos") {
        filtered = filtered.filter((termino) => termino.is_active === true);
      } else if (filterStatus === "inactivos") {
        filtered = filtered.filter((termino) => termino.is_active === false);
      } else if (filterStatus === "por_defecto") {
        filtered = filtered.filter((termino) => termino.is_default === true);
      }
    }

    setFilteredTerminos(filtered);
  };

  const validateForm = (): boolean => {
    const newErrors: Partial<FormData> = {};

    if (!formData.termino_nombre.trim()) {
      newErrors.termino_nombre = "El nombre es requerido";
    }

    if (!formData.termino_descripcion.trim()) {
      newErrors.termino_descripcion = "La descripción es requerida";
    }

    if (!formData.tipo_referencia) {
      newErrors.tipo_referencia = "El tipo de referencia es requerido";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) return;

    try {
      if (editingTermino) {
        await invoke("update_termino_condicion", {
          terminoId: editingTermino.termino_id,
          request: {
            termino_nombre: formData.termino_nombre,
            termino_descripcion: formData.termino_descripcion,
            tipo_referencia: formData.tipo_referencia,
            is_default: formData.is_default,
          },
          updatedBy: user?.usuario_id,
        });
        success("Término y condición actualizado correctamente");
      } else {
        await invoke("create_termino_condicion", {
          request: formData,
          createdBy: user?.usuario_id,
        });
        success("Término y condición creado correctamente");
      }

      resetForm();
      loadTerminos();
    } catch (error) {
      showError("Error al guardar término y condición");
      console.error("Error saving termino:", error);
    }
  };

  const handleEdit = (termino: TerminoCondicion) => {
    setEditingTermino(termino);
    setFormData({
      termino_nombre: termino.termino_nombre,
      termino_descripcion: termino.termino_descripcion,
      tipo_referencia: termino.tipo_referencia,
      is_default: termino.is_default || false,
    });
    setShowAddForm(true);
  };

  const handleDelete = async () => {
    if (!deleteConfirm || !user) return;

    try {
      console.log(
        "Deleting termino with ID:",
        deleteConfirm.termino_id,
        "by user:",
        user.usuario_id
      );
      await invoke("delete_termino_condicion", {
        terminoId: deleteConfirm.termino_id,
        deletedBy: user.usuario_id,
      });
      success("Término y condición desactivado correctamente");
      setDeleteConfirm(null);
      loadTerminos();
    } catch (error) {
      showError("Error al desactivar término y condición");
      console.error("Error deleting termino:", error);
    }
  };

  const handleReactivate = async (termino: TerminoCondicion) => {
    if (!user) return;

    try {
      await invoke("reactivate_termino_condicion", {
        terminoId: termino.termino_id,
        reactivatedBy: user.usuario_id,
      });
      success("Término y condición reactivado correctamente");
      loadTerminos();
    } catch (error) {
      showError("Error al reactivar término y condición");
      console.error("Error reactivating termino:", error);
    }
  };

  // ==================== MÉTODOS ADICIONALES ====================
  // Estos métodos corresponden a todos los comandos disponibles en terminos_condiciones.rs

  /** Obtener solo términos y condiciones activos */
  const getTerminosActivos = async (): Promise<TerminoCondicion[]> => {
    try {
      const data = await invoke<TerminoCondicion[]>(
        "get_terminos_condiciones_activos"
      );
      return data;
    } catch (error) {
      showError("Error al cargar términos activos");
      console.error("Error loading active terminos:", error);
      return [];
    }
  };

  /** Obtener términos y condiciones filtrados por tipo (informe, cotizacion, ambos) */
  const getTerminosByTipo = async (
    tipo: string
  ): Promise<TerminoCondicion[]> => {
    try {
      const data = await invoke<TerminoCondicion[]>(
        "get_terminos_condiciones_by_tipo",
        { tipo }
      );
      return data;
    } catch (error) {
      showError(`Error al cargar términos del tipo ${tipo}`);
      console.error("Error loading terminos by tipo:", error);
      return [];
    }
  };

  /** Obtener términos y condiciones marcados como por defecto para un tipo específico */
  const getTerminosDefault = async (
    tipo: string
  ): Promise<TerminoCondicion[]> => {
    try {
      const data = await invoke<TerminoCondicion[]>(
        "get_terminos_condiciones_default",
        { tipo }
      );
      return data;
    } catch (error) {
      showError(`Error al cargar términos por defecto del tipo ${tipo}`);
      console.error("Error loading default terminos:", error);
      return [];
    }
  };

  /** Obtener un término y condición específico por su ID */
  const getTerminoById = async (
    termino_id: number
  ): Promise<TerminoCondicion | null> => {
    try {
      const data = await invoke<TerminoCondicion | null>(
        "get_termino_condicion_by_id",
        { terminoId: termino_id }
      );
      return data;
    } catch (error) {
      showError("Error al cargar término por ID");
      console.error("Error loading termino by id:", error);
      return null;
    }
  };

  /** Obtener términos aplicados a un informe específico */
  const getTerminosByInforme = async (
    informe_id: number
  ): Promise<TerminoInforme[]> => {
    try {
      const data = await invoke<TerminoInforme[]>("get_terminos_by_informe", {
        informeId: informe_id,
      });
      return data;
    } catch (error) {
      showError("Error al cargar términos del informe");
      console.error("Error loading terminos by informe:", error);
      return [];
    }
  };

  /** Obtener términos aplicados a una cotización específica */
  const getTerminosByCotizacion = async (
    cotizacion_id: number
  ): Promise<TerminoCotizacion[]> => {
    try {
      const data = await invoke<TerminoCotizacion[]>(
        "get_terminos_by_cotizacion",
        { cotizacionId: cotizacion_id }
      );
      return data;
    } catch (error) {
      showError("Error al cargar términos de la cotización");
      console.error("Error loading terminos by cotizacion:", error);
      return [];
    }
  };

  /** Aplicar términos específicos a un informe */
  const applyTerminosToInforme = async (
    informe_id: number,
    terminos: TerminoInformeRequest[]
  ): Promise<boolean> => {
    if (!user) return false;

    try {
      await invoke("apply_terminos_to_informe", {
        informeId: informe_id,
        terminos,
        appliedBy: user.usuario_id,
      });
      success("Términos aplicados al informe correctamente");
      return true;
    } catch (error) {
      showError("Error al aplicar términos al informe");
      console.error("Error applying terminos to informe:", error);
      return false;
    }
  };

  /** Aplicar términos específicos a una cotización */
  const applyTerminosToCotizacion = async (
    cotizacion_id: number,
    terminos: TerminoCotizacionRequest[]
  ): Promise<boolean> => {
    if (!user) return false;

    try {
      await invoke("apply_terminos_to_cotizacion", {
        cotizacionId: cotizacion_id,
        terminos,
        appliedBy: user.usuario_id,
      });
      success("Términos aplicados a la cotización correctamente");
      return true;
    } catch (error) {
      showError("Error al aplicar términos a la cotización");
      console.error("Error applying terminos to cotizacion:", error);
      return false;
    }
  };

  /** Aplicar todos los términos marcados como por defecto a un informe */
  const applyDefaultTerminosToInforme = async (
    informe_id: number
  ): Promise<boolean> => {
    if (!user) return false;

    try {
      await invoke("apply_default_terminos_to_informe", {
        informeId: informe_id,
        appliedBy: user.usuario_id,
      });
      success("Términos por defecto aplicados al informe correctamente");
      return true;
    } catch (error) {
      showError("Error al aplicar términos por defecto al informe");
      console.error("Error applying default terminos to informe:", error);
      return false;
    }
  };

  /** Aplicar todos los términos marcados como por defecto a una cotización */
  const applyDefaultTerminosToCotizacion = async (
    cotizacion_id: number
  ): Promise<boolean> => {
    if (!user) return false;

    try {
      await invoke("apply_default_terminos_to_cotizacion", {
        cotizacionId: cotizacion_id,
        appliedBy: user.usuario_id,
      });
      success("Términos por defecto aplicados a la cotización correctamente");
      return true;
    } catch (error) {
      showError("Error al aplicar términos por defecto a la cotización");
      console.error("Error applying default terminos to cotizacion:", error);
      return false;
    }
  };

  /** Cambiar el estado por defecto de un término y condición */
  const toggleTerminoDefault = async (
    termino: TerminoCondicion,
    is_default: boolean
  ): Promise<boolean> => {
    if (!user) return false;

    try {
      await invoke("toggle_termino_default", {
        terminoId: termino.termino_id,
        isDefault: is_default,
        updatedBy: user.usuario_id,
      });
      success(
        `Término ${is_default ? "marcado" : "desmarcado"} como por defecto`
      );
      loadTerminos();
      return true;
    } catch (error) {
      showError("Error al cambiar estado por defecto");
      console.error("Error toggling default:", error);
      return false;
    }
  };

  // ==================== FIN MÉTODOS ADICIONALES ====================

  // Objeto con todas las funciones de términos y condiciones para uso externo
  const terminosCondicionesAPI = {
    // Métodos de consulta
    getAll: loadTerminos,
    getActivos: getTerminosActivos,
    getByTipo: getTerminosByTipo,
    getDefault: getTerminosDefault,
    getById: getTerminoById,

    // Métodos CRUD
    create: (request: FormData, created_by: number) =>
      invoke("create_termino_condicion", { request, createdBy: created_by }),
    update: (termino_id: number, request: any, updated_by: number) =>
      invoke("update_termino_condicion", {
        terminoId: termino_id,
        request,
        updatedBy: updated_by,
      }),
    delete: (termino_id: number, deleted_by: number) =>
      invoke("delete_termino_condicion", {
        terminoId: termino_id,
        deletedBy: deleted_by,
      }),
    reactivate: (termino_id: number, reactivated_by: number) =>
      invoke("reactivate_termino_condicion", {
        terminoId: termino_id,
        reactivatedBy: reactivated_by,
      }),
    toggleDefault: toggleTerminoDefault,

    // Métodos de aplicación a informes
    getTerminosByInforme,
    applyTerminosToInforme,
    applyDefaultTerminosToInforme,

    // Métodos de aplicación a cotizaciones
    getTerminosByCotizacion,
    applyTerminosToCotizacion,
    applyDefaultTerminosToCotizacion,
  };

  // Hacer disponible las funciones para uso externo (opcional)
  // Se puede acceder como: TerminosCondicionesView.api
  (TerminosCondicionesView as any).api = terminosCondicionesAPI;

  const resetForm = () => {
    setFormData({
      termino_nombre: "",
      termino_descripcion: "",
      tipo_referencia: "",
      is_default: false,
    });
    setErrors({});
    setShowAddForm(false);
    setEditingTermino(null);
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case "informe":
        return "bg-blue-100 text-blue-800";
      case "cotizacion":
        return "bg-green-100 text-green-800";
      case "ambos":
        return "bg-purple-100 text-purple-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const getTypeLabel = (type: string) => {
    switch (type) {
      case "informe":
        return "Informe";
      case "cotizacion":
        return "Cotización";
      case "ambos":
        return "Ambos";
      default:
        return type;
    }
  };

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">Términos y Condiciones</h1>
        <p className="text-gray-600 mt-1">
          Gestiona los términos y condiciones para informes y cotizaciones
        </p>
      </div>

      {/* Controles y Filtros */}
      <div className="mb-6 space-y-4">
        <div className="flex gap-4 items-center">
          <Button onClick={() => setShowAddForm(true)}>
            <Plus className="w-4 h-4 mr-2" />
            Nuevo Término
          </Button>

          <div className="flex-1 max-w-sm">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
              <Input
                placeholder="Buscar términos..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10"
              />
            </div>
          </div>

          <Select value={filterType} onValueChange={setFilterType}>
            <SelectTrigger className="w-40">
              <SelectValue placeholder="Filtrar por tipo" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="todos">Todos los tipos</SelectItem>
              <SelectItem value="informe">Informe</SelectItem>
              <SelectItem value="cotizacion">Cotización</SelectItem>
              <SelectItem value="ambos">Ambos</SelectItem>
            </SelectContent>
          </Select>

          <Select value={filterStatus} onValueChange={setFilterStatus}>
            <SelectTrigger className="w-40">
              <SelectValue placeholder="Filtrar por estado" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="todos">Todos</SelectItem>
              <SelectItem value="activos">Activos</SelectItem>
              <SelectItem value="inactivos">Inactivos</SelectItem>
              <SelectItem value="por_defecto">Por defecto</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Tabla */}
      <div className="border rounded-lg">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Nombre</TableHead>
              <TableHead>Descripción</TableHead>
              <TableHead>Tipo</TableHead>
              <TableHead>Estado</TableHead>
              <TableHead>Por Defecto</TableHead>
              <TableHead>Fecha Creación</TableHead>
              <TableHead>Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loading ? (
              <TableRow>
                <TableCell colSpan={7} className="text-center py-8">
                  Cargando términos...
                </TableCell>
              </TableRow>
            ) : filteredTerminos.length === 0 ? (
              <TableRow>
                <TableCell colSpan={7} className="text-center py-8">
                  No se encontraron términos y condiciones
                </TableCell>
              </TableRow>
            ) : (
              filteredTerminos.map((termino) => (
                <TableRow key={termino.termino_id}>
                  <TableCell className="font-medium">
                    {termino.termino_nombre}
                  </TableCell>
                  <TableCell className="max-w-xs">
                    <div
                      className="truncate"
                      title={termino.termino_descripcion}
                    >
                      {termino.termino_descripcion}
                    </div>
                  </TableCell>
                  <TableCell>
                    <Badge className={getTypeColor(termino.tipo_referencia)}>
                      {getTypeLabel(termino.tipo_referencia)}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={termino.is_active ? "default" : "secondary"}
                    >
                      {termino.is_active ? "Activo" : "Inactivo"}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    {termino.is_default ? (
                      <CheckCircle className="w-4 h-4 text-green-500" />
                    ) : (
                      <Circle className="w-4 h-4 text-gray-400" />
                    )}
                  </TableCell>
                  <TableCell>
                    {termino.created_at
                      ? new Date(termino.created_at).toLocaleDateString()
                      : "-"}
                  </TableCell>
                  <TableCell>
                    <div className="flex gap-2">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleEdit(termino)}
                      >
                        <Edit className="w-4 h-4" />
                      </Button>

                      {termino.is_active ? (
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => setDeleteConfirm(termino)}
                        >
                          <Trash2 className="w-4 h-4 text-red-500" />
                        </Button>
                      ) : (
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleReactivate(termino)}
                        >
                          <Eye className="w-4 h-4 text-green-500" />
                        </Button>
                      )}
                    </div>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Formulario de Creación/Edición */}
      <Dialog open={showAddForm} onOpenChange={(open) => !open && resetForm()}>
        <DialogContent className="sm:max-w-[600px]">
          <form onSubmit={handleSubmit}>
            <DialogHeader>
              <DialogTitle>
                {editingTermino ? "Editar" : "Crear"} Término y Condición
              </DialogTitle>
            </DialogHeader>

            <div className="grid gap-4 py-4">
              <div className="space-y-2">
                <Label htmlFor="termino_nombre">Nombre</Label>
                <Input
                  id="termino_nombre"
                  value={formData.termino_nombre}
                  onChange={(e) =>
                    setFormData({ ...formData, termino_nombre: e.target.value })
                  }
                  placeholder="Ingrese el nombre del término"
                />
                {errors.termino_nombre && (
                  <p className="text-sm text-red-600">
                    {errors.termino_nombre}
                  </p>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="termino_descripcion">Descripción</Label>
                <Textarea
                  id="termino_descripcion"
                  value={formData.termino_descripcion}
                  onChange={(e) =>
                    setFormData({
                      ...formData,
                      termino_descripcion: e.target.value,
                    })
                  }
                  placeholder="Ingrese la descripción del término"
                  rows={4}
                />
                {errors.termino_descripcion && (
                  <p className="text-sm text-red-600">
                    {errors.termino_descripcion}
                  </p>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="tipo_referencia">Tipo de Referencia</Label>
                <Select
                  value={formData.tipo_referencia}
                  onValueChange={(value) =>
                    setFormData({ ...formData, tipo_referencia: value })
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Seleccionar tipo" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="informe">Informe</SelectItem>
                    <SelectItem value="cotizacion">Cotización</SelectItem>
                    <SelectItem value="ambos">Ambos</SelectItem>
                  </SelectContent>
                </Select>
                {errors.tipo_referencia && (
                  <p className="text-sm text-red-600">
                    {errors.tipo_referencia}
                  </p>
                )}
              </div>

              <div className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  id="is_default"
                  checked={formData.is_default}
                  onChange={(e) =>
                    setFormData({ ...formData, is_default: e.target.checked })
                  }
                  className="w-4 h-4"
                />
                <Label htmlFor="is_default">Aplicar por defecto</Label>
              </div>
            </div>

            <DialogFooter>
              <Button type="button" variant="outline" onClick={resetForm}>
                Cancelar
              </Button>
              <Button type="submit">
                {editingTermino ? "Actualizar" : "Crear"}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>

      {/* Dialog de Confirmación de Desactivación */}
      <Dialog
        open={!!deleteConfirm}
        onOpenChange={(open) => !open && setDeleteConfirm(null)}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Desactivar Término y Condición</DialogTitle>
            <DialogDescription>
              ¿Estás seguro de que quieres desactivar "
              {deleteConfirm?.termino_nombre}"? Esta acción se puede revertir
              después.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteConfirm(null)}>
              Cancelar
            </Button>
            <Button variant="destructive" onClick={handleDelete}>
              Desactivar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
