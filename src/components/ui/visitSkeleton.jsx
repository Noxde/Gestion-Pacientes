import React from "react";
import { Skeleton } from "../shadcn/skeleton";
import { Separator } from "../shadcn/separator";

function VisitSkeleton() {
  return (
    <>
      <div className="py-5 flex flex-col gap-2">
        <div className="flex gap-5">
          <div className="flex flex-1 flex-col gap-1">
            <Skeleton className="w-[200px] h-[20px] rounded-sm" />
            <Skeleton className="w-full h-[36px] rounded-md" />
          </div>
          <div className="flex flex-1 flex-col gap-1">
            <Skeleton className="w-[200px] h-[20px] rounded-sm" />
            <Skeleton className="w-full h-[36px] rounded-md" />
          </div>
        </div>

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />
      </div>
      <Separator />
      <div className="py-5 flex flex-col gap-2">
        <div className="flex gap-5">
          <div className="flex flex-1 flex-col gap-1">
            <Skeleton className="w-[200px] h-[20px] rounded-sm" />
            <Skeleton className="w-full h-[36px] rounded-md" />
          </div>
          <div className="flex flex-1 flex-col gap-1">
            <Skeleton className="w-[200px] h-[20px] rounded-sm" />
            <Skeleton className="w-full h-[36px] rounded-md" />
          </div>
        </div>

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />

        <Skeleton className="w-[200px] h-[20px] rounded-sm" />
        <Skeleton className="w-full h-[64px] rounded-md" />
      </div>
    </>
  );
}

export default VisitSkeleton;
