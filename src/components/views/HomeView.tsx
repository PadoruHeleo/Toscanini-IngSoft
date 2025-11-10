import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import {
  IconRadio,
  IconUsers,
  IconClipboardData,
  IconFileText,
  IconTools,
  IconAlertTriangle,
  IconTrendingUp,
  IconShield,
} from "@tabler/icons-react";
import { useAuth } from "@/contexts/AuthContext";
import { useView } from "@/contexts/ViewContext";

interface OrdenTrabajoDetallada {
  orden_id: number;
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  has_garantia?: boolean;
  equipo_id?: number;
  created_by?: number;
  cotizacion_id?: number;
  informe_id?: number;
  pre_informe?: string;
  created_at?: string;
  finished_at?: string;
  numero_serie?: string;
  equipo_marca?: string;
  equipo_modelo?: string;
  equipo_tipo?: string;
  cliente_id?: number;
  cliente_nombre?: string;
  creador_nombre?: string;
  cotizacion_codigo?: string;
  costo_total?: number;
  informe_codigo?: string;
}

interface StatsData {
  total: number;
  con_garantia: number;
  por_estado: Array<{ estado: string | null; count: number }>;
  por_prioridad: Array<{ prioridad: string | null; count: number }>;
}

interface AlertResult {
  hasOldOrders: boolean;
  type: string;
  messages: string[];
}

