import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useState } from "react";
import { ViewTitle } from "@/components/ViewTitle";
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

const piezas = [
  {
    pieza_id: 1,
    pieza_nombre: "Pantalla OLED 6.5",
    pieza_marca: "Samsung",
    pieza_desc: "Pantalla de repuesto para modelos Galaxy A52",
    pieza_precio: 150,
    created_at: "2025-05-30T10:00:00Z",
  },
  {
    pieza_id: 2,
    pieza_nombre: "Batería 4000mAh",
    pieza_marca: "Motorola",
    pieza_desc: "Batería de larga duración compatible con Motorola G9",
    pieza_precio: 50,
    created_at: "2025-05-30T11:00:00Z",
  },
  {
    pieza_id: 3,
    pieza_nombre: "Módulo de cámara trasera",
    pieza_marca: "Xiaomi",
    pieza_desc: "Cámara principal de 64MP para Redmi Note 11",
    pieza_precio: 80,
    created_at: "2025-05-30T12:00:00Z",
  },
];

const equipos = [
  {
    marca: "Motorola",
    modelo: "13",
    nombre_cliente: "Juan Carlos",
    estado: "Aprobado",
    prioridad: "Alta",
    fecha_ingreso: "2025-05-30",
    ver_cotizacion: 1,
  },
  {
    marca: "Samsung",
    modelo: "A52",
    nombre_cliente: "María López",
    estado: "En revisión",
    prioridad: "Media",
    fecha_ingreso: "2025-05-28",
    ver_cotizacion: 0,
  },
  {
    marca: "Xiaomi",
    modelo: "Redmi Note 11",
    nombre_cliente: "Pedro Ruiz",
    estado: "Reparado",
    prioridad: "Baja",
    fecha_ingreso: "2025-05-20",
    ver_cotizacion: 1,
  },
  {
    marca: "Apple",
    modelo: "iPhone 13",
    nombre_cliente: "Lucía Ramírez",
    estado: "Aprobado",
    prioridad: "Alta",
    fecha_ingreso: "2025-05-29",
    ver_cotizacion: 1,
  },
  {
    marca: "Huawei",
    modelo: "P30 Lite",
    nombre_cliente: "Carlos Soto",
    estado: "En revisión",
    prioridad: "Media",
    fecha_ingreso: "2025-05-27",
    ver_cotizacion: 0,
  },
  {
    marca: "LG",
    modelo: "K50",
    nombre_cliente: "Fernanda Díaz",
    estado: "Rechazado",
    prioridad: "Baja",
    fecha_ingreso: "2025-05-26",
    ver_cotizacion: 0,
  },
  {
    marca: "Nokia",
    modelo: "5.4",
    nombre_cliente: "Esteban Herrera",
    estado: "Reparado",
    prioridad: "Media",
    fecha_ingreso: "2025-05-25",
    ver_cotizacion: 1,
  },
  {
    marca: "Sony",
    modelo: "Xperia XZ1",
    nombre_cliente: "Ana Beltrán",
    estado: "Aprobado",
    prioridad: "Alta",
    fecha_ingreso: "2025-05-24",
    ver_cotizacion: 1,
  },
  {
    marca: "ZTE",
    modelo: "Blade V2020",
    nombre_cliente: "Diego Campos",
    estado: "En revisión",
    prioridad: "Baja",
    fecha_ingreso: "2025-05-23",
    ver_cotizacion: 0,
  },
  {
    marca: "Alcatel",
    modelo: "3L",
    nombre_cliente: "Patricia Núñez",
    estado: "Rechazado",
    prioridad: "Media",
    fecha_ingreso: "2025-05-22",
    ver_cotizacion: 0,
  },
];

