import React from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import {
  IconRadio,
  IconSettings,
  IconUsers,
  IconClipboardData,
  IconFileText,
  IconTools,
  IconShield,
  IconDatabase,
  IconMail,
  IconCalendar,
  IconPrinter,
  IconSearch,
} from "@tabler/icons-react";

interface FeatureCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  features: string[];
  status: "completed" | "in-progress" | "planned";
}

const FeatureCard: React.FC<FeatureCardProps> = ({
  icon,
  title,
  description,
  features,
  status,
}) => {
  const getStatusBadge = () => {
    switch (status) {
      case "completed":
        return (
          <Badge className="bg-green-100 text-green-800">Completado</Badge>
        );
      case "in-progress":
        return (
          <Badge className="bg-yellow-100 text-yellow-800">En Desarrollo</Badge>
        );
      case "planned":
        return <Badge className="bg-blue-100 text-blue-800">Planificado</Badge>;
    }
  };

  return (
    <Card className="h-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-blue-100 rounded-md">{icon}</div>
            <div>
              <CardTitle className="text-lg">{title}</CardTitle>
              <CardDescription className="text-sm">
                {description}
              </CardDescription>
            </div>
          </div>
          {getStatusBadge()}
        </div>
      </CardHeader>
      <CardContent>
        <ul className="space-y-1">
          {features.map((feature, index) => (
            <li key={index} className="flex items-center text-sm text-gray-600">
              <span className="w-1 h-1 bg-blue-400 rounded-full mr-2"></span>
              {feature}
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  );
};

export function HomeView() {
  const features: FeatureCardProps[] = [
    {
      icon: <IconRadio className="h-6 w-6 text-blue-600" />,
      title: "Gestión de Equipos",
      description:
        "Administración completa del inventario de equipos de comunicación",
      features: [
        "Registro de equipos con marca, modelo y número de serie",
        "Autocompletado inteligente para marcas y modelos existentes",
        "Asociación con clientes propietarios",
        "Control de ubicación y precios",
        "Historial completo de cada equipo",
        "Búsqueda y filtrado avanzado",
      ],
      status: "completed",
    },
    {
      icon: <IconUsers className="h-6 w-6 text-green-600" />,
      title: "Gestión de Clientes",
      description: "Base de datos centralizada de clientes y sus equipos",
      features: [
        "Registro completo de datos del cliente",
        "Historial de equipos por cliente",
        "Información de contacto y comunicación",
        "Gestión de garantías y contratos",
        "Reportes de actividad por cliente",
      ],
      status: "completed",
    },
    {
      icon: <IconTools className="h-6 w-6 text-orange-600" />,
      title: "Órdenes de Trabajo",
      description: "Control completo del flujo de trabajo de mantenimiento",
      features: [
        "Creación automática desde ingreso de equipos",
        "Estados de seguimiento (Recibido, En Proceso, Completado)",
        "Asignación de prioridades",
        "Pre-informes de estado inicial",
        "Integración con cotizaciones e informes",
        "Códigos únicos de identificación",
      ],
      status: "completed",
    },
    {
      icon: <IconFileText className="h-6 w-6 text-purple-600" />,
      title: "Cotizaciones",
      description: "Generación y gestión de presupuestos para servicios",
      features: [
        "Creación de cotizaciones detalladas",
        "Gestión de piezas y materiales necesarios",
        "Cálculo automático de totales",
        "Estados de aprobación",
        "Vinculación con órdenes de trabajo",
        "Generación de documentos PDF",
      ],
      status: "in-progress",
    },
    {
      icon: <IconClipboardData className="h-6 w-6 text-red-600" />,
      title: "Informes Técnicos",
      description: "Documentación detallada de trabajos realizados",
      features: [
        "Informes de diagnóstico y reparación",
        "Registro de piezas utilizadas",
        "Documentación de procedimientos",
        "Fotografías y evidencias",
        "Firmas digitales de conformidad",
        "Exportación a PDF",
      ],
      status: "in-progress",
    },
    {
      icon: <IconDatabase className="h-6 w-6 text-indigo-600" />,
      title: "Gestión de Inventario",
      description: "Control de piezas y materiales en stock",
      features: [
        "Registro de piezas y componentes",
        "Control de stock y disponibilidad",
        "Alertas de stock bajo",
        "Historial de movimientos",
        "Integración con cotizaciones",
        "Reportes de inventario",
      ],
      status: "planned",
    },
    {
      icon: <IconShield className="h-6 w-6 text-gray-600" />,
      title: "Seguridad y Auditoría",
      description: "Control de acceso y trazabilidad de operaciones",
      features: [
        "Sistema de usuarios y roles",
        "Autenticación segura con JWT",
        "Log de auditoría completo",
        "Control de sesiones",
        "Recuperación de contraseñas",
        "Trazabilidad de cambios",
      ],
      status: "completed",
    },
    {
      icon: <IconMail className="h-6 w-6 text-blue-500" />,
      title: "Notificaciones",
      description: "Sistema de comunicación y alertas automáticas",
      features: [
        "Notificaciones por email",
        "Alertas de vencimientos",
        "Recordatorios de seguimiento",
        "Estados de órdenes de trabajo",
        "Confirmaciones de recepción",
        "Notificaciones personalizables",
      ],
      status: "planned",
    },
    {
      icon: <IconPrinter className="h-6 w-6 text-teal-600" />,
      title: "Reportes y Documentos",
      description: "Generación automática de documentos y reportes",
      features: [
        "Reportes de productividad",
        "Estadísticas de equipos",
        "Documentos PDF personalizables",
        "Etiquetas de equipos",
        "Certificados de trabajo",
        "Reportes financieros",
      ],
      status: "planned",
    },
  ];

  const completedFeatures = features.filter(
    (f) => f.status === "completed"
  ).length;
  const totalFeatures = features.length;
  const completionPercentage = Math.round(
    (completedFeatures / totalFeatures) * 100
  );

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="text-center space-y-4">
        <div className="flex items-center justify-center space-x-3">
          <IconRadio className="h-12 w-12 text-blue-600" />
          <div>
            <h1 className="text-3xl font-bold text-gray-900">
              Sistema de Gestión Toscanini
            </h1>
            <p className="text-lg text-gray-600">
              Plataforma integral para servicios técnicos de equipos de
              comunicación
            </p>
          </div>
        </div>

        <div className="flex items-center justify-center space-x-6 text-sm text-gray-600">
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-green-400 rounded-full"></div>
            <span>{completedFeatures} módulos completados</span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-yellow-400 rounded-full"></div>
            <span>
              {features.filter((f) => f.status === "in-progress").length} en
              desarrollo
            </span>
          </div>
          <div className="flex items-center space-x-2">
            <div className="w-3 h-3 bg-blue-400 rounded-full"></div>
            <span>
              {features.filter((f) => f.status === "planned").length}{" "}
              planificados
            </span>
          </div>
        </div>

        <div className="max-w-md mx-auto">
          <div className="flex justify-between text-sm text-gray-600 mb-1">
            <span>Progreso del desarrollo</span>
            <span>{completionPercentage}%</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div
              className="bg-blue-600 h-2 rounded-full transition-all duration-300"
              style={{ width: `${completionPercentage}%` }}
            ></div>
          </div>
        </div>
      </div>

      <Separator />

      {/* Características principales */}
      <div className="space-y-4">
        <div className="text-center">
          <h2 className="text-2xl font-semibold text-gray-900">
            Características Principales
          </h2>
          <p className="text-gray-600">
            Una solución completa para la gestión de servicios técnicos
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map((feature, index) => (
            <FeatureCard key={index} {...feature} />
          ))}
        </div>
      </div>

      <Separator />

      {/* Información adicional */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <IconCalendar className="h-5 w-5 text-blue-600" />
              <span>Flujo de Trabajo</span>
            </CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-gray-600">
            <div className="space-y-3">
              <div className="flex items-center space-x-2">
                <span className="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-semibold">
                  1
                </span>
                <span>Recepción de equipos</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-semibold">
                  2
                </span>
                <span>Diagnóstico y cotización</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-semibold">
                  3
                </span>
                <span>Reparación y pruebas</span>
              </div>
              <div className="flex items-center space-x-2">
                <span className="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-semibold">
                  4
                </span>
                <span>Entrega y facturación</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <IconSearch className="h-5 w-5 text-green-600" />
              <span>Beneficios Clave</span>
            </CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-gray-600">
            <ul className="space-y-2">
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-green-400 rounded-full"></span>
                <span>Trazabilidad completa</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-green-400 rounded-full"></span>
                <span>Reducción de tiempos</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-green-400 rounded-full"></span>
                <span>Control de calidad</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-green-400 rounded-full"></span>
                <span>Gestión centralizada</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-green-400 rounded-full"></span>
                <span>Reportes automáticos</span>
              </li>
            </ul>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <IconSettings className="h-5 w-5 text-purple-600" />
              <span>Tecnología</span>
            </CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-gray-600">
            <ul className="space-y-2">
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-purple-400 rounded-full"></span>
                <span>Aplicación de escritorio (Tauri)</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-purple-400 rounded-full"></span>
                <span>Interface React + TypeScript</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-purple-400 rounded-full"></span>
                <span>Base de datos SQLite</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-purple-400 rounded-full"></span>
                <span>Backend Rust</span>
              </li>
              <li className="flex items-center space-x-2">
                <span className="w-1 h-1 bg-purple-400 rounded-full"></span>
                <span>Componentes Shadcn/ui</span>
              </li>
            </ul>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
