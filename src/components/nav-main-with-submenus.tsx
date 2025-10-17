import { type Icon } from "@tabler/icons-react";
import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";

import { useView } from "@/contexts/ViewContext";
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from "@/components/ui/sidebar";

export interface NavItemWithSub {
  title: string;
  url: string;
  icon?: Icon;
  items?: NavItemWithSub[];
}

export function NavMainWithSubmenus({ items }: { items: NavItemWithSub[] }) {
  const { currentView, setCurrentView } = useView();
  const [openItems, setOpenItems] = useState<string[]>([]);

  const toggleItem = (title: string) => {
    setOpenItems((prev) =>
      prev.includes(title)
        ? prev.filter((item) => item !== title)
        : [...prev, title]
    );
  };

  const renderItem = (item: NavItemWithSub) => {
    const hasSubItems = item.items && item.items.length > 0;
    const isOpen = openItems.includes(item.title);
    const viewId = item.title.toLowerCase();
    const isActive = currentView === viewId;

    if (hasSubItems) {
      return (
        <SidebarMenuItem key={item.title}>
          <SidebarMenuButton
            tooltip={item.title}
            onClick={() => toggleItem(item.title)}
            className="w-full justify-between"
          >
            <div className="flex items-center">
              {item.icon && <item.icon />}
              <span>{item.title}</span>
            </div>
            {isOpen ? (
              <ChevronDown className="h-4 w-4" />
            ) : (
              <ChevronRight className="h-4 w-4" />
            )}
          </SidebarMenuButton>
          {isOpen && (
            <SidebarMenuSub>
              {item.items?.map((subItem) => {
                const subViewId = subItem.title.toLowerCase();
                const isSubActive = currentView === subViewId;

                return (
                  <SidebarMenuSubItem key={subItem.title}>
                    <SidebarMenuSubButton
                      asChild
                      isActive={isSubActive}
                      onClick={() => setCurrentView(subViewId)}
                    >
                      <span className="cursor-pointer">
                        {subItem.icon && <subItem.icon />}
                        <span>{subItem.title}</span>
                      </span>
                    </SidebarMenuSubButton>
                  </SidebarMenuSubItem>
                );
              })}
            </SidebarMenuSub>
          )}
        </SidebarMenuItem>
      );
    }

    return (
      <SidebarMenuItem key={item.title}>
        <SidebarMenuButton
          tooltip={item.title}
          isActive={isActive}
          onClick={() => setCurrentView(viewId)}
        >
          {item.icon && <item.icon />}
          <span>{item.title}</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    );
  };

  return (
    <SidebarGroup>
      <SidebarGroupContent className="flex flex-col gap-2">
        <SidebarMenu>{items.map(renderItem)}</SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