export function ListaDeEquipos() {
  const [formData, setFormData] = useState({
    manoObra: "",
    repuestos: "",
    otrosCargos: "",
    otrasObs: "",
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleGuardar = () => {
  const horaActual = new Date().toISOString();

  const updatedFormData = {
    ...formData,
    pieza: piezas.find((p) => p.pieza_id === piezaSeleccionada),
    horaRegistro: horaActual,
  };
  setFormData(updatedFormData);
  console.log("Datos del formulario:", updatedFormData);
};

  const isFormValid =
  formData.manoObra.trim() !== "" &&
  formData.repuestos.trim() !== "" &&
  formData.otrosCargos.trim() !== "" &&
  formData.otrasObs.trim() !== "";

  const [piezaSeleccionada, setPiezaSeleccionada] = useState<number | null>(null);

const handleSeleccionPieza = (e: React.ChangeEvent<HTMLSelectElement>) => {
  setPiezaSeleccionada(parseInt(e.target.value));
};

  return (
    <div className="p-4">
      <ViewTitle />
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[120px]">Marca</TableHead>
            <TableHead>Modelo</TableHead>
            <TableHead>Cliente</TableHead>
            <TableHead className="text-right">Estado</TableHead>
            <TableHead>Prioridad</TableHead>
            <TableHead>Fecha de Ingreso</TableHead>
            <TableHead>Acciones</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {equipos.map((equipo, index) => (
            <TableRow key={index}>
              <TableCell className="font-medium">{equipo.marca}</TableCell>
              <TableCell>{equipo.modelo}</TableCell>
              <TableCell>{equipo.nombre_cliente}</TableCell>
              <TableCell className="text-right">{equipo.estado}</TableCell>
              <TableCell>{equipo.prioridad}</TableCell>
              <TableCell>{equipo.fecha_ingreso}</TableCell>
              <TableCell>
                {equipo.ver_cotizacion === 1 ? (
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="secondary" size="sm">
                        Ver Cotización
                      </Button>
                    </DialogTrigger>
                    <DialogContent className="max-w-4xl">
                      <DialogHeader>
                        <DialogTitle>Cotización del equipo</DialogTitle>
                        <DialogDescription>
                          Marca: {equipo.marca}
                          <br />
                          Modelo: {equipo.modelo}
                          <br />
                          Cliente: {equipo.nombre_cliente}
                          <br />
                          Estado: {equipo.estado}
                          <br />
                          Prioridad: {equipo.prioridad}
                          <br />
                          Fecha de ingreso: {equipo.fecha_ingreso}
                        </DialogDescription>
                      </DialogHeader>

                      <div className="mt-4 h-[500px]">
                        <iframe
                          src="/cotizacion.pdf"
                          className="w-full h-full rounded border"
                          title="Cotización PDF"
                        />
                      </div>

                      <DialogFooter className="mt-4">
                        <DialogClose asChild>
                          <Button variant="secondary">Cerrar</Button>
                        </DialogClose>
                      </DialogFooter>
                    </DialogContent>
                  </Dialog>
                ) : (
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="outline" size="sm">
                        Crear Cotización
                      </Button>
                    </DialogTrigger>
                    <DialogContent className="sm:max-w-[425px]">
                      <DialogHeader>
                        <DialogTitle>Generar Nueva Cotización</DialogTitle>
                      </DialogHeader>
                      <div className="grid gap-4">
                        <div className="grid gap-3">
                          <Label htmlFor="pieza">Seleccionar pieza</Label>
                          <select
                            id="pieza"
                            className="border rounded px-3 py-2"
                            value={piezaSeleccionada ?? ""}
                            onChange={handleSeleccionPieza}
                          >
                            <option value="">-- Seleccionar pieza --</option>
                            {piezas.map((pieza) => (
                              <option key={pieza.pieza_id} value={pieza.pieza_id}>
                                {pieza.pieza_nombre} ({pieza.pieza_marca})
                              </option>
                            ))}
                          </select>
                        </div>
                        <div className="grid gap-3">
                          <Label htmlFor="mano-obra">Mano de obra *</Label>
                          <Input
                            id="mano-obra"
                            name="manoObra"
                            value={formData.manoObra}
                            onChange={handleInputChange}
                            required
                          />
                        </div>
                        <div className="grid gap-3">
                          <Label htmlFor="repuestos">Repuestos *</Label>
                          <Input
                            id="repuestos"
                            name="repuestos"
                            value={formData.repuestos}
                            onChange={handleInputChange}
                            required
                          />
                        </div>
                        <div className="grid gap-3">
                          <Label htmlFor="otros-cargos">Otros cargos *</Label>
                          <Input
                            id="otros-cargos"
                            name="otrosCargos"
                            value={formData.otrosCargos}
                            onChange={handleInputChange}
                            required
                          />
                        </div>
                        <div className="grid gap-3">
                          <Label htmlFor="otras-obs">Otras Observaciones *</Label>
                          <Input
                            id="otras-obs"
                            name="otrasObs"
                            value={formData.otrasObs}
                            onChange={handleInputChange}
                            required
                          />
                        </div>
                      </div>
                      <DialogFooter>
                        <DialogClose asChild>
                          <Button variant="outline">Cancelar</Button>
                        </DialogClose>
                        <Button type="button" onClick={handleGuardar} disabled={!isFormValid}>
                          Guardar Cambios
                        </Button>
                      </DialogFooter>
                    </DialogContent>
                  </Dialog>
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}