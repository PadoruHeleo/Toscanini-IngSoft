import { AppSidebar } from "@/components/app-sidebar";
import {
  DashboardView,
  LifecycleView,
  ListaDeEquipos,
  ProjectsView,
  TeamView,
  SettingsView,
  HelpView,
} from "@/components/views";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { ViewProvider, useView } from "@/contexts/ViewContext";

// Componente para renderizar la vista activa
function ViewRenderer() {
  const { currentView } = useView();
  switch (currentView) {
    case "dashboard":
      return <DashboardView />;
    case "lifecycle":
      return <LifecycleView />;
    case "analytics":
      return <ListaDeEquipos />;
    case "projects":
      return <ProjectsView />;
    case "team":
      return <TeamView />;
    case "settings":
      return <SettingsView />;
    case "gethelp":
      return <HelpView />;
    default:
      return <DashboardView />;
  }
}

export default function App() {
  return (
    <ViewProvider>
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
    </ViewProvider>
  );
}
