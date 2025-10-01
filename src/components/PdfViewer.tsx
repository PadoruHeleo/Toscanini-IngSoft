import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Download, FileText, X, ZoomIn, ZoomOut, RotateCw } from "lucide-react";
import { useToastContext } from "@/contexts/ToastContext";

interface PdfViewerProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  documentType: "cotizacion" | "informe";
  documentId: number;
  filename?: string;
}

export function PdfViewer({
  open,
  onOpenChange,
  title,
  documentType,
  documentId,
  filename = "documento.pdf",
}: PdfViewerProps) {
  const [pdfData, setPdfData] = useState<Uint8Array | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [zoom, setZoom] = useState(100);

  const { success, error: showError } = useToastContext();

  // Limpiar URL cuando el componente se desmonta o cambia
  useEffect(() => {
    return () => {
      if (pdfUrl) {
        URL.revokeObjectURL(pdfUrl);
      }
    };
  }, [pdfUrl]);

  // Generar PDF cuando se abre el modal
  useEffect(() => {
    if (open && documentId) {
      generatePdf();
    }
  }, [open, documentId, documentType]);

  const generatePdf = async () => {
    try {
      setLoading(true);
      setError(null);

      // Limpiar PDF anterior
      if (pdfUrl) {
        URL.revokeObjectURL(pdfUrl);
        setPdfUrl(null);
      }

      let pdfBytes: number[];

      if (documentType === "cotizacion") {
        pdfBytes = await invoke<number[]>("generate_cotizacion_pdf_command", {
          cotizacionId: documentId,
        });
      } else {
        pdfBytes = await invoke<number[]>("generate_informe_pdf_command", {
          informeId: documentId,
        });
      }

      // Convertir array de números a Uint8Array
      const uint8Array = new Uint8Array(pdfBytes);
      setPdfData(uint8Array);

      // Crear URL del blob para mostrar en el visor
      const blob = new Blob([uint8Array], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      setPdfUrl(url);
    } catch (error) {
      console.error("Error generando PDF:", error);
      let errorMessage = "Error desconocido generando PDF";

      if (error instanceof Error) {
        errorMessage = error.message;
      } else if (typeof error === "string") {
        errorMessage = error;
      }

      // Mejorar mensajes de error específicos
      if (errorMessage.includes("wkhtmltopdf")) {
        errorMessage =
          "wkhtmltopdf no está instalado o no se encuentra en el PATH del sistema. Por favor, instale wkhtmltopdf para generar PDFs.";
      } else if (
        errorMessage.includes("database") ||
        errorMessage.includes("Database")
      ) {
        errorMessage =
          "Error de conexión con la base de datos. Verifique que la base de datos esté disponible.";
      } else if (
        errorMessage.includes("not found") ||
        errorMessage.includes("no encontrado")
      ) {
        errorMessage = `No se encontraron datos para el ${
          documentType === "cotizacion" ? "cotización" : "informe"
        } con ID ${documentId}.`;
      }

      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = () => {
    if (!pdfData) {
      showError("Error", "No hay datos de PDF para descargar");
      return;
    }

    try {
      const blob = new Blob([new Uint8Array(pdfData)], {
        type: "application/pdf",
      });
      const url = URL.createObjectURL(blob);

      // Crear elemento de descarga
      const link = document.createElement("a");
      link.href = url;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      // Limpiar URL
      URL.revokeObjectURL(url);

      success(
        "Descarga iniciada",
        `El archivo ${filename} se ha descargado correctamente`
      );
    } catch (error) {
      showError("Error al descargar", "No se pudo descargar el archivo PDF");
    }
  };

  const handleZoomIn = () => {
    setZoom((prev) => Math.min(prev + 25, 200));
  };

  const handleZoomOut = () => {
    setZoom((prev) => Math.max(prev - 25, 50));
  };

  const handleZoomReset = () => {
    setZoom(100);
  };

  const handleClose = () => {
    // Limpiar datos al cerrar
    if (pdfUrl) {
      URL.revokeObjectURL(pdfUrl);
      setPdfUrl(null);
    }
    setPdfData(null);
    setError(null);
    setZoom(100);
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-6xl w-[95vw] h-[90vh] flex flex-col">
        <DialogHeader>
          <DialogTitle className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <FileText className="h-5 w-5" />
              {title}
            </div>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleClose}
              className="h-8 w-8 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          </DialogTitle>
        </DialogHeader>

        {/* Barra de herramientas */}
        <div className="flex items-center justify-between border-b pb-3 mb-3">
          <div className="flex items-center gap-2">
            {/* Controles de zoom */}
            <Button
              variant="outline"
              size="sm"
              onClick={handleZoomOut}
              disabled={zoom <= 50}
              title="Alejar"
            >
              <ZoomOut className="h-4 w-4" />
            </Button>

            <span className="text-sm font-medium min-w-[60px] text-center">
              {zoom}%
            </span>

            <Button
              variant="outline"
              size="sm"
              onClick={handleZoomIn}
              disabled={zoom >= 200}
              title="Acercar"
            >
              <ZoomIn className="h-4 w-4" />
            </Button>

            <Button
              variant="outline"
              size="sm"
              onClick={handleZoomReset}
              title="Tamaño original"
            >
              <RotateCw className="h-4 w-4" />
            </Button>
          </div>

          {/* Botón de descarga */}
          <Button
            onClick={handleDownload}
            disabled={!pdfData || loading}
            className="bg-blue-600 hover:bg-blue-700"
          >
            <Download className="h-4 w-4 mr-2" />
            Descargar PDF
          </Button>
        </div>

        {/* Área de contenido */}
        <div className="flex-1 overflow-auto">
          {loading && (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"></div>
                <p className="text-gray-600">Generando PDF...</p>
              </div>
            </div>
          )}

          {error && (
            <div className="flex items-center justify-center h-full">
              <div className="text-center max-w-md">
                <div className="bg-red-50 border border-red-200 rounded-lg p-6">
                  <div className="text-red-600 mb-4">
                    <FileText className="h-12 w-12 mx-auto opacity-50" />
                  </div>
                  <h3 className="text-lg font-medium text-red-800 mb-2">
                    Error al generar PDF
                  </h3>
                  <p className="text-red-600 text-sm mb-4">{error}</p>
                  <Button onClick={generatePdf} variant="outline" size="sm">
                    Reintentar
                  </Button>
                </div>
              </div>
            </div>
          )}

          {pdfUrl && !loading && !error && (
            <div className="h-full w-full">
              <embed
                src={pdfUrl}
                type="application/pdf"
                width="100%"
                height="100%"
                style={{
                  transform: `scale(${zoom / 100})`,
                  transformOrigin: "top left",
                  minHeight: "600px",
                }}
              />
            </div>
          )}
        </div>

        {/* Mensaje de ayuda */}
        {!loading && !error && pdfData && (
          <div className="text-xs text-gray-500 text-center pt-2 border-t">
            💡 Usa los controles de zoom para ajustar el tamaño del documento.
            Haz clic en "Descargar PDF" para guardar el archivo en tu
            dispositivo.
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}

export default PdfViewer;
