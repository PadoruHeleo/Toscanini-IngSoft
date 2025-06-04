import { AppSidebar } from "@/components/app-sidebar";
import { ListaDeEquipos, SettingsView, HelpView } from "@/components/views";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { ViewProvider, useView } from "@/contexts/ViewContext";
import { AuthProvider } from "@/contexts/AuthContext";
import { ProtectedRoute } from "@/components/ProtectedRoute";

// Componente para renderizar la vista activa
function ViewRenderer() {
  const { currentView } = useView();
  switch (currentView) {
    case "lista de equipos":
      return <ListaDeEquipos />;
    case "projects":
    case "settings":
      return <SettingsView />;
    case "gethelp":
      return <HelpView />;
    default:
      return <ListaDeEquipos />;
  }
}

export default function App() {
  return (
    <AuthProvider>
      <ViewProvider>
        <ProtectedRoute>
          <SidebarProvider>
            <AppSidebar variant="inset" />
            <SidebarInset>
              <div className="flex flex-1 flex-col">
                <div className="@container/main flex flex-1 flex-col gap-2">
                  <ViewRenderer />
                </div>
              </div>
            </SidebarInset>
          </SidebarProvider>
        </ProtectedRoute>
      </ViewProvider>
    </AuthProvider>
  );
}
