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
  return (
    <div className="p-4">
      <ViewTitle></ViewTitle>
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
                {equipo.ver_cotizacion === 1 && (
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
                        Marca: {equipo.marca}<br />
                        Modelo: {equipo.modelo}<br />
                        Cliente: {equipo.nombre_cliente}<br />
                        Estado: {equipo.estado}<br />
                        Prioridad: {equipo.prioridad}<br />
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
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}