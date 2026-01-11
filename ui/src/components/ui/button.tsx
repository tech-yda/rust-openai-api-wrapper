import * as React from "react"
import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const buttonVariants = cva(
  [
    "relative inline-flex items-center justify-center gap-2",
    "whitespace-nowrap text-xs font-medium tracking-wider uppercase",
    "transition-all duration-300",
    "disabled:pointer-events-none disabled:opacity-40",
    "[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
    "shrink-0 [&_svg]:shrink-0",
    "outline-none focus-visible:ring-1 focus-visible:ring-foreground/50",
  ],
  {
    variants: {
      variant: {
        default: [
          "bg-foreground text-background border border-foreground",
          "hover:bg-transparent hover:text-foreground",
          "active:scale-[0.98]",
        ],
        destructive: [
          "bg-transparent text-destructive border border-destructive",
          "hover:bg-destructive hover:text-white",
          "active:scale-[0.98]",
        ],
        outline: [
          "bg-transparent text-foreground border border-border",
          "hover:border-foreground hover:bg-foreground/5",
          "active:scale-[0.98]",
        ],
        secondary: [
          "bg-secondary text-secondary-foreground border border-border",
          "hover:bg-secondary/80 hover:border-foreground/30",
          "active:scale-[0.98]",
        ],
        ghost: [
          "bg-transparent text-foreground border border-transparent",
          "hover:bg-secondary hover:border-border",
          "active:scale-[0.98]",
        ],
        link: [
          "text-foreground underline-offset-4",
          "hover:underline",
        ],
      },
      size: {
        default: "h-10 px-5 py-2",
        sm: "h-8 px-4 py-1.5 text-[10px]",
        lg: "h-12 px-8 py-3",
        icon: "size-10",
        "icon-sm": "size-8",
        "icon-lg": "size-12",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean
  }) {
  const Comp = asChild ? Slot : "button"

  return (
    <Comp
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}

export { Button, buttonVariants }
