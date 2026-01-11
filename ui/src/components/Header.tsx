import { Link } from '@tanstack/react-router'

export default function Header() {
  return (
    <header className="relative h-16 px-6 flex items-center border-b border-border/50 bg-background overflow-hidden">
      {/* Gradient line at top */}
      <div className="absolute top-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-white/30 to-transparent" />

      {/* Diagonal pattern background */}
      <div className="absolute inset-0 diagonal-lines opacity-50" />

      <Link
        to="/"
        className="relative flex items-center gap-4 group"
      >
        {/* Logo mark - geometric shape */}
        <div className="relative">
          <div className="w-8 h-8 border-2 border-foreground rotate-45 transition-transform duration-300 group-hover:rotate-[225deg]">
            <div className="absolute inset-1 bg-foreground/10" />
          </div>
          {/* Glitch shadow */}
          <div className="absolute inset-0 w-8 h-8 border-2 border-foreground/20 rotate-45 translate-x-[2px] translate-y-[2px] opacity-0 group-hover:opacity-100 transition-opacity" />
        </div>

        {/* Brand text */}
        <div className="flex flex-col">
          <h1 className="font-display text-2xl tracking-[0.2em] gradient-text">
            VOID
          </h1>
          <span className="text-[10px] tracking-[0.3em] text-muted-foreground uppercase -mt-1">
            neural interface
          </span>
        </div>
      </Link>

      {/* Decorative elements */}
      <div className="flex-1" />

      <div className="flex items-center gap-6">
        {/* Status indicator */}
        <div className="flex items-center gap-2">
          <div className="relative">
            <div className="w-2 h-2 bg-foreground animate-pulse" />
            <div className="absolute inset-0 w-2 h-2 bg-foreground/50 animate-ping" />
          </div>
          <span className="text-xs tracking-widest text-muted-foreground uppercase">
            online
          </span>
        </div>

        {/* Decorative line */}
        <div className="hidden md:flex items-center gap-1">
          {[...Array(5)].map((_, i) => (
            <div
              key={i}
              className="w-[2px] bg-gradient-to-b from-foreground/40 to-transparent"
              style={{ height: `${8 + i * 4}px` }}
            />
          ))}
        </div>
      </div>

      {/* Bottom gradient line */}
      <div className="absolute bottom-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-white/10 to-transparent" />
    </header>
  )
}
