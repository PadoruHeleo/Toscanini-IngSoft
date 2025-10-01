import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Download, FileText, ZoomIn, ZoomOut, RotateCw } from "lucide-react";
import { useToastContext } from "@/contexts/ToastContext";

interface PdfViewerProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  cotizacionId?: number;
  informeId?: number;
  filename?: string;
}

export function PdfViewer({
  open,
  onOpenChange,
  title,
  cotizacionId,
  informeId,
  filename = "documento.pdf",
}: PdfViewerProps) {
  const [cotizacionPdfData, setCotizacionPdfData] = useState<Uint8Array | null>(
    null
  );
  const [informePdfData, setInformePdfData] = useState<Uint8Array | null>(null);
  const [cotizacionLoading, setCotizacionLoading] = useState(false);
  const [informeLoading, setInformeLoading] = useState(false);
  const [cotizacionError, setCotizacionError] = useState<string | null>(null);
  const [informeError, setInformeError] = useState<string | null>(null);
  const [cotizacionPdfUrl, setCotizacionPdfUrl] = useState<string | null>(null);
  const [informePdfUrl, setInformePdfUrl] = useState<string | null>(null);
  const [zoom, setZoom] = useState(100);
  const [activeTab, setActiveTab] = useState(
    cotizacionId ? "cotizacion" : informeId ? "informe" : "cotizacion"
  );

  const { success, error: showError } = useToastContext();

  // Limpiar URLs cuando el componente se desmonta o cambia
  useEffect(() => {
    return () => {
      if (cotizacionPdfUrl) {
        URL.revokeObjectURL(cotizacionPdfUrl);
      }
      if (informePdfUrl) {
        URL.revokeObjectURL(informePdfUrl);
      }
    };
  }, [cotizacionPdfUrl, informePdfUrl]);

  // Generar PDFs cuando se abre el modal
  useEffect(() => {
    if (open) {
      if (cotizacionId) {
        generateCotizacionPdf();
      }
      if (informeId) {
        generateInformePdf();
      }
    }
  }, [open, cotizacionId, informeId]);

  const generateCotizacionPdf = async () => {
    if (!cotizacionId) return;

    try {
      setCotizacionLoading(true);
      setCotizacionError(null);

      // Limpiar PDF anterior
      if (cotizacionPdfUrl) {
        URL.revokeObjectURL(cotizacionPdfUrl);
        setCotizacionPdfUrl(null);
      }

      const pdfBytes = await invoke<number[]>(
        "generate_cotizacion_pdf_command",
        {
          cotizacionId: cotizacionId,
        }
      );

      // Convertir array de números a Uint8Array
      const uint8Array = new Uint8Array(pdfBytes);
      setCotizacionPdfData(uint8Array);

      // Crear URL del blob para mostrar en el visor
      const blob = new Blob([uint8Array], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      setCotizacionPdfUrl(url);
    } catch (error) {
      console.error("Error generando PDF de cotización:", error);
      let errorMessage = "Error desconocido generando PDF de cotización";

      if (error instanceof Error) {
        errorMessage = error.message;
      } else if (typeof error === "string") {
        errorMessage = error;
      }

      setCotizacionError(errorMessage);
    } finally {
      setCotizacionLoading(false);
    }
  };

  const generateInformePdf = async () => {
    if (!informeId) return;

    try {
      setInformeLoading(true);
      setInformeError(null);

      // Limpiar PDF anterior
      if (informePdfUrl) {
        URL.revokeObjectURL(informePdfUrl);
        setInformePdfUrl(null);
      }

      const pdfBytes = await invoke<number[]>("generate_informe_pdf_command", {
        informeId: informeId,
      });

      // Convertir array de números a Uint8Array
      const uint8Array = new Uint8Array(pdfBytes);
      setInformePdfData(uint8Array);

      // Crear URL del blob para mostrar en el visor
      const blob = new Blob([uint8Array], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      setInformePdfUrl(url);
    } catch (error) {
      console.error("Error generando PDF de informe:", error);
      let errorMessage = "Error desconocido generando PDF de informe";

      if (error instanceof Error) {
        errorMessage = error.message;
      } else if (typeof error === "string") {
        errorMessage = error;
      }

      setInformeError(errorMessage);
    } finally {
      setInformeLoading(false);
    }
  };

  const handleDownload = async (type: "cotizacion" | "informe") => {
    const pdfData = type === "cotizacion" ? cotizacionPdfData : informePdfData;
    let downloadFilename = "";

    if (type === "cotizacion" && cotizacionId) {
      try {
        // Obtener datos de la cotización para el nombre del archivo
        const cotizacionData = await invoke<any>("get_cotizacion_by_id", {
          cotizacionId: cotizacionId,
        });

        if (cotizacionData && cotizacionData.cotizacion_codigo) {
          downloadFilename = `${cotizacionData.cotizacion_codigo}.pdf`;
        } else {
          downloadFilename = `COT-${cotizacionId}.pdf`;
        }
      } catch (error) {
        console.error("Error obteniendo datos de cotización:", error);
        downloadFilename = `COT-${cotizacionId}.pdf`;
      }
    } else if (type === "informe" && informeId) {
      try {
        // Obtener datos del informe para el nombre del archivo
        const informeData = await invoke<any>("get_informe_by_id", {
          informeId: informeId,
        });

        if (informeData && informeData.informe_codigo) {
          downloadFilename = `${informeData.informe_codigo}.pdf`;
        } else {
          downloadFilename = `IT-${informeId}.pdf`;
        }
      } catch (error) {
        console.error("Error obteniendo datos de informe:", error);
        downloadFilename = `IT-${informeId}.pdf`;
      }
    } else {
      downloadFilename = `documento_${type}.pdf`;
    }

    if (!pdfData) {
      showError("Error", `No hay datos de PDF de ${type} para descargar`);
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
      link.download = downloadFilename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      // Limpiar URL
      URL.revokeObjectURL(url);

      success(
        "Descarga iniciada",
        `El archivo ${downloadFilename} se ha descargado correctamente`
      );
    } catch (error) {
      showError(
        "Error al descargar",
        `No se pudo descargar el archivo PDF de ${type}`
      );
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
    if (cotizacionPdfUrl) {
      URL.revokeObjectURL(cotizacionPdfUrl);
      setCotizacionPdfUrl(null);
    }
    if (informePdfUrl) {
      URL.revokeObjectURL(informePdfUrl);
      setInformePdfUrl(null);
    }
    setCotizacionPdfData(null);
    setInformePdfData(null);
    setCotizacionError(null);
    setInformeError(null);
    setZoom(100);
    onOpenChange(false);
  };

  const renderPdfSection = (
    type: "cotizacion" | "informe",
    loading: boolean,
    error: string | null,
    pdfUrl: string | null,
    pdfData: Uint8Array | null,
    onRetry: () => void
  ) => (
    <div className="flex-1 flex flex-col">
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
          onClick={() => handleDownload(type)}
          disabled={!pdfData || loading}
          className="bg-blue-600 hover:bg-blue-700"
        >
          <Download className="h-4 w-4 mr-2" />
          Descargar PDF {type === "cotizacion" ? "Cotización" : "Informe"}
        </Button>
      </div>

      {/* Área de contenido */}
      <div className="flex-1 overflow-auto">
        {loading && (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"></div>
              <p className="text-gray-600">Generando PDF de {type}...</p>
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
                  Error al generar PDF de {type}
                </h3>
                <p className="text-red-600 text-sm mb-4">{error}</p>
                <Button onClick={onRetry} variant="outline" size="sm">
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
        </div>
      )}
    </div>
  );

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="!max-w-[90vw] !w-[90vw] !h-[90vh] flex flex-col p-3">
        <DialogHeader className="pb-2">
          <DialogTitle className="flex items-center gap-2 text-lg">
            <FileText className="h-5 w-5" />
            {title}
          </DialogTitle>
        </DialogHeader>

        {/* Pestañas para cotización e informe */}
        <Tabs
          value={activeTab}
          onValueChange={setActiveTab}
          className="flex-1 flex flex-col"
        >
          {/* Mostrar pestañas siempre - cotización e informe */}
          <TabsList className="grid w-full grid-cols-2 mb-2">
            <TabsTrigger
              value="cotizacion"
              className="flex items-center gap-2 px-6 py-2"
            >
              <FileText className="h-4 w-4" />
              Cotización {!cotizacionId && "(No disponible)"}
            </TabsTrigger>
            <TabsTrigger
              value="informe"
              className="flex items-center gap-2 px-6 py-2"
            >
              <FileText className="h-4 w-4" />
              Informe {!informeId && "(No disponible)"}
            </TabsTrigger>
          </TabsList>

          {/* Pestaña de Cotización */}
          <TabsContent value="cotizacion" className="flex-1 flex flex-col mt-4">
            {cotizacionId ? (
              renderPdfSection(
                "cotizacion",
                cotizacionLoading,
                cotizacionError,
                cotizacionPdfUrl,
                cotizacionPdfData,
                generateCotizacionPdf
              )
            ) : (
              <div className="flex items-center justify-center h-full">
                <div className="text-center max-w-md">
                  <div className="bg-gray-50 border border-gray-200 rounded-lg p-6">
                    <div className="text-gray-400 mb-4">
                      <FileText className="h-12 w-12 mx-auto opacity-50" />
                    </div>
                    <h3 className="text-lg font-medium text-gray-700 mb-2">
                      Cotización no disponible
                    </h3>
                    <p className="text-gray-600 text-sm">
                      Aún no se ha creado una cotización para esta orden de
                      trabajo.
                    </p>
                  </div>
                </div>
              </div>
            )}
          </TabsContent>

          {/* Pestaña de Informe */}
          <TabsContent value="informe" className="flex-1 flex flex-col mt-4">
            {informeId ? (
              renderPdfSection(
                "informe",
                informeLoading,
                informeError,
                informePdfUrl,
                informePdfData,
                generateInformePdf
              )
            ) : (
              <div className="flex items-center justify-center h-full">
                <div className="text-center max-w-md">
                  <div className="bg-gray-50 border border-gray-200 rounded-lg p-6">
                    <div className="text-gray-400 mb-4">
                      <FileText className="h-12 w-12 mx-auto opacity-50" />
                    </div>
                    <h3 className="text-lg font-medium text-gray-700 mb-2">
                      Informe técnico no disponible
                    </h3>
                    <p className="text-gray-600 text-sm">
                      Aún no se ha creado un informe técnico para esta orden de
                      trabajo.
                    </p>
                  </div>
                </div>
              </div>
            )}
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}

export default PdfViewer;
