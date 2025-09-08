import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface Cliente {
  cliente_id: number;
  cliente_nombre?: string;
  cliente_rut?: string;
  cliente_correo?: string;
  cliente_telefono?: string;
}

interface Equipo {
  equipo_id: number;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  equipo_ubicacion?: string;
  created_at: string;
}

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string | null;
  orden_desc?: string | null;
  prioridad?: string | null;
  estado?: string | null;
  has_garantia?: boolean | null;
  equipo_id?: number | null;
  created_by?: number | null;
  cotizacion_id?: number | null;
  informe_id?: number | null;
  pre_informe?: string | null;
  created_at?: string | null;
  finished_at?: string | null;
}

interface Cotizacion {
  cotizacion_id: number;
  cliente_id: number;
  fecha: string | null;
  total: number | null;
  estado: string;
}

interface Informe {
  id: number;
  informe_codigo: string;
  diagnostico: string[];
  created_at: string;
  solucion_aplicada: string;
  tecnico_responsable: string;
  orden_codigo?: string;
}

function formatChileanDate(dateString?: string | null) {
  if (!dateString) return "";
  const date = new Date(dateString);
  return date.toLocaleString("es-CL", { timeZone: "America/Santiago" });
}

function capitalize(str: string) {
  if (!str) return "";
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function formatCurrency(amount?: number | null) {
  if (!amount) return "N/A";
  return new Intl.NumberFormat("es-CL", {
    style: "currency",
    currency: "CLP",
  }).format(amount);
}

export function ClienteHistorialDialog({
  open,
  onOpenChange,
  cliente,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  cliente: Cliente | null;
}) {
  const [equipos, setEquipos] = useState<Equipo[]>([]);
  const [ordenes, setOrdenes] = useState<OrdenTrabajo[]>([]);
  const [cotizaciones, setCotizaciones] = useState<Cotizacion[]>([]);
  const [informes, setInformes] = useState<Informe[]>([]);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState("equipos");

  // Estados para los sub-dialogs de detalles
  const [selectedInforme, setSelectedInforme] = useState<Informe | null>(null);
  const [openInformeDialog, setOpenInformeDialog] = useState(false);

  useEffect(() => {
    if (open && cliente) {
      loadClienteData();
    } else {
      setEquipos([]);
      setOrdenes([]);
      setCotizaciones([]);
      setInformes([]);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, cliente]);

  const loadClienteData = async () => {
    if (!cliente) return;
    setLoading(true);
    try {
      // Cargar equipos del cliente
      const equiposData = await invoke<Equipo[]>("get_equipos_by_cliente", {
        clienteId: cliente.cliente_id,
      }).catch(() => []);

      // Cargar órdenes de trabajo del cliente (nuevo comando)
      const ordenesData = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_by_cliente",
        { clienteId: cliente.cliente_id }
      ).catch(() => []);

      // Cargar cotizaciones del cliente
      const cotizacionesData = await invoke<Cotizacion[]>(
        "get_cotizaciones_cliente",
        { clienteId: cliente.cliente_id }
      ).catch(() => []);

      // Cargar informes del cliente
      const informesData = await invoke<Informe[]>("get_informes_by_cliente", {
        clienteId: cliente.cliente_id,
      }).catch(() => []);

      setEquipos(equiposData);
      setOrdenes(ordenesData);
      setCotizaciones(cotizacionesData);
      setInformes(informesData);
    } catch (error) {
      console.error("Error cargando datos del cliente:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleVerInforme = (informe: Informe) => {
    setSelectedInforme(informe);
    setOpenInformeDialog(true);
  };

  if (!cliente) return null;

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent style={{ minWidth: "1000px", maxHeight: "80vh" }}>
          <DialogHeader>
            <DialogTitle>Historial de Cliente</DialogTitle>
            <DialogDescription>
              Revisa todo el historial y actividad del cliente seleccionado.
            </DialogDescription>
          </DialogHeader>

          <div className="grid gap-4">
            {/* Información del cliente */}
            <div className="p-4 bg-gray-50 rounded-lg">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <strong>Nombre:</strong> {cliente.cliente_nombre || "N/A"}
                  <br />
                  <strong>RUT:</strong> {cliente.cliente_rut || "N/A"}
                </div>
                <div>
                  <strong>Correo:</strong> {cliente.cliente_correo || "N/A"}
                  <br />
                  <strong>Teléfono:</strong> {cliente.cliente_telefono || "N/A"}
                </div>
              </div>
            </div>

            {/* Tabs para diferentes secciones */}
            <Tabs value={activeTab} onValueChange={setActiveTab}>
              <TabsList className="grid w-full grid-cols-4">
                <TabsTrigger value="equipos">
                  Equipos ({equipos.length})
                </TabsTrigger>
                <TabsTrigger value="ordenes">
                  Órdenes ({ordenes.length})
                </TabsTrigger>
                <TabsTrigger value="cotizaciones">
                  Cotizaciones ({cotizaciones.length})
                </TabsTrigger>
                <TabsTrigger value="informes">
                  Informes ({informes.length})
                </TabsTrigger>
              </TabsList>

              {/* Tab de Equipos */}
              <TabsContent value="equipos">
                <Table>
                  <TableCaption>Equipos registrados del cliente</TableCaption>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Marca</TableHead>
                      <TableHead>Modelo</TableHead>
                      <TableHead>Número de Serie</TableHead>
                      <TableHead>Tipo</TableHead>
                      <TableHead>Ubicación</TableHead>
                      <TableHead>Fecha Registro</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {loading ? (
                      <TableRow>
                        <TableCell colSpan={6}>Cargando...</TableCell>
                      </TableRow>
                    ) : equipos.length === 0 ? (
                      <TableRow>
                        <TableCell colSpan={6} className="text-center">
                          No hay equipos registrados.
                        </TableCell>
                      </TableRow>
                    ) : (
                      equipos.map((equipo) => (
                        <TableRow key={equipo.equipo_id}>
                          <TableCell>{equipo.equipo_marca || "N/A"}</TableCell>
                          <TableCell>{equipo.equipo_modelo || "N/A"}</TableCell>
                          <TableCell>{equipo.numero_serie || "N/A"}</TableCell>
                          <TableCell>{equipo.equipo_tipo || "N/A"}</TableCell>
                          <TableCell>
                            {equipo.equipo_ubicacion || "N/A"}
                          </TableCell>
                          <TableCell>
                            {formatChileanDate(equipo.created_at)}
                          </TableCell>
                        </TableRow>
                      ))
                    )}
                  </TableBody>
                </Table>
              </TabsContent>

              {/* Tab de Órdenes */}
              <TabsContent value="ordenes">
                <Table>
                  <TableCaption>Órdenes de trabajo del cliente</TableCaption>
                  <TableHeader>
                    <TableRow>
                      <TableHead>ID</TableHead>
                      <TableHead>Código</TableHead>
                      <TableHead>Estado</TableHead>
                      <TableHead>Prioridad</TableHead>
                      <TableHead>Fecha Creación</TableHead>
                      <TableHead>Fecha Fin</TableHead>
                      <TableHead>Descripción</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {loading ? (
                      <TableRow>
                        <TableCell colSpan={7}>Cargando...</TableCell>
                      </TableRow>
                    ) : ordenes.length === 0 ? (
                      <TableRow>
                        <TableCell colSpan={7} className="text-center">
                          No hay órdenes de trabajo.
                        </TableCell>
                      </TableRow>
                    ) : (
                      ordenes.map((orden) => (
                        <TableRow key={orden.orden_id}>
                          <TableCell>{orden.orden_id}</TableCell>
                          <TableCell>{orden.orden_codigo || "N/A"}</TableCell>
                          <TableCell>
                            {capitalize(orden.estado ?? "")}
                          </TableCell>
                          <TableCell>{orden.prioridad || "N/A"}</TableCell>
                          <TableCell>
                            {orden.created_at
                              ? formatChileanDate(orden.created_at)
                              : "N/A"}
                          </TableCell>
                          <TableCell>
                            {orden.finished_at
                              ? formatChileanDate(orden.finished_at)
                              : "En proceso"}
                          </TableCell>
                          <TableCell>{orden.orden_desc || "N/A"}</TableCell>
                        </TableRow>
                      ))
                    )}
                  </TableBody>
                </Table>
              </TabsContent>

              {/* Tab de Cotizaciones */}
              <TabsContent value="cotizaciones">
                <Table>
                  <TableCaption>Cotizaciones del cliente</TableCaption>
                  <TableHeader>
                    <TableRow>
                      <TableHead>ID</TableHead>
                      <TableHead>Estado</TableHead>
                      <TableHead>Total</TableHead>
                      <TableHead>Fecha</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {loading ? (
                      <TableRow>
                        <TableCell colSpan={4}>Cargando...</TableCell>
                      </TableRow>
                    ) : cotizaciones.length === 0 ? (
                      <TableRow>
                        <TableCell colSpan={4} className="text-center">
                          No hay cotizaciones.
                        </TableCell>
                      </TableRow>
                    ) : (
                      cotizaciones.map((cotizacion) => (
                        <TableRow key={cotizacion.cotizacion_id}>
                          <TableCell>{cotizacion.cotizacion_id}</TableCell>
                          <TableCell>{capitalize(cotizacion.estado)}</TableCell>
                          <TableCell>
                            {formatCurrency(cotizacion.total)}
                          </TableCell>
                          <TableCell>
                            {cotizacion.fecha
                              ? formatChileanDate(cotizacion.fecha)
                              : "N/A"}
                          </TableCell>
                        </TableRow>
                      ))
                    )}
                  </TableBody>
                </Table>
              </TabsContent>

              {/* Tab de Informes */}
              <TabsContent value="informes">
                <Table>
                  <TableCaption>Informes técnicos del cliente</TableCaption>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Código Informe</TableHead>
                      <TableHead>Orden Asociada</TableHead>
                      <TableHead>Técnico</TableHead>
                      <TableHead>Fecha</TableHead>
                      <TableHead>Acciones</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {loading ? (
                      <TableRow>
                        <TableCell colSpan={5}>Cargando...</TableCell>
                      </TableRow>
                    ) : informes.length === 0 ? (
                      <TableRow>
                        <TableCell colSpan={5} className="text-center">
                          No hay informes técnicos.
                        </TableCell>
                      </TableRow>
                    ) : (
                      informes.map((informe) => (
                        <TableRow key={informe.id}>
                          <TableCell>{informe.informe_codigo}</TableCell>
                          <TableCell>{informe.orden_codigo || "N/A"}</TableCell>
                          <TableCell>{informe.tecnico_responsable}</TableCell>
                          <TableCell>
                            {formatChileanDate(informe.created_at)}
                          </TableCell>
                          <TableCell>
                            <Button
                              variant="outline"
                              size="sm"
                              onClick={() => handleVerInforme(informe)}
                            >
                              Ver Detalle
                            </Button>
                          </TableCell>
                        </TableRow>
                      ))
                    )}
                  </TableBody>
                </Table>
              </TabsContent>
            </Tabs>

            <DialogFooter>
              <Button onClick={() => onOpenChange(false)} variant="outline">
                Cerrar
              </Button>
            </DialogFooter>
          </div>
        </DialogContent>
      </Dialog>

      {/* Dialog para ver detalle del informe */}
      <Dialog open={openInformeDialog} onOpenChange={setOpenInformeDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Detalle del Informe</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            {selectedInforme ? (
              <>
                <div>
                  <strong>Código:</strong> {selectedInforme.informe_codigo}
                </div>
                <div>
                  <strong>Orden Asociada:</strong>{" "}
                  {selectedInforme.orden_codigo || "N/A"}
                </div>
                <div>
                  <strong>Técnico Responsable:</strong>{" "}
                  {selectedInforme.tecnico_responsable}
                </div>
                <div>
                  <strong>Diagnóstico:</strong>
                  <div className="mt-1 p-2 bg-gray-50 rounded">
                    {Array.isArray(selectedInforme.diagnostico)
                      ? selectedInforme.diagnostico.join(", ")
                      : selectedInforme.diagnostico}
                  </div>
                </div>
                <div>
                  <strong>Solución Aplicada:</strong>
                  <div className="mt-1 p-2 bg-gray-50 rounded">
                    {selectedInforme.solucion_aplicada}
                  </div>
                </div>
                <div>
                  <strong>Fecha de Creación:</strong>{" "}
                  {formatChileanDate(selectedInforme.created_at)}
                </div>
              </>
            ) : (
              <p>No hay información del informe disponible.</p>
            )}
          </div>
          <DialogFooter>
            <Button
              onClick={() => setOpenInformeDialog(false)}
              variant="outline"
            >
              Cerrar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
