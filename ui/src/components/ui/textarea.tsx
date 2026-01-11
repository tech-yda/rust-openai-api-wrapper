import * as React from "react"

import { cn } from "@/lib/utils"

function Textarea({ className, ...props }: React.ComponentProps<"textarea">) {
  return (
    <textarea
      data-slot="textarea"
      className={cn(
        "flex w-full min-h-20 resize-none",
        "bg-transparent text-sm text-foreground",
        "border border-border",
        "px-4 py-3",
        "placeholder:text-muted-foreground/50 placeholder:tracking-wider",
        "transition-all duration-300",
        "outline-none",
        "focus:border-foreground/50",
        "disabled:cursor-not-allowed disabled:opacity-40",
        className
      )}
      {...props}
    />
  )
}

export { Textarea }