export function HomeView() {
  const { user } = useAuth();
  const { setCurrentView } = useView();
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState<StatsData | null>(null);
  const [totalEquipos, setTotalEquipos] = useState<number>(0);
  const [totalClientes, setTotalClientes] = useState<number>(0);
  const [totalCotizaciones, setTotalCotizaciones] = useState<number>(0);
  const [totalInformes, setTotalInformes] = useState<number>(0);
  const [equiposStats, setEquiposStats] = useState<any>(null);
  const [alertas, setAlertas] = useState<AlertResult[]>([]);
  const [ordenesRecientes, setOrdenesRecientes] = useState<
    OrdenTrabajoDetallada[]
  >([]);
  const [currentDate, setCurrentDate] = useState<string>("");

  useEffect(() => {
    const updateDate = () => {
      const now = new Date();
      setCurrentDate(
        now.toLocaleDateString("es-ES", {
          weekday: "long",
          year: "numeric",
          month: "long",
          day: "numeric",
        })
      );
    };
    updateDate();
    const interval = setInterval(updateDate, 60000); // Actualizar cada minuto
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);

      // Cargar estadísticas de órdenes
      const ordenesStats = await invoke<StatsData>("get_ordenes_trabajo_stats");
      setStats(ordenesStats);

      // Cargar totales
      const equipos = await invoke<number>("count_equipos");
      setTotalEquipos(equipos);

      const clientes = await invoke<number>("count_clientes");
      setTotalClientes(clientes);

      const cotizaciones = await invoke<number>("count_cotizaciones");
      setTotalCotizaciones(cotizaciones);

      const informes = await invoke<number>("count_informes");
      setTotalInformes(informes);

      // Cargar estadísticas de equipos
      const equiposStatsData = await invoke<any>(
        "get_estadisticas_equipos_sistema"
      );
      setEquiposStats(equiposStatsData);

      // Cargar alertas
      const ordenesDetalladas = await invoke<OrdenTrabajoDetallada[]>(
        "get_ordenes_trabajo_detalladas"
      );
      const alertasData = await checkOrdenesAllNotifications(ordenesDetalladas);
      setAlertas(alertasData);

      // Cargar órdenes recientes (últimas 5)
      const ordenesRecientesData = ordenesDetalladas
        .sort((a, b) => {
          const dateA = a.created_at ? new Date(a.created_at).getTime() : 0;
          const dateB = b.created_at ? new Date(b.created_at).getTime() : 0;
          return dateB - dateA;
        })
        .slice(0, 5);
      setOrdenesRecientes(ordenesRecientesData);
    } catch (error) {
      console.error("Error cargando datos del dashboard:", error);
    } finally {
      setLoading(false);
    }
  };

  // Funciones de alertas (copiadas de use-periodic-notification.ts)
  async function checkOrdenSinCotizacion(
    ordenesData: OrdenTrabajoDetallada[]
  ): Promise<AlertResult> {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages: Array<string> = [];

    for (const orden of ordenesData) {
      if (orden.cotizacion_id === null && orden.created_at) {
        const createdAt = new Date(orden.created_at);
        const diffTime = Math.abs(now.getTime() - createdAt.getTime());
        const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

        if (diffDays >= 2) {
          hasOldOrders = true;
          let message: string = `La orden ${orden.orden_codigo} del equipo ${
            orden.equipo_marca
          } ${
            orden.equipo_modelo
          }, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffDays} días sin cotización`;
          messages.push(message);
        }
      }
    }

    return {
      hasOldOrders: hasOldOrders,
      type: "sin cotización",
      messages: messages,
    };
  }

  async function checkOrdenCotNoEnviada(
    ordenesData: OrdenTrabajoDetallada[]
  ): Promise<AlertResult> {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages: Array<string> = [];

    for (const orden of ordenesData) {
      if (orden.estado == "recibido" && orden.created_at) {
        const createdAt = new Date(orden.created_at);
        const diffTime = Math.abs(now.getTime() - createdAt.getTime());
        const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

        if (diffDays >= 3) {
          hasOldOrders = true;
          let message: string = `La orden ${orden.orden_codigo} del equipo ${
            orden.equipo_marca
          } ${
            orden.equipo_modelo
          }, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffDays} días sin cotización enviada al cliente`;
          messages.push(message);
        }
      }
    }

    return {
      hasOldOrders: hasOldOrders,
      type: "con cotización no enviada",
      messages: messages,
    };
  }

  async function checkOrdenPrioridadNoAtendida(
    ordenesData: OrdenTrabajoDetallada[]
  ): Promise<AlertResult> {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages: Array<string> = [];

    for (const orden of ordenesData) {
      if (
        orden.estado == "recibido" &&
        orden.created_at &&
        orden.prioridad == "alta"
      ) {
        const createdAt = new Date(orden.created_at);
        const diffTime = Math.abs(now.getTime() - createdAt.getTime());
        const diffHours = Math.floor(diffTime / (1000 * 60 * 60));

        if (diffHours >= 24) {
          hasOldOrders = true;
          let message: string = `La orden de prioridad ${orden.prioridad} ${
            orden.orden_codigo
          } del equipo ${orden.equipo_marca} ${
            orden.equipo_modelo
          }, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffHours} horas sin ser atendida`;
          messages.push(message);
        }
      }
    }

    return {
      hasOldOrders: hasOldOrders,
      type: "con prioridad alta no atendida",
      messages: messages,
    };
  }

  async function checkOrdenesAllNotifications(
    ordenesData: OrdenTrabajoDetallada[]
  ): Promise<AlertResult[]> {
    const result_sin_cotizacion = await checkOrdenSinCotizacion(ordenesData);
    const result_cot_no_enviada = await checkOrdenCotNoEnviada(ordenesData);
    const result_prioridad_no_atendida = await checkOrdenPrioridadNoAtendida(
      ordenesData
    );

    return [
      result_sin_cotizacion,
      result_cot_no_enviada,
      result_prioridad_no_atendida,
    ];
  }

  const getEstadoColor = (estado: string | null) => {
    switch (estado) {
      case "recibido":
        return "bg-blue-100 text-blue-800";
      case "cotizacion_enviada":
        return "bg-yellow-100 text-yellow-800";
      case "en_reparacion":
        return "bg-orange-100 text-orange-800";
      case "espera_de_retiro":
        return "bg-green-100 text-green-800";
      case "entregado":
        return "bg-gray-100 text-gray-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const getPrioridadColor = (prioridad: string | null) => {
    switch (prioridad) {
      case "alta":
        return "bg-red-100 text-red-800";
      case "media":
        return "bg-yellow-100 text-yellow-800";
      case "baja":
        return "bg-green-100 text-green-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const totalAlertas = alertas.reduce(
    (acc, alerta) => acc + (alerta.hasOldOrders ? alerta.messages.length : 0),
    0
  );

  const ordenesPendientes =
    stats?.por_estado
      .filter(
        (e) =>
          e.estado === "recibido" ||
          e.estado === "cotizacion_enviada" ||
          e.estado === "en_reparacion"
      )
      .reduce((acc, e) => acc + e.count, 0) || 0;

  const ordenesPrioridadAlta =
    stats?.por_prioridad
      .filter((p) => p.prioridad === "alta")
      .reduce((acc, p) => acc + p.count, 0) || 0;

  const ordenesAtrasadas = alertas.reduce(
    (acc, alerta) => acc + (alerta.hasOldOrders ? alerta.messages.length : 0),
    0
  );

  if (loading) {
    return (
      <div className="p-6 flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mx-auto mb-4"></div>
          <p className="text-gray-600">Cargando dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header con información contextual */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">
            Bienvenido, {user?.usuario_nombre || "Usuario"}
          </h1>
          <p className="text-gray-600 mt-1">{currentDate}</p>
        </div>
        <div className="flex items-center space-x-4">
          {/* El banner de conexión se muestra automáticamente en App.tsx */}
        </div>
      </div>

      <Separator />

      {/* Sección superior: Distribuciones (izquierda) y Actividad Reciente (derecha) */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Columna izquierda: Distribuciones */}
        <div className="space-y-6">
          {/* Distribución por estado */}
          <Card>
            <CardHeader>
              <CardTitle>Distribución por Estado</CardTitle>
              <CardDescription>
                Órdenes de trabajo agrupadas por estado actual
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {stats?.por_estado && stats.por_estado.length > 0 ? (
                  stats.por_estado.map((item, idx) => (
                    <div
                      key={idx}
                      className="flex items-center justify-between"
                    >
                      <div className="flex items-center space-x-2">
                        <Badge className={getEstadoColor(item.estado)}>
                          {item.estado || "Sin estado"}
                        </Badge>
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className="text-sm font-medium">
                          {item.count}
                        </span>
                        <div className="w-24 bg-gray-200 rounded-full h-2">
                          <div
                            className="bg-blue-600 h-2 rounded-full"
                            style={{
                              width: `${
                                stats.total > 0
                                  ? (item.count / stats.total) * 100
                                  : 0
                              }%`,
                            }}
                          ></div>
                        </div>
                      </div>
                    </div>
                  ))
                ) : (
                  <p className="text-sm text-gray-500 text-center py-4">
                    No hay datos disponibles
                  </p>
                )}
              </div>
            </CardContent>
          </Card>

          {/* Distribución por prioridad */}
          <Card>
            <CardHeader>
              <CardTitle>Distribución por Prioridad</CardTitle>
              <CardDescription>
                Órdenes de trabajo agrupadas por nivel de prioridad
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {stats?.por_prioridad && stats.por_prioridad.length > 0 ? (
                  stats.por_prioridad.map((item, idx) => (
                    <div
                      key={idx}
                      className="flex items-center justify-between"
                    >
                      <div className="flex items-center space-x-2">
                        <Badge className={getPrioridadColor(item.prioridad)}>
                          {item.prioridad || "Sin prioridad"}
                        </Badge>
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className="text-sm font-medium">
                          {item.count}
                        </span>
                        <div className="w-24 bg-gray-200 rounded-full h-2">
                          <div
                            className="bg-orange-600 h-2 rounded-full"
                            style={{
                              width: `${
                                stats.total > 0
                                  ? (item.count / stats.total) * 100
                                  : 0
                              }%`,
                            }}
                          ></div>
                        </div>
                      </div>
                    </div>
                  ))
                ) : (
                  <p className="text-sm text-gray-500 text-center py-4">
                    No hay datos disponibles
                  </p>
                )}
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Columna derecha: Actividad Reciente */}
        <Card>
          <CardHeader>
            <CardTitle>Actividad Reciente</CardTitle>
            <CardDescription>
              Últimas 5 órdenes de trabajo creadas
            </CardDescription>
          </CardHeader>
          <CardContent>
            {ordenesRecientes.length > 0 ? (
              <div className="space-y-3">
                {ordenesRecientes.map((orden) => (
                  <div
                    key={orden.orden_id}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <div className="flex-1">
                      <div className="flex items-center space-x-2 mb-1">
                        <span className="font-semibold text-sm">
                          {orden.orden_codigo || `Orden #${orden.orden_id}`}
                        </span>
                        <Badge className={getEstadoColor(orden.estado || null)}>
                          {orden.estado || "Sin estado"}
                        </Badge>
                        {orden.prioridad && (
                          <Badge className={getPrioridadColor(orden.prioridad)}>
                            {orden.prioridad}
                          </Badge>
                        )}
                      </div>
                      <div className="text-xs text-gray-600">
                        <span>
                          {orden.equipo_marca} {orden.equipo_modelo}
                        </span>
                        {orden.cliente_nombre && (
                          <span className="ml-2">• {orden.cliente_nombre}</span>
                        )}
                        {orden.created_at && (
                          <span className="ml-2">
                            •{" "}
                            {new Date(orden.created_at).toLocaleDateString(
                              "es-ES"
                            )}
                          </span>
                        )}
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setCurrentView("órdenes de trabajo")}
                    >
                      Ver
                    </Button>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-sm text-gray-500 text-center py-4">
                No hay órdenes recientes
              </p>
            )}
            <div className="mt-4">
              <Button
                variant="outline"
                onClick={() => setCurrentView("órdenes de trabajo")}
              >
                Ver todas las órdenes
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Sección inferior: Tarjetas de métricas ocupando todo el ancho */}
      <div className="space-y-6">
        {/* Tarjetas de métricas en grid de 4 columnas */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Total Órdenes
              </CardTitle>
              <IconClipboardData className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats?.total || 0}</div>
              <p className="text-xs text-muted-foreground">
                {ordenesPendientes} pendientes
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Órdenes Críticas
              </CardTitle>
              <IconAlertTriangle className="h-4 w-4 text-red-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-red-600">
                {ordenesAtrasadas}
              </div>
              <p className="text-xs text-muted-foreground">
                Requieren atención urgente
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Prioridad Alta
              </CardTitle>
              <IconTrendingUp className="h-4 w-4 text-orange-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-orange-600">
                {ordenesPrioridadAlta}
              </div>
              <p className="text-xs text-muted-foreground">Órdenes urgentes</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Total Equipos
              </CardTitle>
              <IconRadio className="h-4 w-4 text-blue-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{totalEquipos}</div>
              <p className="text-xs text-muted-foreground">
                {equiposStats?.en_sistema || 0} en sistema
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Total Clientes
              </CardTitle>
              <IconUsers className="h-4 w-4 text-green-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{totalClientes}</div>
              <p className="text-xs text-muted-foreground">
                Clientes registrados
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Cotizaciones
              </CardTitle>
              <IconFileText className="h-4 w-4 text-purple-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{totalCotizaciones}</div>
              <p className="text-xs text-muted-foreground">
                Cotizaciones totales
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">Informes</CardTitle>
              <IconTools className="h-4 w-4 text-indigo-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{totalInformes}</div>
              <p className="text-xs text-muted-foreground">
                Informes generados
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Con Garantía
              </CardTitle>
              <IconShield className="h-4 w-4 text-teal-600" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">
                {stats?.con_garantia || 0}
              </div>
              <p className="text-xs text-muted-foreground">
                Órdenes con garantía
              </p>
            </CardContent>
          </Card>
        </div>

        {/* Alertas ocupando todo el ancho */}
        {totalAlertas > 0 && (
          <Card className="border-red-200 bg-red-50">
            <CardHeader>
              <CardTitle className="flex items-center space-x-2 text-red-800">
                <IconAlertTriangle className="h-5 w-5" />
                <span>Alertas y Acciones Urgentes</span>
              </CardTitle>
              <CardDescription className="text-red-700">
                {totalAlertas} alerta{totalAlertas !== 1 ? "s" : ""} que
                requieren atención inmediata
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {alertas.map((alerta, idx) => {
                  if (!alerta.hasOldOrders) return null;
                  return (
                    <div
                      key={idx}
                      className="bg-white p-4 rounded-lg border border-red-200"
                    >
                      <h4 className="font-semibold text-red-800 mb-2">
                        ⚠️ Órdenes {alerta.type}
                      </h4>
                      <ul className="space-y-1">
                        {alerta.messages.map((msg, msgIdx) => (
                          <li
                            key={msgIdx}
                            className="text-sm text-gray-700 flex items-start"
                          >
                            <span className="w-1.5 h-1.5 bg-red-500 rounded-full mt-1.5 mr-2 flex-shrink-0"></span>
                            {msg}
                          </li>
                        ))}
                      </ul>
                    </div>
                  );
                })}
              </div>
              <div className="mt-4">
                <Button
                  variant="outline"
                  className="border-red-300 text-red-700 hover:bg-red-100"
                  onClick={() => setCurrentView("órdenes de trabajo")}
                >
                  Ver todas las órdenes
                </Button>
              </div>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
