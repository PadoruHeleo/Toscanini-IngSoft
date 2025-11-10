import { AppSidebar } from "@/components/layout/app-sidebar";
import {
  HomeView,
  EquiposView,
  ClientesView,
  OrdenesTrabajoView,
  AjustesDeCuentaView,
  HelpView,
  PiezasView,
  UsuarioView,
  TerminosCondicionesView,
  LogsAuditoriaView,
  InventarioEquiposView,
  InventarioPiezasView,
  SalidasEquipoView,
} from "@/components/views";
import { AccessDenied } from "@/components/common/AccessDenied";
import { useViewPermissions } from "@/hooks/use-permissions";
import { usePeriodicNotification } from "@/hooks/use-periodic-notification";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { ViewProvider, useView } from "@/contexts/ViewContext";
import { AuthProvider } from "@/contexts/AuthContext";
import { ToastProvider } from "@/contexts/ToastContext";
import { ProtectedRoute } from "@/components/layout/ProtectedRoute";
import { SessionExpirationWarning } from "@/components/features/session/SessionExpirationWarning";
import { DatabaseConnectionBanner } from "@/components/features/database/DatabaseConnectionBanner";
import { Toaster } from "@/components/ui/toaster";

// Componente de las notificaciones periodicas, actualmente maneja las notificaciones a laboratorio por atraso
// El valor es el tiempo en minutos
function PeriodicNotifications() {
  usePeriodicNotification(5); // 0.2 debido a testeo
  return null;
}

// Componente para renderizar la vista activa
function ViewRenderer() {
  const { currentView } = useView();
  const { canViewUsers, canViewTermsConditions } = useViewPermissions();

  switch (currentView) {
    case "dashboard":
    case "inicio":
    case "home":
      return <HomeView />;
    case "equipos en reparación":
    case "lista de equipos": // Mantener compatibilidad hacia atrás
      return <EquiposView />;
    case "clientes":
      return <ClientesView />;
    case "órdenes de trabajo":
      return <OrdenesTrabajoView />;
    case "inventario de equipos":
      return <InventarioEquiposView />;
    case "inventario de piezas":
      return <InventarioPiezasView />;
    case "piezas": // Mantener por compatibilidad
      return <PiezasView />;
    case "usuario":
    case "usuarios": // Mantener compatibilidad hacia atrás
      if (!canViewUsers) {
        return <AccessDenied />;
      }
      return <UsuarioView />;
    case "registros de auditoría":
    case "logs de auditoría": // Mantener compatibilidad hacia atrás
      if (!canViewUsers) {
        return <AccessDenied />;
      }
      return <LogsAuditoriaView />;
    case "salidas de equipos":
      if (!canViewUsers) {
        return <AccessDenied />;
      }
      return <SalidasEquipoView />;
    case "términos y condiciones":
      if (!canViewTermsConditions) {
        return <AccessDenied />;
      }
      return <TerminosCondicionesView />;
    case "projects":
    case "ajustes de cuenta":
      return <AjustesDeCuentaView />;
    case "get help":
    case "gethelp":
      return <HelpView />;
    default:
      return <HomeView />;
  }
}

export default function App() {
  return (
    <AuthProvider>
      <ToastProvider>
        <ViewProvider>
          <ProtectedRoute>
            <SidebarProvider>
              <PeriodicNotifications />{" "}
              {/* Componente de notificaciones periodicas, insertado aqui, evaluar cambiar su posicion */}
              <AppSidebar variant="inset" />{" "}
              <SidebarInset>
                <div className="flex flex-1 flex-col">
                  <DatabaseConnectionBanner />
                  <div className="@container/main flex flex-1 flex-col gap-2">
                    <ViewRenderer />
                  </div>
                </div>
              </SidebarInset>
            </SidebarProvider>
            <SessionExpirationWarning />
            <Toaster />
          </ProtectedRoute>
        </ViewProvider>
      </ToastProvider>
    </AuthProvider>
  );
}
