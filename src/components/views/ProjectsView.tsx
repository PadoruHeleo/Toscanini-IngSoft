"use client";

import { ViewTitle } from "@/components/ViewTitle";

export function ProjectsView() {
  return (
    <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
      <ViewTitle />
      <div className="border border-dashed rounded-lg p-6 text-center">
        <p className="text-muted-foreground">
          Projects list will be displayed here
        </p>
      </div>
    </div>
  );
}
