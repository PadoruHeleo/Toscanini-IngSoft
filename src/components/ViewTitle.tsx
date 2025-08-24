import { useView } from "@/contexts/ViewContext";
import { RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";

export function ViewTitle({ onRefresh }: { onRefresh?: () => void } = {}) {
  const { currentView } = useView();

  // Convertir la primera letra a mayúscula y separar palabras si están en camelCase
  const formatTitle = (view: string) => {
    // Primero reemplazamos camelCase con espacios
    const withSpaces = view.replace(/([a-z])([A-Z])/g, "$1 $2");
    // Luego convertimos la primera letra a mayúscula
    return withSpaces.charAt(0).toUpperCase() + withSpaces.slice(1);
  };

  return (
    <div className="flex items-center gap-2 border-b pb-4 mb-4">
      <h1 className="text-2xl font-bold">{formatTitle(currentView)}</h1>
      <div className="bg-primary/10 text-primary text-xs px-2 py-1 rounded-md">
        Active Section
      </div>
      {onRefresh && (
        <Button
          size="icon"
          variant="ghost"
          onClick={onRefresh}
          title="Refrescar"
          className="ml-2"
        >
          <RefreshCw className="h-5 w-5" />
        </Button>
      )}
    </div>
  );
}
