"use client";

import { ViewTitle } from "@/components/layout/ViewTitle";

export function HelpView() {
  return (
    <div className="px-6 pt-6 space-y-6">
      <ViewTitle />
      <div className="border border-dashed rounded-lg p-6 text-center">
        <p className="text-muted-foreground">
          Help resources and documentation will be displayed here
        </p>
      </div>
    </div>
  );
}
