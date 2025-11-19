import * as React from "react";
import {
  IconChartBar,
  IconInnerShadowTop,
  IconSettings,
  IconLayoutDashboard,
  IconUsers,
  IconClipboardList,
  IconFileText,
  IconHistory,
  IconPackage,
  IconTool,
  IconUserCog,
} from "@tabler/icons-react";

import {
  NavMainWithSubmenus,
  type NavItemWithSub,
} from "@/components/navigation/nav-main-with-submenus";
import { NavSecondary } from "@/components/navigation/nav-secondary";
import { NavUser } from "@/components/navigation/nav-user";
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
  } = useViewPermissions();

  // Configuración de navegación con estructura jerárquica
  const buildNavItems = (): NavItemWithSub[] => {
    const items: NavItemWithSub[] = [
      {
        title: "Inicio",
        url: "#",
        icon: IconLayoutDashboard,
      },
    ];

    // Sección Clientes con submenús
    if (canViewClients || canViewOrders || canViewEquipment) {
      const clientesSubItems: NavItemWithSub[] = [];

      if (canViewClients) {
        clientesSubItems.push({
          title: "Clientes",
          url: "#",
          icon: IconUsers,
        });
      }

      if (canViewOrders) {
        clientesSubItems.push({
          title: "Órdenes de Trabajo",
          url: "#",
          icon: IconClipboardList,
        });
      }

      if (canViewEquipment) {
        clientesSubItems.push({
          title: "Equipos en Reparación",
          url: "#",
          icon: IconTool,
        });
      }

      if (clientesSubItems.length > 0) {
        items.push({
          title: "Clientes",
          url: "#",
          icon: IconUsers,
          items: clientesSubItems,
        });
      }
    }

    // Sección Inventario con submenús
    const inventarioSubItems: NavItemWithSub[] = [
      {
        title: "Inventario de Equipos",
        url: "#",
        icon: IconChartBar,
      },
      {
        title: "Inventario de Piezas",
        url: "#",
        icon: IconPackage,
      },
    ];

    items.push({
      title: "Inventario",
      url: "#",
      icon: IconPackage,
      items: inventarioSubItems,
    });

    // Sección Administrador con submenús
    if (canViewUsers || canViewTermsConditions) {
      const adminSubItems: NavItemWithSub[] = [];

      if (canViewUsers) {
        adminSubItems.push({
          title: "Usuario",
          url: "#",
          icon: IconUsers,
        });
        adminSubItems.push({
          title: "Registros de Auditoría",
          url: "#",
          icon: IconHistory,
        });
        adminSubItems.push({
          title: "Salidas de Equipos",
          url: "#",
          icon: IconPackage,
        });
      }

      if (canViewTermsConditions) {
        adminSubItems.push({
          title: "Términos y Condiciones",
          url: "#",
          icon: IconFileText,
        });
      }

      if (adminSubItems.length > 0) {
        items.push({
          title: "Administrador",
          url: "#",
          icon: IconUserCog,
          items: adminSubItems,
        });
      }
    }

    return items;
  };

  const navMain = buildNavItems();

  const navSecondary = [
    {
      title: "Ajustes de Cuenta",
      url: "#",
      icon: IconSettings,
    },
  ];
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
        <NavMainWithSubmenus items={navMain} />
        <NavSecondary items={navSecondary} className="mt-auto" />
      </SidebarContent>{" "}
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
    </Sidebar>
  );
}
