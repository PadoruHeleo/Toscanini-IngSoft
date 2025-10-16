import * as React from "react";
import {
  IconChartBar,
  IconHelp,
  IconInnerShadowTop,
  IconSettings,
  IconHome,
  IconUsers,
  IconClipboardList,
  IconFileText,
  IconHistory,
} from "@tabler/icons-react";

import { NavMain } from "@/components/nav-main";
import { NavSecondary } from "@/components/nav-secondary";
import { NavUser } from "@/components/nav-user";
import { useViewPermissions } from "@/hooks/use-permissions";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const {
    canViewUsers,
    canViewTermsConditions,
    canViewEquipment,
    canViewClients,
    canViewOrders,
    canViewParts,
  } = useViewPermissions();

  // Configuración base de navegación
  const baseNavItems = [
    {
      title: "Inicio",
      url: "#",
      icon: IconHome,
      requiresPermission: false,
    },
    {
      title: "Lista de equipos",
      url: "#",
      icon: IconChartBar,
      requiresPermission: false,
      hasPermission: canViewEquipment,
    },
    {
      title: "Clientes",
      url: "#",
      icon: IconUsers,
      requiresPermission: false,
      hasPermission: canViewClients,
    },
    {
      title: "Órdenes de Trabajo",
      url: "#",
      icon: IconClipboardList,
      requiresPermission: false,
      hasPermission: canViewOrders,
    },
    {
      title: "Piezas",
      url: "#",
      icon: IconChartBar,
      requiresPermission: false,
      hasPermission: canViewParts,
    },
    {
      title: "Usuarios",
      url: "#",
      icon: IconUsers,
      requiresPermission: true,
      hasPermission: canViewUsers,
    },
    {
      title: "Logs de Auditoría",
      url: "#",
      icon: IconHistory,
      requiresPermission: true,
      hasPermission: canViewUsers,
    },
    {
      title: "Términos y Condiciones",
      url: "#",
      icon: IconFileText,
      requiresPermission: true,
      hasPermission: canViewTermsConditions,
    },
  ];

  // Filtrar elementos según permisos
  const navMain = baseNavItems.filter((item) => {
    // Si tiene hasPermission definido, usarlo para determinar visibilidad
    if (item.hasPermission !== undefined) return item.hasPermission;
    // Si no tiene hasPermission pero requiere permisos, no mostrar
    if (item.requiresPermission) return false;
    // Elementos públicos sin restricciones siempre visibles
    return true;
  });

  const data = {
    navMain,
    navSecondary: [
      {
        title: "Ajustes de Cuenta",
        url: "#",
        icon: IconSettings,
      },
      {
        title: "Get Help",
        url: "#",
        icon: IconHelp,
      },
    ],
  };
  return (
    <Sidebar collapsible="offcanvas" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:!p-1.5"
            >
              <a href="#">
                <IconInnerShadowTop className="!size-5" />
                <span className="text-base font-semibold">Toscanini.</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={data.navMain} />
        <NavSecondary items={data.navSecondary} className="mt-auto" />
      </SidebarContent>{" "}
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
    </Sidebar>
  );
}
