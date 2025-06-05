import { AppSidebar } from "@/components/app-sidebar";
import {
  HomeView,
  EquiposView,
  ClientesView,
  SettingsView,
  HelpView,
} from "@/components/views";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { ViewProvider, useView } from "@/contexts/ViewContext";
import { AuthProvider } from "@/contexts/AuthContext";
import { ToastProvider } from "@/contexts/ToastContext";
import { ProtectedRoute } from "@/components/ProtectedRoute";
import { SessionExpirationWarning } from "@/components/SessionExpirationWarning";
import { DatabaseConnectionBanner } from "@/components/DatabaseConnectionBanner";
import { Toaster } from "@/components/ui/toaster";

// Componente para renderizar la vista activa
function ViewRenderer() {
  const { currentView } = useView();
  switch (currentView) {
    case "dashboard":
    case "inicio":
    case "home":
      return <HomeView />;
    case "lista de equipos":
      return <EquiposView />;
    case "clientes":
      return <ClientesView />;
    case "projects":
    case "settings":
      return <SettingsView />;
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
